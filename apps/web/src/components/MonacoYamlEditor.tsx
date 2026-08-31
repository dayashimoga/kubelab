'use client';

import React, { useState } from 'react';
import { Play, Copy, Check, FileCode, CheckCircle2, RotateCcw } from 'lucide-react';

interface MonacoYamlEditorProps {
  initialYaml: string;
  onApply: (yaml: string) => void;
}

export const MonacoYamlEditor: React.FC<MonacoYamlEditorProps> = ({ initialYaml, onApply }) => {
  const [yamlContent, setYamlContent] = useState(initialYaml);
  const [applied, setApplied] = useState(false);
  const [copied, setCopied] = useState(false);

  const handleApply = () => {
    onApply(yamlContent);
    setApplied(true);
    setTimeout(() => setApplied(false), 2500);
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(yamlContent);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex flex-col h-full bg-slate-950 rounded-xl border border-slate-800 overflow-hidden font-mono text-xs shadow-2xl">
      {/* Editor Header */}
      <div className="flex items-center justify-between px-4 py-2.5 bg-slate-900 border-b border-slate-800 select-none">
        <div className="flex items-center gap-2 text-slate-300">
          <FileCode className="w-4 h-4 text-cyan-400" />
          <span className="font-semibold text-xs">manifest.yaml</span>
          <span className="text-[10px] text-slate-500 font-mono">k8s-schema-validated</span>
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
            className="flex items-center gap-1.5 px-3 py-1 rounded-lg bg-emerald-600 hover:bg-emerald-500 text-white font-bold text-xs shadow-md shadow-emerald-600/20 transition-all"
          >
            {applied ? (
              <>
                <CheckCircle2 className="w-3.5 h-3.5" />
                <span>Applied!</span>
              </>
            ) : (
              <>
                <Play className="w-3 h-3 fill-current" />
                <span>kubectl apply</span>
              </>
            )}
          </button>
        </div>
      </div>

      {/* Code Textarea / Monaco Viewport */}
      <div className="flex-1 relative flex">
        {/* Line numbers column */}
        <div className="w-10 bg-slate-950 border-r border-slate-800/80 py-3 text-right pr-2 text-slate-600 select-none font-mono text-xs">
          {yamlContent.split('\n').map((_, idx) => (
            <div key={idx} className="leading-relaxed">
              {idx + 1}
            </div>
          ))}
        </div>

        {/* Text editor */}
        <textarea
          value={yamlContent}
          onChange={(e) => setYamlContent(e.target.value)}
          className="flex-1 bg-transparent text-cyan-200 p-3 font-mono text-xs leading-relaxed focus:outline-none resize-none selection:bg-cyan-500 selection:text-black"
          spellCheck={false}
        />
      </div>
    </div>
  );
};
