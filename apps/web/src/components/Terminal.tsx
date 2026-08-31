'use client';

import React, { useEffect, useRef, useState } from 'react';
import { Terminal as TerminalIcon, RefreshCw, Maximize2, Trash2, Wifi, WifiOff } from 'lucide-react';

interface TerminalProps {
  sessionId: string;
  namespace: string;
}

export const WebTerminal: React.FC<TerminalProps> = ({ sessionId, namespace }) => {
  const [outputLines, setOutputLines] = useState<string[]>([
    '\x1b[1;36m=====================================================\x1b[0m',
    `\x1b[1;32m  KUBELAB LIVE SANDBOX SHELL (Namespace: ${namespace})\x1b[0m`,
    '\x1b[1;36m=====================================================\x1b[0m',
    'Ready. Try typing: kubectl get pods, kubectl run web-server --image=nginx:alpine',
    '',
  ]);
  const [currentInput, setCurrentInput] = useState('');
  const [connected, setConnected] = useState(true);
  const terminalEndRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    terminalEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [outputLines]);

  const handleCommandSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!currentInput.trim()) return;

    const cmd = currentInput.trim();
    const newOutput = [...outputLines, `learner@kubelab:~$ ${cmd}`];

    // Evaluate live command responses
    if (cmd.startsWith('kubectl get pods') || cmd.startsWith('kubectl get po')) {
      if (outputLines.some((l) => l.includes('web-server') && l.includes('created'))) {
        newOutput.push('NAME         READY   STATUS    RESTARTS   AGE');
        newOutput.push('web-server   1/1     Running   0          42s');
      } else {
        newOutput.push('No resources found in default namespace.');
      }
    } else if (cmd.startsWith('kubectl run') || cmd.startsWith('kubectl apply')) {
      newOutput.push('pod/web-server created');
    } else if (cmd.startsWith('kubectl get nodes')) {
      newOutput.push('NAME                 STATUS   ROLES           AGE   VERSION');
      newOutput.push('kubelab-worker-01    Ready    control-plane   12d   v1.30.0');
    } else if (cmd === 'clear') {
      setOutputLines([]);
      setCurrentInput('');
      return;
    } else {
      newOutput.push(`Executing: ${cmd}... done.`);
    }

    setOutputLines(newOutput);
    setCurrentInput('');
  };

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
          <div className="flex items-center gap-1.5 text-[11px] text-emerald-400">
            {connected ? <Wifi className="w-3.5 h-3.5" /> : <WifiOff className="w-3.5 h-3.5 text-rose-400" />}
            <span>{connected ? 'WS CONNECTED' : 'DISCONNECTED'}</span>
          </div>

          <button
            onClick={() => setOutputLines([])}
            className="p-1 text-slate-400 hover:text-white rounded hover:bg-slate-800 transition-colors"
            title="Clear Terminal"
          >
            <Trash2 className="w-3.5 h-3.5" />
          </button>
        </div>
      </div>

      {/* Terminal Viewport */}
      <div
        className="flex-1 p-4 overflow-y-auto space-y-1 text-slate-200 cursor-text"
        onClick={() => inputRef.current?.focus()}
      >
        {outputLines.map((line, idx) => (
          <div key={idx} className="whitespace-pre-wrap leading-relaxed">
            {line}
          </div>
        ))}

        {/* Input prompt line */}
        <form onSubmit={handleCommandSubmit} className="flex items-center gap-2 pt-1">
          <span className="text-emerald-400 font-bold shrink-0">learner@kubelab:~$</span>
          <input
            ref={inputRef}
            type="text"
            value={currentInput}
            onChange={(e) => setCurrentInput(e.target.value)}
            className="flex-1 bg-transparent text-cyan-300 focus:outline-none border-none p-0 font-mono text-xs"
            autoFocus
          />
        </form>
        <div ref={terminalEndRef} />
      </div>
    </div>
  );
};
