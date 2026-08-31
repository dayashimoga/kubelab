'use client';

import React, { useState, useCallback } from 'react';
import dynamic from 'next/dynamic';
import { Play, Copy, Check, FileCode, CheckCircle2, RotateCcw, AlertCircle } from 'lucide-react';

// Dynamically import Monaco Editor (SSR-safe via next/dynamic)
const Editor = dynamic(() => import('@monaco-editor/react').then(mod => mod.default), {
  ssr: false,
  loading: () => (
    <div className="flex-1 flex items-center justify-center bg-slate-950 text-slate-500 text-xs font-mono">
      Loading Monaco Editor…
    </div>
  ),
});

interface MonacoYamlEditorProps {
  initialYaml: string;
  onApply: (yaml: string) => Promise<void> | void;
}

export const MonacoYamlEditor: React.FC<MonacoYamlEditorProps> = ({ initialYaml, onApply }) => {
  const [yamlContent, setYamlContent] = useState(initialYaml);
  const [isApplying, setIsApplying] = useState(false);
  const [applied, setApplied] = useState(false);
  const [copied, setCopied] = useState(false);

  const handleApply = async () => {
    setIsApplying(true);
    try {
      await onApply(yamlContent);
      setApplied(true);
      setTimeout(() => setApplied(false), 3000);
    } catch {
      // Handled by parent
    } finally {
      setIsApplying(false);
    }
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(yamlContent);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleEditorChange = useCallback((value: string | undefined) => {
    if (value !== undefined) {
      setYamlContent(value);
    }
  }, []);

  return (
    <div className="flex flex-col h-full bg-slate-950 rounded-xl border border-slate-800 overflow-hidden font-mono text-xs shadow-2xl">
      {/* Editor Header */}
      <div className="flex items-center justify-between px-4 py-2.5 bg-slate-900 border-b border-slate-800 select-none">
        <div className="flex items-center gap-2 text-slate-300">
          <FileCode className="w-4 h-4 text-cyan-400" />
          <span className="font-semibold text-xs">manifest.yaml</span>
          <span className="text-[10px] text-cyan-400 bg-cyan-500/10 px-2 py-0.5 rounded border border-cyan-500/20 font-mono">
            k8s-schema-v1.30
          </span>
        </div>

        <div className="flex items-center gap-2">
          <button
            onClick={handleCopy}
            className="flex items-center gap-1 px-2.5 py-1 rounded bg-slate-800 hover:bg-slate-700 text-slate-300 text-xs transition-colors"
          >
            {copied ? <Check className="w-3 h-3 text-emerald-400" /> : <Copy className="w-3 h-3" />}
            <span>{copied ? 'Copied' : 'Copy'}</span>
          </button>

          <button
            onClick={() => setYamlContent(initialYaml)}
            className="p-1 text-slate-400 hover:text-white rounded hover:bg-slate-800 transition-colors"
            title="Reset Manifest"
          >
            <RotateCcw className="w-3.5 h-3.5" />
          </button>

          <button
            onClick={handleApply}
            disabled={isApplying}
            className="flex items-center gap-1.5 px-3 py-1 rounded-lg bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 text-white font-bold text-xs shadow-md shadow-emerald-600/20 transition-all"
          >
            {isApplying ? (
              <>
                <span className="w-3 h-3 border-2 border-white border-t-transparent rounded-full animate-spin" />
                <span>Applying...</span>
              </>
            ) : applied ? (
              <>
                <CheckCircle2 className="w-3.5 h-3.5" />
                <span>Applied to Cluster!</span>
              </>
            ) : (
              <>
                <Play className="w-3 h-3 fill-current" />
                <span>kubectl apply -f</span>
              </>
            )}
          </button>
        </div>
      </div>

      {/* Monaco Editor Viewport */}
      <div className="flex-1 relative overflow-hidden" style={{ minHeight: '300px' }}>
        <Editor
          defaultLanguage="yaml"
          value={yamlContent}
          onChange={handleEditorChange}
          theme="vs-dark"
          options={{
            fontSize: 13,
            fontFamily: '"JetBrains Mono", "Fira Code", "Cascadia Code", Menlo, monospace',
            minimap: { enabled: false },
            scrollBeyondLastLine: false,
            wordWrap: 'on',
            tabSize: 2,
            lineNumbers: 'on',
            renderLineHighlight: 'line',
            automaticLayout: true,
            padding: { top: 12, bottom: 12 },
            cursorBlinking: 'smooth',
            cursorSmoothCaretAnimation: 'on',
            smoothScrolling: true,
            bracketPairColorization: { enabled: true },
            guides: { indentation: true },
            suggest: { showWords: false },
            quickSuggestions: false,
          }}
        />
      </div>
    </div>
  );
};
