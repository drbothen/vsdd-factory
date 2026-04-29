import React, { useState, useEffect, useRef, useCallback } from 'react';
import ExcalidrawView from './ExcalidrawView.jsx';
import HistorySidebar from './HistorySidebar.jsx';

export default function App() {
  const [files, setFiles] = useState([]);
  const [activeFile, setActiveFile] = useState(window.__ACTIVE_FILE__ || null);
  const [splitFile, setSplitFile] = useState(null);
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const [drawingData, setDrawingData] = useState(null);
  const wsRef = useRef(null);

  // Connect WebSocket
  useEffect(() => {
    const url = window.__WS_URL__ || ('ws://' + window.location.host); // nosemgrep: javascript.lang.security.detect-insecure-websocket.detect-insecure-websocket -- local-only dev server; window.__WS_URL__ override provides wss:// in any non-loopback deployment.
    function connect() {
      const ws = new WebSocket(url);
      ws.onopen = () => { wsRef.current = ws; };
      ws.onmessage = (msg) => {
        const data = JSON.parse(msg.data);
        if (data.type === 'reload') {
          fetchFiles();
        } else if (data.type === 'load-drawing') {
          setDrawingData({ elements: data.elements, appState: data.appState, files: data.files });
        }
      };
      ws.onclose = () => { setTimeout(connect, 1000); };
      wsRef.current = ws;
    }
    connect();
    return () => { if (wsRef.current) wsRef.current.close(); };
  }, []);

  // Fetch file list
  const fetchFiles = useCallback(() => {
    fetch('/api/files')
      .then(r => r.json())
      .then(data => {
        setFiles(data);
        if (data.length > 0 && !activeFile) {
          setActiveFile(data[0].name);
        }
      })
      .catch(() => {});
  }, [activeFile]);

  useEffect(() => { fetchFiles(); }, [fetchFiles]);

  // Load excalidraw data when active file changes
  useEffect(() => {
    if (!activeFile || !activeFile.endsWith('.excalidraw')) {
      setDrawingData(null);
      return;
    }
    fetch('/api/drawing/' + encodeURIComponent(activeFile))
      .then(r => r.json())
      .then(data => setDrawingData(data))
      .catch(() => setDrawingData(null));
  }, [activeFile]);

  const isExcalidraw = activeFile && activeFile.endsWith('.excalidraw');
  const isManifest = activeFile && activeFile === 'screen.json';
  const sidebarWidth = sidebarOpen ? 250 : 0;

  return (
    <div style={{ display: 'flex', height: '100vh', width: '100vw' }}>
      <HistorySidebar
        files={files}
        activeFile={activeFile}
        onSelect={setActiveFile}
        isOpen={sidebarOpen}
        onToggle={() => setSidebarOpen(!sidebarOpen)}
      />
      <div style={{
        flex: 1,
        marginLeft: sidebarWidth,
        transition: 'margin-left 0.2s',
        display: 'flex',
        flexDirection: 'column',
        height: '100vh',
      }}>
        {/* Main pane */}
        <div style={{ flex: splitFile ? 1 : 'auto', height: splitFile ? '50%' : '100%', position: 'relative' }}>
          {isExcalidraw && drawingData ? (
            <ExcalidrawView
              initialData={drawingData}
              ws={wsRef.current}
              fileName={activeFile}
            />
          ) : activeFile ? (
            <iframe
              src={'/html/' + encodeURIComponent(activeFile)}
              style={{ width: '100%', height: '100%', border: 'none' }}
              title={activeFile}
            />
          ) : (
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: '100%', color: '#666', fontFamily: 'system-ui' }}>
              Waiting for the agent to push a screen...
            </div>
          )}
        </div>

        {/* Split pane */}
        {splitFile && (
          <div style={{ flex: 1, borderTop: '2px solid #ccc', position: 'relative' }}>
            {splitFile.endsWith('.excalidraw') ? (
              <ExcalidrawView
                initialData={null}
                ws={wsRef.current}
                fileName={splitFile}
              />
            ) : (
              <iframe
                src={'/html/' + encodeURIComponent(splitFile)}
                style={{ width: '100%', height: '100%', border: 'none' }}
                title={splitFile}
              />
            )}
          </div>
        )}
      </div>
    </div>
  );
}
