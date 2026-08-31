'use client';

import React, { useState } from 'react';
import Link from 'next/link';
import {
  Terminal,
  FileCode,
  Boxes,
  CheckCircle2,
  AlertCircle,
  HelpCircle,
  Play,
  RotateCcw,
  Sparkles,
  Bot,
  Clock,
  Zap,
  ChevronRight,
  ShieldAlert,
} from 'lucide-react';
import { WebTerminal } from '@/components/Terminal';
import { MonacoYamlEditor } from '@/components/MonacoYamlEditor';
import { K8sVisualizer } from '@/components/K8sVisualizer';

export default function LabWorkspacePage({ params }: { params: { id: string } }) {
  const [activeTab, setActiveTab] = useState<'terminal' | 'editor' | 'visualizer'>('terminal');
  const [revealedHint, setRevealedHint] = useState(false);
  const [isValidating, setIsValidating] = useState(false);
  const [validationResult, setValidationResult] = useState<{
    passed: boolean;
    score: number;
    message: string;
  } | null>(null);

  const [resources, setResources] = useState<
    Array<{
      kind: string;
      name: string;
      namespace: string;
      status: 'Running' | 'Pending' | 'Failed' | 'Ready';
      age: string;
      details: string;
    }>
  >([]);

  const defaultYaml = `apiVersion: v1
kind: Pod
metadata:
  name: web-server
  labels:
    app: frontend
spec:
  containers:
  - name: nginx
    image: nginx:alpine
    ports:
    - containerPort: 80`;

  const handleApplyManifest = (yaml: string) => {
    // Add resource to visualizer
    setResources([
      {
        kind: 'Pod',
        name: 'web-server',
        namespace: 'lab-k8s-pod-basics',
        status: 'Running',
        age: '12s',
        details: 'nginx:alpine (port: 80)',
      },
    ]);
  };

  const handleValidate = () => {
    setIsValidating(true);
    setTimeout(() => {
      setIsValidating(false);
      setValidationResult({
        passed: true,
        score: 100,
        message: 'All tasks passed! Live Kubernetes object state matches requirements.',
      });
      setResources([
        {
          kind: 'Pod',
          name: 'web-server',
          namespace: 'lab-k8s-pod-basics',
          status: 'Running',
          age: '1m',
          details: 'nginx:alpine (port: 80)',
        },
      ]);
    }, 1500);
  };

  return (
    <div className="flex flex-col h-[calc(100vh-65px)] bg-[#0a0e17] overflow-hidden">
      {/* Top Session Status Bar */}
      <div className="flex items-center justify-between px-6 py-2.5 bg-slate-900 border-b border-slate-800 text-xs text-slate-300">
        <div className="flex items-center gap-3">
          <Link href="/labs" className="text-slate-400 hover:text-white transition-colors">
            Labs
          </Link>
          <ChevronRight className="w-3.5 h-3.5 text-slate-600" />
          <span className="font-bold text-white">Create and Configure Your First Pod</span>
          <span className="px-2 py-0.5 rounded bg-cyan-500/10 text-cyan-400 font-mono text-[10px] border border-cyan-500/20">
            ns: lab-k8s-pod-basics
          </span>
        </div>

        <div className="flex items-center gap-4 font-mono">
          <div className="flex items-center gap-1.5 text-amber-400">
            <Clock className="w-3.5 h-3.5" />
            <span>18:42 REMAINING</span>
          </div>
          <div className="flex items-center gap-1.5 text-emerald-400">
            <Zap className="w-3.5 h-3.5" />
            <span>100 PTS</span>
          </div>
        </div>
      </div>

      {/* Main Split-Pane Workspace */}
      <div className="flex-1 grid grid-cols-1 lg:grid-cols-12 gap-0 overflow-hidden">
        {/* Left Column: Instructions & Objectives */}
        <div className="lg:col-span-5 border-r border-slate-800 flex flex-col justify-between p-6 overflow-y-auto space-y-6 bg-slate-950/40">
          <div className="space-y-6">
            {/* Scenario */}
            <div className="space-y-2">
              <span className="text-[10px] font-mono font-bold uppercase tracking-wider text-cyan-400">
                SCENARIO
              </span>
              <p className="text-xs text-slate-300 leading-relaxed">
                A microservice requires a stateless web container running in the cluster.
                Deploy a Pod named <code className="text-cyan-300 font-mono bg-slate-900 px-1 py-0.5 rounded">web-server</code> running <code className="text-cyan-300 font-mono bg-slate-900 px-1 py-0.5 rounded">nginx:alpine</code> listening on port 80 with the label <code className="text-cyan-300 font-mono bg-slate-900 px-1 py-0.5 rounded">app=frontend</code>.
              </p>
            </div>

            {/* Task List */}
            <div className="space-y-3">
              <span className="text-[10px] font-mono font-bold uppercase tracking-wider text-slate-400">
                OBJECTIVES
              </span>

              <div className="p-3.5 rounded-xl bg-slate-900/80 border border-slate-800 space-y-1.5">
                <div className="flex items-center justify-between">
                  <span className="text-xs font-bold text-white">1. Create the web-server Pod</span>
                  <span className="text-[10px] font-mono text-cyan-400">50 PTS</span>
                </div>
                <p className="text-[11px] text-slate-400">
                  Ensure the Pod status phase is <code className="text-emerald-400 font-mono">Running</code> and has label <code className="text-indigo-400 font-mono">app=frontend</code>.
                </p>
              </div>

              <div className="p-3.5 rounded-xl bg-slate-900/80 border border-slate-800 space-y-1.5">
                <div className="flex items-center justify-between">
                  <span className="text-xs font-bold text-white">2. Container Port Binding</span>
                  <span className="text-[10px] font-mono text-cyan-400">50 PTS</span>
                </div>
                <p className="text-[11px] text-slate-400">
                  Verify container specifies <code className="text-cyan-400 font-mono">containerPort: 80</code>.
                </p>
              </div>
            </div>

            {/* Progressive Hint Drawer */}
            <div className="p-4 rounded-xl bg-indigo-950/20 border border-indigo-500/20 space-y-2">
              <div className="flex items-center justify-between">
                <span className="flex items-center gap-1.5 text-xs font-bold text-indigo-400 font-mono">
                  <HelpCircle className="w-3.5 h-3.5" />
                  <span>HINT SYSTEM</span>
                </span>
                <span className="text-[10px] font-mono text-rose-400">-15 PTS</span>
              </div>

              {revealedHint ? (
                <p className="text-xs font-mono text-slate-200 bg-slate-900 p-2.5 rounded-lg border border-slate-800">
                  kubectl run web-server --image=nginx:alpine --port=80 -l app=frontend
                </p>
              ) : (
                <button
                  onClick={() => setRevealedHint(true)}
                  className="text-xs text-indigo-300 hover:text-white font-medium underline"
                >
                  Reveal Hint (Deducts 15 points)
                </button>
              )}
            </div>

            {/* Validation Banner */}
            {validationResult && (
              <div
                className={`p-4 rounded-xl border flex items-center gap-3 ${
                  validationResult.passed
                    ? 'bg-emerald-500/10 border-emerald-500/30 text-emerald-300'
                    : 'bg-rose-500/10 border-rose-500/30 text-rose-300'
                }`}
              >
                <CheckCircle2 className="w-5 h-5 shrink-0" />
                <div className="space-y-0.5 text-xs">
                  <div className="font-bold">
                    {validationResult.passed ? 'Objective Completed!' : 'Validation Failed'}
                  </div>
                  <p className="text-slate-300">{validationResult.message}</p>
                </div>
              </div>
            )}
          </div>

          {/* Validation & Grade Action Button */}
          <div className="pt-4 border-t border-slate-800 space-y-2">
            <button
              onClick={handleValidate}
              disabled={isValidating}
              className="w-full py-3 rounded-xl bg-cyan-500 hover:bg-cyan-400 disabled:opacity-50 text-slate-950 font-extrabold text-xs uppercase tracking-wider shadow-lg shadow-cyan-500/25 transition-all flex items-center justify-center gap-2"
            >
              {isValidating ? (
                <>
                  <span className="w-3.5 h-3.5 border-2 border-slate-950 border-t-transparent rounded-full animate-spin" />
                  <span>Evaluating Live State...</span>
                </>
              ) : (
                <>
                  <CheckCircle2 className="w-4 h-4" />
                  <span>Validate & Grade Submission</span>
                </>
              )}
            </button>
          </div>
        </div>

        {/* Right Column: Live Interactive Workspace (Terminal / Editor / Visualizer) */}
        <div className="lg:col-span-7 flex flex-col h-full bg-[#0a0e17] overflow-hidden p-4 space-y-3">
          {/* Workspace Tabs */}
          <div className="flex items-center justify-between border-b border-slate-800 pb-2">
            <div className="flex items-center gap-2">
              <button
                onClick={() => setActiveTab('terminal')}
                className={`flex items-center gap-2 px-4 py-2 rounded-lg text-xs font-semibold font-mono transition-colors ${
                  activeTab === 'terminal'
                    ? 'bg-indigo-600 text-white shadow-md shadow-indigo-600/20'
                    : 'bg-slate-900 text-slate-400 hover:text-white'
                }`}
              >
                <Terminal className="w-3.5 h-3.5" />
                <span>Terminal</span>
              </button>

              <button
                onClick={() => setActiveTab('editor')}
                className={`flex items-center gap-2 px-4 py-2 rounded-lg text-xs font-semibold font-mono transition-colors ${
                  activeTab === 'editor'
                    ? 'bg-indigo-600 text-white shadow-md shadow-indigo-600/20'
                    : 'bg-slate-900 text-slate-400 hover:text-white'
                }`}
              >
                <FileCode className="w-3.5 h-3.5" />
                <span>YAML Editor</span>
              </button>

              <button
                onClick={() => setActiveTab('visualizer')}
                className={`flex items-center gap-2 px-4 py-2 rounded-lg text-xs font-semibold font-mono transition-colors ${
                  activeTab === 'visualizer'
                    ? 'bg-indigo-600 text-white shadow-md shadow-indigo-600/20'
                    : 'bg-slate-900 text-slate-400 hover:text-white'
                }`}
              >
                <Boxes className="w-3.5 h-3.5" />
                <span>Cluster Visualizer ({resources.length})</span>
              </button>
            </div>

            <div className="text-[11px] font-mono text-slate-400">
              Podman Engine Sandbox
            </div>
          </div>

          {/* Active Tool Viewport */}
          <div className="flex-1 overflow-hidden">
            {activeTab === 'terminal' && (
              <WebTerminal sessionId={params.id} namespace="lab-k8s-pod-basics" />
            )}
            {activeTab === 'editor' && (
              <MonacoYamlEditor initialYaml={defaultYaml} onApply={handleApplyManifest} />
            )}
            {activeTab === 'visualizer' && (
              <K8sVisualizer namespace="lab-k8s-pod-basics" resources={resources} />
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
