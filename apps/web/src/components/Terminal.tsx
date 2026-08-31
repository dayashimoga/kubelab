'use client';

import React, { useEffect, useRef, useState, useCallback } from 'react';
import { Terminal as TerminalIcon, Trash2, Wifi, WifiOff } from 'lucide-react';

interface TerminalProps {
  sessionId: string;
  namespace: string;
}

export const WebTerminal: React.FC<TerminalProps> = ({ sessionId, namespace }) => {
  const terminalRef = useRef<HTMLDivElement>(null);
  const xtermRef = useRef<any>(null);
  const fitAddonRef = useRef<any>(null);
  const wsRef = useRef<WebSocket | null>(null);
  const [connected, setConnected] = useState(false);
  const [loaded, setLoaded] = useState(false);

  // Dynamically import xterm (Next.js SSR-safe)
  useEffect(() => {
    let disposed = false;

    const initTerminal = async () => {
      const { Terminal } = await import('@xterm/xterm');
      const { FitAddon } = await import('@xterm/addon-fit');
      const { WebLinksAddon } = await import('@xterm/addon-web-links');

      // Ensure we haven't unmounted during async import
      if (disposed || !terminalRef.current) return;

      // Import CSS
      await import('@xterm/xterm/css/xterm.css');

      const fitAddon = new FitAddon();
      const webLinksAddon = new WebLinksAddon();

      const term = new Terminal({
        cursorBlink: true,
        fontSize: 13,
        fontFamily: '"JetBrains Mono", "Fira Code", "Cascadia Code", Menlo, monospace',
        theme: {
          background: '#0a0e1a',
          foreground: '#cbd5e1',
          cursor: '#22d3ee',
          cursorAccent: '#0a0e1a',
          selectionBackground: '#22d3ee33',
          black: '#0f172a',
          red: '#f43f5e',
          green: '#34d399',
          yellow: '#fbbf24',
          blue: '#60a5fa',
          magenta: '#c084fc',
          cyan: '#22d3ee',
          white: '#e2e8f0',
          brightBlack: '#475569',
          brightRed: '#fb7185',
          brightGreen: '#6ee7b7',
          brightYellow: '#fde68a',
          brightBlue: '#93c5fd',
          brightMagenta: '#d8b4fe',
          brightCyan: '#67e8f9',
          brightWhite: '#f8fafc',
        },
        allowTransparency: true,
        scrollback: 5000,
        convertEol: true,
      });

      term.loadAddon(fitAddon);
      term.loadAddon(webLinksAddon);
      term.open(terminalRef.current);
      fitAddon.fit();

      xtermRef.current = term;
      fitAddonRef.current = fitAddon;
      setLoaded(true);

      // Write welcome banner
      term.writeln('\x1b[1;36m=====================================================\x1b[0m');
      term.writeln(`\x1b[1;32m  KUBELAB LIVE SANDBOX SHELL (Namespace: ${namespace})\x1b[0m`);
      term.writeln('\x1b[1;36m=====================================================\x1b[0m');
      term.writeln('Ready. Try: kubectl get pods, kubectl run web-server --image=nginx:alpine');
      term.writeln('');

      // Connect WebSocket
      const wsUrl = `ws://localhost:8080/v1/ws/terminal/${sessionId}`;
      try {
        const ws = new WebSocket(wsUrl);

        ws.onopen = () => {
          setConnected(true);
          term.writeln('\x1b[1;32m● WebSocket connected to API Gateway\x1b[0m');
          term.writeln('');
        };

        ws.onmessage = (event) => {
          term.write(event.data);
        };

        ws.onclose = () => {
          setConnected(false);
          term.writeln('\x1b[1;33m● WebSocket disconnected — using local sandbox shell\x1b[0m');
        };

        ws.onerror = () => {
          setConnected(false);
        };

        wsRef.current = ws;
      } catch {
        setConnected(false);
      }

      // Handle user input: stream to WebSocket or local fallback
      let localBuffer = '';

      term.onData((data: string) => {
        if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
          // Stream raw keystrokes to backend PTY
          wsRef.current.send(JSON.stringify({ type: 'data', data }));
        } else {
          // Local sandbox fallback with line editing
          if (data === '\r') {
            term.writeln('');
            handleLocalCommand(term, localBuffer.trim(), namespace);
            localBuffer = '';
            term.write('\x1b[1;32mlearner@kubelab:~$\x1b[0m ');
          } else if (data === '\x7f') {
            // Backspace
            if (localBuffer.length > 0) {
              localBuffer = localBuffer.slice(0, -1);
              term.write('\b \b');
            }
          } else if (data >= ' ') {
            localBuffer += data;
            term.write(data);
          }
        }
      });

      // Show initial prompt in local mode
      if (!wsRef.current || wsRef.current.readyState !== WebSocket.OPEN) {
        term.write('\x1b[1;32mlearner@kubelab:~$\x1b[0m ');
      }
    };

    initTerminal();

    return () => {
      disposed = true;
      if (wsRef.current) wsRef.current.close();
      if (xtermRef.current) xtermRef.current.dispose();
    };
  }, [sessionId, namespace]);

  // Resize handler
  useEffect(() => {
    const handleResize = () => {
      if (fitAddonRef.current) {
        try {
          fitAddonRef.current.fit();
        } catch {
          // Terminal may not be visible
        }
      }
    };

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, [loaded]);

  const handleClear = useCallback(() => {
    if (xtermRef.current) {
      xtermRef.current.clear();
    }
  }, []);

  return (
    <div className="flex flex-col h-full bg-slate-950 rounded-xl border border-slate-800 overflow-hidden font-mono text-xs shadow-2xl">
      {/* Terminal Top Bar */}
      <div className="flex items-center justify-between px-4 py-2.5 bg-slate-900 border-b border-slate-800 select-none">
        <div className="flex items-center gap-2">
          <div className="flex gap-1.5">
            <span className="w-3 h-3 rounded-full bg-rose-500/80 inline-block" />
            <span className="w-3 h-3 rounded-full bg-amber-500/80 inline-block" />
            <span className="w-3 h-3 rounded-full bg-emerald-500/80 inline-block" />
          </div>
          <span className="text-slate-400 font-semibold ml-2 flex items-center gap-1.5">
            <TerminalIcon className="w-3.5 h-3.5 text-cyan-400" />
            <span>bash • sandbox-shell ({namespace})</span>
          </span>
        </div>

        <div className="flex items-center gap-3">
          <div className="flex items-center gap-1.5 text-[11px]">
            {connected ? (
              <span className="text-emerald-400 flex items-center gap-1">
                <Wifi className="w-3.5 h-3.5" />
                <span>WS CONNECTED</span>
              </span>
            ) : (
              <span className="text-amber-400 flex items-center gap-1">
                <WifiOff className="w-3.5 h-3.5" />
                <span>SANDBOX SHELL</span>
              </span>
            )}
          </div>

          <button
            onClick={handleClear}
            className="p-1 text-slate-400 hover:text-white rounded hover:bg-slate-800 transition-colors"
            title="Clear Terminal"
          >
            <Trash2 className="w-3.5 h-3.5" />
          </button>
        </div>
      </div>

      {/* xterm.js Viewport */}
      <div
        ref={terminalRef}
        className="flex-1 overflow-hidden"
        style={{ minHeight: '300px' }}
      />
    </div>
  );
};

/** Local sandbox command dispatcher for offline/demo mode */
function handleLocalCommand(term: any, cmd: string, namespace: string) {
  if (!cmd) return;

  if (cmd.startsWith('kubectl get pods') || cmd.startsWith('kubectl get po')) {
    term.writeln('NAME         READY   STATUS    RESTARTS   AGE');
    term.writeln('web-server   1/1     Running   0          42s');
  } else if (cmd.startsWith('kubectl run') || cmd.startsWith('kubectl apply')) {
    term.writeln('pod/web-server created');
  } else if (cmd.startsWith('kubectl get nodes')) {
    term.writeln('NAME                 STATUS   ROLES           AGE   VERSION');
    term.writeln('kubelab-worker-01    Ready    control-plane   12d   v1.30.0');
  } else if (cmd.startsWith('kubectl get svc') || cmd.startsWith('kubectl get services')) {
    term.writeln('NAME         TYPE        CLUSTER-IP   EXTERNAL-IP   PORT(S)   AGE');
    term.writeln('kubernetes   ClusterIP   10.96.0.1    <none>        443/TCP   12d');
  } else if (cmd.startsWith('kubectl describe')) {
    term.writeln(`Name:         web-server`);
    term.writeln(`Namespace:    ${namespace}`);
    term.writeln(`Status:       Running`);
    term.writeln(`IP:           10.244.0.5`);
  } else if (cmd === 'clear') {
    term.clear();
  } else if (cmd === 'help') {
    term.writeln('Available: kubectl get pods|svc|nodes, kubectl run, kubectl apply, kubectl describe, clear');
  } else {
    term.writeln(`\x1b[33msandbox:\x1b[0m executed '${cmd}' in namespace ${namespace}`);
  }
}
