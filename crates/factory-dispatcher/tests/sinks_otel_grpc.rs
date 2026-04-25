//! Integration test for the OTLP/gRPC sink (S-1.9).
//!
//! Runs a real `tonic`-based mock `LogsService` server bound on a
//! random localhost port, points the [`OtelGrpcSink`] at it, and
//! asserts the events arrive with the OTLP record mapping the spec
//! pins (body / timestamp / reserved attributes / flat attributes).
//!
//! This is end-to-end across the gRPC wire — the mock receiver and the
//! sink share the same generated proto types from `opentelemetry-proto`,
//! so a wire-incompatible change in the conversion layer will break
//! these tests in addition to the unit tests.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use sink_core::{Sink, SinkEvent};
use sink_otel_grpc::{BatchConfig, OtelGrpcConfig, OtelGrpcSink};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::sync::Mutex as AsyncMutex;
use tokio::sync::oneshot;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use opentelemetry_proto::tonic::collector::logs::v1::logs_service_server::{
    LogsService, LogsServiceServer,
};
use opentelemetry_proto::tonic::collector::logs::v1::{
    ExportLogsServiceRequest, ExportLogsServiceResponse,
};
use opentelemetry_proto::tonic::common::v1::any_value::Value as AnyValueInner;
use opentelemetry_proto::tonic::logs::v1::LogRecord;

/// Mock OTLP collector. Records every received `ResourceLogs` flattened
/// to a `LogRecord` list so tests can assert directly on what came over
/// the wire.
#[derive(Clone, Default)]
struct MockLogsService {
    received: Arc<AsyncMutex<Vec<LogRecord>>>,
}

#[tonic::async_trait]
impl LogsService for MockLogsService {
    async fn export(
        &self,
        request: Request<ExportLogsServiceRequest>,
    ) -> Result<Response<ExportLogsServiceResponse>, Status> {
        let req = request.into_inner();
        let mut guard = self.received.lock().await;
        for rl in req.resource_logs {
            for sl in rl.scope_logs {
                for lr in sl.log_records {
                    guard.push(lr);
                }
            }
        }
        Ok(Response::new(ExportLogsServiceResponse {
            partial_success: None,
        }))
    }
}

struct MockServer {
    addr: SocketAddr,
    received: Arc<AsyncMutex<Vec<LogRecord>>>,
    shutdown_tx: Option<oneshot::Sender<()>>,
    runtime: Runtime,
}

impl MockServer {
    /// Spin up a tokio runtime + server bound to `127.0.0.1:0`. Returns
    /// once the listener is bound (so tests don't race the first send).
    fn start() -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(1)
            .thread_name("mock-otlp-server")
            .build()
            .expect("build mock server runtime");

        let received: Arc<AsyncMutex<Vec<LogRecord>>> = Arc::default();
        let svc = MockLogsService {
            received: Arc::clone(&received),
        };

        let (addr_tx, addr_rx) = std::sync::mpsc::channel::<SocketAddr>();
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

        let svc_for_task = svc.clone();
        runtime.spawn(async move {
            // Bind to an ephemeral port and report it back.
            let listener = TcpListener::bind("127.0.0.1:0")
                .await
                .expect("bind ephemeral port");
            let addr = listener.local_addr().expect("local_addr");
            addr_tx.send(addr).expect("send addr");

            // Convert the listener into the tonic incoming stream.
            let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);

            Server::builder()
                .add_service(LogsServiceServer::new(svc_for_task))
                .serve_with_incoming_shutdown(incoming, async {
                    let _ = shutdown_rx.await;
                })
                .await
                .expect("mock server serve");
        });

        let addr = addr_rx
            .recv_timeout(Duration::from_secs(5))
            .expect("mock server bind timed out");

        Self {
            addr,
            received,
            shutdown_tx: Some(shutdown_tx),
            runtime,
        }
    }

    fn endpoint(&self) -> String {
        format!("http://{}", self.addr)
    }

    fn snapshot(&self) -> Vec<LogRecord> {
        self.runtime
            .block_on(async { self.received.lock().await.clone() })
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        // The runtime drops here; outstanding tasks (the server) wind
        // down via the shutdown signal.
    }
}

fn wait_until<F: Fn() -> bool>(predicate: F, timeout: Duration) -> bool {
    let deadline = std::time::Instant::now() + timeout;
    while std::time::Instant::now() < deadline {
        if predicate() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    predicate()
}

fn body_string(record: &LogRecord) -> Option<&str> {
    match record.body.as_ref()?.value.as_ref()? {
        AnyValueInner::StringValue(s) => Some(s.as_str()),
        _ => None,
    }
}

fn attr_string<'a>(record: &'a LogRecord, key: &str) -> Option<&'a str> {
    record
        .attributes
        .iter()
        .find(|kv| kv.key == key)
        .and_then(|kv| kv.value.as_ref())
        .and_then(|v| v.value.as_ref())
        .and_then(|v| match v {
            AnyValueInner::StringValue(s) => Some(s.as_str()),
            _ => None,
        })
}

fn attr_int(record: &LogRecord, key: &str) -> Option<i64> {
    record
        .attributes
        .iter()
        .find(|kv| kv.key == key)
        .and_then(|kv| kv.value.as_ref())
        .and_then(|v| v.value.as_ref())
        .and_then(|v| match v {
            AnyValueInner::IntValue(i) => Some(*i),
            _ => None,
        })
}

fn build_event(idx: u64, plugin: &str) -> SinkEvent {
    SinkEvent::new()
        .insert("type", "plugin.invoked")
        .insert("ts_epoch", serde_json::json!(1_777_003_425_000_u64 + idx))
        .insert("dispatcher_trace_id", format!("trace-{idx}"))
        .insert("session_id", "sess-1")
        .insert("plugin_name", plugin)
        .insert("plugin_version", "0.1.0")
        .insert("seq", serde_json::json!(idx as i64))
}

#[test]
fn ten_events_arrive_with_correct_attribute_mapping() {
    let server = MockServer::start();
    let cfg = OtelGrpcConfig {
        name: "local-grafana".into(),
        enabled: true,
        endpoint: server.endpoint(),
        resource_attributes: Default::default(),
        // Big interval, batch_size <= 10 so flush() is the trigger we
        // verify here.
        batch: BatchConfig {
            size: 100,
            interval_ms: 60_000,
        },
        queue_depth: 1000,
        routing_filter: None,
        tags: Default::default(),
    };
    let sink = OtelGrpcSink::new(cfg).expect("build sink");

    for i in 0..10 {
        sink.submit(build_event(i, "capture-commit-activity"));
    }
    sink.flush().expect("flush");

    let arrived = wait_until(|| server.snapshot().len() >= 10, Duration::from_secs(5));
    let snap = server.snapshot();
    assert!(
        arrived,
        "expected 10 records to arrive at mock; got {}",
        snap.len()
    );
    assert_eq!(snap.len(), 10, "every event made it through the wire");

    // Verify the first record's mapping in detail.
    let r0 = &snap[0];
    assert_eq!(body_string(r0), Some("plugin.invoked"));
    assert_eq!(r0.time_unix_nano, 1_777_003_425_000 * 1_000_000);
    assert_eq!(attr_string(r0, "dispatcher_trace_id"), Some("trace-0"));
    assert_eq!(attr_string(r0, "session_id"), Some("sess-1"));
    assert_eq!(
        attr_string(r0, "plugin_name"),
        Some("capture-commit-activity")
    );
    assert_eq!(attr_string(r0, "plugin_version"), Some("0.1.0"));
    assert_eq!(attr_int(r0, "seq"), Some(0));

    // No reserved-field leak: type / ts_epoch were lifted, not
    // duplicated as attributes.
    assert!(!r0.attributes.iter().any(|kv| kv.key == "type"));
    assert!(!r0.attributes.iter().any(|kv| kv.key == "ts_epoch"));

    sink.shutdown();
}

#[test]
fn batch_size_trigger_flushes_without_explicit_flush_call() {
    let server = MockServer::start();
    let cfg = OtelGrpcConfig {
        name: "size-trig".into(),
        enabled: true,
        endpoint: server.endpoint(),
        resource_attributes: Default::default(),
        // batch_size = 5 so the 5th event triggers a flush all on its
        // own; interval is huge so it can't be the one that fires.
        batch: BatchConfig {
            size: 5,
            interval_ms: 60_000,
        },
        queue_depth: 1000,
        routing_filter: None,
        tags: Default::default(),
    };
    let sink = OtelGrpcSink::new(cfg).expect("build sink");

    for i in 0..5 {
        sink.submit(build_event(i, "x"));
    }
    let arrived = wait_until(|| server.snapshot().len() >= 5, Duration::from_secs(5));
    let snap = server.snapshot();
    assert!(
        arrived,
        "expected 5-event batch to flush; got {}",
        snap.len()
    );
    assert_eq!(snap.len(), 5);

    sink.shutdown();
}

#[test]
fn endpoint_unreachable_records_failure_without_panicking() {
    // Reserved port 1 — guaranteed unreachable on every Unix box. The
    // sink must not panic; failures land in take_failures().
    let cfg = OtelGrpcConfig {
        name: "unreachable".into(),
        enabled: true,
        endpoint: "http://127.0.0.1:1".into(),
        resource_attributes: Default::default(),
        batch: BatchConfig {
            size: 2,
            interval_ms: 100,
        },
        queue_depth: 1000,
        routing_filter: None,
        tags: Default::default(),
    };
    let sink = OtelGrpcSink::new(cfg).expect("build sink");

    for i in 0..4 {
        sink.submit(build_event(i, "x"));
    }
    // Wait up to 8s for at least one failure. On most systems the
    // connect refusal is sub-second; we give wide headroom for slow CI.
    let got_failure = wait_until(|| !sink.take_failures().is_empty(), Duration::from_secs(8));
    if !got_failure {
        // take_failures drains, so retry once with a fresh check via
        // queue_full_count to make sure we're not papering over a real
        // bug. With queue_depth=1000 and 4 events submitted, queue is
        // empty — so seeing nothing means the worker silently dropped.
        panic!(
            "endpoint-unreachable run never recorded a SinkFailure; \
             queue_full_count = {}",
            sink.queue_full_count()
        );
    }

    // Cleanly stop the worker so the test doesn't leak threads.
    sink.shutdown();
}

#[test]
fn flush_after_unreachable_returns_ok_but_records_failure() {
    // flush()'s contract is "make best-effort progress, don't block on
    // a stuck collector". A flush against an unreachable endpoint must
    // still return — the failure is captured separately.
    let cfg = OtelGrpcConfig {
        name: "flush-unreachable".into(),
        enabled: true,
        endpoint: "http://127.0.0.1:1".into(),
        resource_attributes: Default::default(),
        batch: BatchConfig {
            size: 1000,
            interval_ms: 60_000,
        },
        queue_depth: 1000,
        routing_filter: None,
        tags: Default::default(),
    };
    let sink = OtelGrpcSink::new(cfg).expect("build sink");
    sink.submit(build_event(0, "x"));
    // flush() must return Ok once the worker has tried and failed.
    sink.flush().expect("flush returns Ok even when send fails");
    let failures = sink.take_failures();
    assert!(
        !failures.is_empty(),
        "flush should have triggered a send attempt that recorded a failure"
    );
    sink.shutdown();
}
