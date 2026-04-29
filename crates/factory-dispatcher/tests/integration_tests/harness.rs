//! Shared test harness helpers for S-4.07 integration tests.
//!
//! Provides:
//! - `OtlpMockServer`: reusable tonic-based OTLP gRPC mock (modelled after
//!   the existing `sinks_otel_grpc.rs` inline server)
//! - `wait_for`: polling helper for async assertions
//! - `build_event`: canonical event builder for test fixtures

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use opentelemetry_proto::tonic::collector::logs::v1::logs_service_server::{
    LogsService, LogsServiceServer,
};
use opentelemetry_proto::tonic::collector::logs::v1::{
    ExportLogsServiceRequest, ExportLogsServiceResponse,
};
use opentelemetry_proto::tonic::logs::v1::LogRecord;
use sink_core::SinkEvent;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::sync::Mutex as AsyncMutex;
use tokio::sync::oneshot;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

// ── OTLP gRPC mock server ────────────────────────────────────────────────────

/// Mock OTLP LogsService — records every received LogRecord for assertion.
#[derive(Clone, Default)]
pub struct MockLogsService {
    pub received: Arc<AsyncMutex<Vec<LogRecord>>>,
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

/// Self-contained mock OTLP gRPC server. Binds on an ephemeral port.
///
/// Usage:
/// ```rust
/// let server = OtlpMockServer::start();
/// // ... submit events to a sink pointing at server.endpoint() ...
/// let records = server.snapshot();
/// assert_eq!(records.len(), 10);
/// ```
pub struct OtlpMockServer {
    pub addr: SocketAddr,
    pub received: Arc<AsyncMutex<Vec<LogRecord>>>,
    shutdown_tx: Option<oneshot::Sender<()>>,
    pub runtime: Runtime,
}

impl OtlpMockServer {
    /// Start the mock server on a random localhost port. Returns once bound.
    pub fn start() -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(1)
            .thread_name("mock-otlp-harness")
            .build()
            .expect("build mock server runtime");

        let received: Arc<AsyncMutex<Vec<LogRecord>>> = Arc::default();
        let svc = MockLogsService {
            received: Arc::clone(&received),
        };

        let (addr_tx, addr_rx) = std::sync::mpsc::channel::<SocketAddr>();
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

        let svc_clone = svc.clone();
        runtime.spawn(async move {
            let listener = TcpListener::bind("127.0.0.1:0")
                .await
                .expect("bind ephemeral port");
            let addr = listener.local_addr().expect("local_addr");
            addr_tx.send(addr).expect("send addr");

            let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);
            Server::builder()
                .add_service(LogsServiceServer::new(svc_clone))
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

    /// gRPC endpoint URL to pass to `OtelGrpcConfig::endpoint`.
    pub fn endpoint(&self) -> String {
        format!("http://{}", self.addr)
    }

    /// Snapshot of all received LogRecords (blocks until lock acquired).
    pub fn snapshot(&self) -> Vec<LogRecord> {
        self.runtime
            .block_on(async { self.received.lock().await.clone() })
    }
}

impl Drop for OtlpMockServer {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
}

// ── Polling helper ────────────────────────────────────────────────────────────

/// Poll `predicate` every 25 ms until it returns `true` or `timeout` elapses.
/// Returns the final value of `predicate()`.
pub fn wait_for<F: Fn() -> bool>(predicate: F, timeout: Duration) -> bool {
    let deadline = std::time::Instant::now() + timeout;
    while std::time::Instant::now() < deadline {
        if predicate() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    predicate()
}

// ── Canonical event builder ───────────────────────────────────────────────────

/// Build a canonical `SinkEvent` fixture for integration tests.
///
/// Sets `type`, `ts_epoch`, `dispatcher_trace_id`, `session_id`, and
/// `plugin_name`. Used as golden-input data across multiple ACs.
pub fn build_event(idx: u64, event_type: &str) -> SinkEvent {
    SinkEvent::new()
        .insert("type", event_type)
        .insert("ts_epoch", serde_json::json!(1_777_003_425_000_u64 + idx))
        .insert("dispatcher_trace_id", format!("trace-{idx}"))
        .insert("session_id", "sess-integration")
        .insert("plugin_name", "test-plugin")
        .insert("schema_version", serde_json::json!(1))
}
