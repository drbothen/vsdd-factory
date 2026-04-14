import React from 'react';

const ICONS = {
  '.html': '\u{1F4C4}',
  '.excalidraw': '\u{1F4D0}',
  '.json': '\u{2699}',
};

export default function HistorySidebar({ files, activeFile, onSelect, isOpen, onToggle }) {
  return (
    <>
      <button
        onClick={onToggle}
        style={{
          position: 'fixed', left: isOpen ? 250 : 0, top: '50%',
          transform: 'translateY(-50%)', zIndex: 1000,
          background: '#fff', border: '1px solid #ccc',
          borderLeft: 'none', borderRadius: '0 6px 6px 0',
          padding: '8px 4px', cursor: 'pointer', fontSize: '12px',
          transition: 'left 0.2s',
        }}
      >
        {isOpen ? '\u25C0' : '\u25B6'}
      </button>
      {isOpen && (
        <div style={{
          position: 'fixed', left: 0, top: 0, bottom: 0, width: 250,
          background: '#fff', borderRight: '1px solid #ccc',
          overflowY: 'auto', zIndex: 999, padding: '12px 0',
          fontFamily: 'system-ui, sans-serif',
        }}>
          <div style={{ padding: '0 12px 8px', fontSize: '11px', color: '#888', textTransform: 'uppercase', letterSpacing: '0.05em' }}>
            History
          </div>
          {files.map((f) => {
            const ext = f.name.slice(f.name.lastIndexOf('.'));
            const icon = ICONS[ext] || '\u{1F4C4}';
            const isActive = f.name === activeFile;
            return (
              <div
                key={f.name}
                onClick={() => onSelect(f.name)}
                style={{
                  padding: '8px 12px', cursor: 'pointer', fontSize: '13px',
                  background: isActive ? '#e8f4fd' : 'transparent',
                  borderLeft: isActive ? '3px solid #0071e3' : '3px solid transparent',
                }}
              >
                {icon} {f.name}
              </div>
            );
          })}
        </div>
      )}
    </>
  );
}
