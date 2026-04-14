import React, { useCallback, useRef, useState, useEffect } from 'react';

let ExcalidrawComponent = null;

export default function ExcalidrawView({ initialData, ws, fileName }) {
  const [loaded, setLoaded] = useState(false);
  const excalidrawAPI = useRef(null);
  const debounceTimer = useRef(null);

  useEffect(() => {
    import('@excalidraw/excalidraw').then((mod) => {
      ExcalidrawComponent = mod.Excalidraw;
      setLoaded(true);
    });
  }, []);

  useEffect(() => {
    if (!ws) return;
    const handler = (msg) => {
      try {
        const data = JSON.parse(msg.data);
        if (data.type === 'load-drawing' && data.file === fileName && excalidrawAPI.current) {
          excalidrawAPI.current.updateScene({
            elements: data.elements || [],
            appState: data.appState || {},
            storeAction: 'capture',
          });
        }
      } catch (e) {}
    };
    ws.addEventListener('message', handler);
    return () => ws.removeEventListener('message', handler);
  }, [ws, fileName]);

  const handleChange = useCallback((elements, appState, files) => {
    if (debounceTimer.current) clearTimeout(debounceTimer.current);
    debounceTimer.current = setTimeout(() => {
      if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({
          type: 'drawing-updated',
          file: fileName,
          data: { elements, appState, files },
        }));
      }
    }, 500);
  }, [ws, fileName]);

  if (!loaded) {
    return (
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: '100%', fontFamily: 'system-ui', color: '#666' }}>
        Loading Excalidraw...
      </div>
    );
  }

  const viewMode = initialData?.appState?.viewMode || false;

  return (
    <div style={{ height: '100%', width: '100%' }}>
      <ExcalidrawComponent
        excalidrawAPI={(api) => { excalidrawAPI.current = api; }}
        initialData={initialData || { elements: [], appState: { viewBackgroundColor: '#ffffff' } }}
        onChange={handleChange}
        viewModeEnabled={viewMode}
        theme={window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'}
      />
    </div>
  );
}
