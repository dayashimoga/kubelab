'use client';

import React from 'react';
import { Boxes, Server, Network, Activity, CheckCircle2, AlertCircle } from 'lucide-react';

interface ResourceItem {
  kind: string;
  name: string;
  namespace: string;
  status: 'Running' | 'Pending' | 'Failed' | 'Ready';
  age: string;
  details: string;
}

interface K8sVisualizerProps {
  namespace: string;
  resources: ResourceItem[];
}

export const K8sVisualizer: React.FC<K8sVisualizerProps> = ({ namespace, resources }) => {
  return (
    <div className="flex flex-col h-full bg-slate-950 rounded-xl border border-slate-800 overflow-hidden font-mono text-xs shadow-2xl">
      {/* Top Header */}
      <div className="flex items-center justify-between px-4 py-2.5 bg-slate-900 border-b border-slate-800 select-none">
        <div className="flex items-center gap-2 text-slate-300">
          <Boxes className="w-4 h-4 text-cyan-400" />
          <span className="font-semibold text-xs">Kubernetes Cluster Visualizer</span>
          <span className="text-[10px] text-cyan-400 bg-cyan-500/10 px-2 py-0.5 rounded border border-cyan-500/20">
            ns: {namespace}
          </span>
        </div>
        <div className="flex items-center gap-2 text-slate-400 text-[11px]">
          <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
          <span>Live API Sync</span>
        </div>
      </div>

      {/* Visual Resource Tree */}
      <div className="flex-1 p-4 overflow-y-auto space-y-4">
        {resources.length === 0 ? (
          <div className="h-full flex flex-col items-center justify-center text-slate-500 space-y-2 py-12">
            <Boxes className="w-8 h-8 text-slate-600 animate-bounce" />
            <p className="text-xs">No active resources found in namespace `{namespace}`.</p>
            <p className="text-[11px] text-slate-600">Deploy resources via Terminal or YAML editor to view them live.</p>
          </div>
        ) : (
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
            {resources.map((res, idx) => (
              <div
                key={idx}
                className="p-3.5 rounded-xl bg-slate-900/80 border border-slate-800 hover:border-cyan-500/40 transition-all space-y-2"
              >
                <div className="flex items-center justify-between">
                  <span className="px-2 py-0.5 rounded bg-indigo-500/20 text-indigo-300 text-[10px] uppercase font-bold">
                    {res.kind}
                  </span>
                  <div className="flex items-center gap-1 text-[11px]">
                    {res.status === 'Running' || res.status === 'Ready' ? (
                      <span className="flex items-center gap-1 text-emerald-400">
                        <CheckCircle2 className="w-3.5 h-3.5" />
                        <span>{res.status}</span>
                      </span>
                    ) : (
                      <span className="flex items-center gap-1 text-amber-400">
                        <AlertCircle className="w-3.5 h-3.5" />
                        <span>{res.status}</span>
                      </span>
                    )}
                  </div>
                </div>

                <div className="text-white font-bold text-xs truncate">{res.name}</div>
                <div className="text-[10px] text-slate-400 flex justify-between">
                  <span>{res.details}</span>
                  <span className="text-slate-500">{res.age}</span>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};
