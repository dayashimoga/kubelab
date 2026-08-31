'use client';

import React, { useState } from 'react';
import Link from 'next/link';
import {
  Boxes,
  Terminal,
  BookOpen,
  ArrowRight,
  ArrowLeft,
  CheckCircle2,
  HelpCircle,
  Play,
  Copy,
  Sparkles,
  Bot,
  Flame,
  Check,
  ChevronRight,
} from 'lucide-react';

export default function LessonViewPage() {
  const [copiedCode, setCopiedCode] = useState(false);
  const [selectedAnswer, setSelectedAnswer] = useState<string | null>(null);
  const [quizSubmitted, setQuizSubmitted] = useState(false);
  const [activeDiagramLayer, setActiveDiagramLayer] = useState<'all' | 'net' | 'cgroup'>('all');

  const handleCopy = (text: string) => {
    navigator.clipboard.writeText(text);
    setCopiedCode(true);
    setTimeout(() => setCopiedCode(false), 2000);
  };

  const sampleYaml = `apiVersion: v1
kind: Pod
metadata:
  name: web-server
  labels:
    app: frontend
    tier: web
spec:
  containers:
  - name: nginx
    image: nginx:1.25-alpine
    ports:
    - containerPort: 80
    resources:
      requests:
        cpu: "100m"
        memory: "128Mi"
      limits:
        cpu: "250m"
        memory: "256Mi"`;

  return (
    <div className="container-max py-8 space-y-8">
      {/* Breadcrumb & Navigation */}
      <div className="flex flex-wrap items-center justify-between gap-4 border-b border-slate-800 pb-4">
        <div className="flex items-center gap-2 text-xs font-mono text-slate-400">
          <Link href="/learn" className="hover:text-slate-200">Tracks</Link>
          <ChevronRight className="w-3.5 h-3.5" />
          <span className="text-cyan-400">Kubernetes Core</span>
          <ChevronRight className="w-3.5 h-3.5" />
          <span className="text-slate-200 font-semibold">Lesson 1: Understanding Pods</span>
        </div>

        <div className="flex items-center gap-3">
          <Link
            href="/labs/k8s-pod-basics"
            className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-lg bg-cyan-500 hover:bg-cyan-400 text-slate-950 font-bold text-xs shadow-md shadow-cyan-500/20 transition-all"
          >
            <Terminal className="w-3.5 h-3.5" />
            <span>Launch Matching Lab</span>
          </Link>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-12 gap-8">
        {/* Main Lesson Content */}
        <div className="lg:col-span-8 space-y-8">
          {/* Title Banner */}
          <div className="space-y-3">
            <div className="inline-flex items-center gap-2 px-2.5 py-0.5 rounded-md bg-indigo-500/10 border border-indigo-500/20 text-indigo-400 text-xs font-mono">
              <span>MODULE 1 • KUBERNETES PRIMITIVES</span>
            </div>
            <h1 className="text-2xl sm:text-4xl font-extrabold text-white">
              Understanding the Pod: The Atomic Primitive
            </h1>
            <p className="text-slate-300 text-sm sm:text-base leading-relaxed">
              Why Kubernetes uses Pods instead of bare containers, how shared Linux network namespaces function, and the coordination role of the pause container.
            </p>
          </div>

          {/* Interactive Architecture Visualization */}
          <div className="glass-panel p-6 space-y-4 border-indigo-500/30">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Boxes className="w-5 h-5 text-cyan-400" />
                <h3 className="font-bold text-sm text-white uppercase tracking-wider font-mono">
                  Interactive Pod Architecture Visualizer
                </h3>
              </div>
              <div className="flex gap-1.5 bg-slate-900 p-1 rounded-lg border border-slate-800 text-[11px] font-mono">
                <button
                  onClick={() => setActiveDiagramLayer('all')}
                  className={`px-2.5 py-1 rounded-md transition-colors ${
                    activeDiagramLayer === 'all' ? 'bg-indigo-600 text-white' : 'text-slate-400 hover:text-slate-200'
                  }`}
                >
                  All Layers
                </button>
                <button
                  onClick={() => setActiveDiagramLayer('net')}
                  className={`px-2.5 py-1 rounded-md transition-colors ${
                    activeDiagramLayer === 'net' ? 'bg-indigo-600 text-white' : 'text-slate-400 hover:text-slate-200'
                  }`}
                >
                  Network NS
                </button>
                <button
                  onClick={() => setActiveDiagramLayer('cgroup')}
                  className={`px-2.5 py-1 rounded-md transition-colors ${
                    activeDiagramLayer === 'cgroup' ? 'bg-indigo-600 text-white' : 'text-slate-400 hover:text-slate-200'
                  }`}
                >
                  cgroups
                </button>
              </div>
            </div>

            {/* Visual SVG Diagram Canvas */}
            <div className="w-full h-72 bg-slate-950/80 rounded-xl border border-slate-800 p-4 flex items-center justify-center relative overflow-hidden">
              <svg className="w-full h-full" viewBox="0 0 600 240" fill="none" xmlns="http://www.w3.org/2000/svg">
                {/* Pod Boundary Box */}
                <rect x="20" y="20" width="560" height="200" rx="12" fill="#0f172a" stroke="#334155" strokeWidth="2" strokeDasharray="6 6" />
                <text x="35" y="45" fill="#38bdf8" fontSize="12" fontWeight="bold" fontFamily="monospace">
                  Pod Boundary: web-server (IP: 10.244.1.45)
                </text>

                {/* Pause Container */}
                <rect x="40" y="70" width="150" height="130" rx="8" fill="#1e293b" stroke="#06b6d4" strokeWidth="1.5" />
                <text x="50" y="95" fill="#06b6d4" fontSize="11" fontWeight="bold" fontFamily="monospace">
                  pause Container
                </text>
                <text x="50" y="115" fill="#94a3b8" fontSize="10" fontFamily="sans-serif">
                  • Holds Net NS
                </text>
                <text x="50" y="135" fill="#94a3b8" fontSize="10" fontFamily="sans-serif">
                  • Binds 10.244.1.45
                </text>
                <text x="50" y="155" fill="#94a3b8" fontSize="10" fontFamily="sans-serif">
                  • IPC / UTS Shared
                </text>

                {/* Main Container */}
                <rect x="220" y="70" width="160" height="130" rx="8" fill="#1e1b4b" stroke="#6366f1" strokeWidth="1.5" />
                <text x="230" y="95" fill="#a5b4fc" fontSize="11" fontWeight="bold" fontFamily="monospace">
                  nginx Container
                </text>
                <text x="230" y="115" fill="#cbd5e1" fontSize="10" fontFamily="sans-serif">
                  • Port :80
                </text>
                <text x="230" y="135" fill="#cbd5e1" fontSize="10" fontFamily="sans-serif">
                  • Shares localhost
                </text>
                <text x="230" y="155" fill="#cbd5e1" fontSize="10" fontFamily="sans-serif">
                  • Main Workload
                </text>

                {/* Sidecar Container */}
                <rect x="410" y="70" width="150" height="130" rx="8" fill="#022c22" stroke="#10b981" strokeWidth="1.5" />
                <text x="420" y="95" fill="#34d399" fontSize="11" fontWeight="bold" fontFamily="monospace">
                  sidecar (agent)
                </text>
                <text x="420" y="115" fill="#a7f3d0" fontSize="10" fontFamily="sans-serif">
                  • Log / Metric shipper
                </text>
                <text x="420" y="135" fill="#a7f3d0" fontSize="10" fontFamily="sans-serif">
                  • Connects via 127.0.0.1
                </text>
              </svg>
            </div>
            <p className="text-xs text-slate-400">
              Notice how the application containers connect directly to each other via <code className="text-cyan-400 bg-slate-900 px-1 py-0.5 rounded font-mono">localhost</code> and share the exact same network stack.
            </p>
          </div>

          {/* Lesson Text Sections */}
          <div className="space-y-6 text-slate-300 text-sm leading-relaxed">
            <h2 className="text-xl font-bold text-white">1. The Role of the Pause Container</h2>
            <p>
              When Kubernetes initializes a Pod on a node, the container runtime (CRI-O / containerd) first spawns an infrastructure container named <strong className="text-white">pause</strong>. The pause container requests a network namespace from the CNI plugin, acquires a dedicated cluster IP, and remains sleeping.
            </p>
            <p>
              When subsequent application containers (like nginx or your Go microservice) start, they join the pause container’s existing Linux namespace. This ensures that if your application process restarts or crashes, the network interface and IP address do not disappear or get reassigned.
            </p>

            <h2 className="text-xl font-bold text-white">2. Declarative Pod Specification</h2>
            <p>
              In production, you define Pods using declarative YAML manifests. Below is a production-ready Pod manifest with resource limits:
            </p>

            {/* Code Block with Copy Button */}
            <div className="relative rounded-xl overflow-hidden bg-slate-950 border border-slate-800">
              <div className="flex items-center justify-between px-4 py-2 bg-slate-900 border-b border-slate-800 text-xs font-mono text-slate-400">
                <span>pod.yaml</span>
                <button
                  onClick={() => handleCopy(sampleYaml)}
                  className="flex items-center gap-1 hover:text-white transition-colors"
                >
                  {copiedCode ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                  <span>{copiedCode ? 'Copied' : 'Copy'}</span>
                </button>
              </div>
              <pre className="p-4 text-xs font-mono text-cyan-300 overflow-x-auto">
                <code>{sampleYaml}</code>
              </pre>
            </div>
          </div>

          {/* Mini Knowledge Check Quiz */}
          <div className="glass-panel p-6 space-y-4 border-cyan-500/30">
            <div className="flex items-center gap-2">
              <Flame className="w-5 h-5 text-amber-400" />
              <h3 className="font-bold text-sm text-white uppercase tracking-wider font-mono">
                Knowledge Check • 50 XP
              </h3>
            </div>
            <p className="text-sm text-slate-200">
              What allows two separate containers running inside the same Pod to communicate via <code className="text-cyan-400 font-mono">localhost</code>?
            </p>

            <div className="space-y-2">
              {[
                { id: 'a', text: 'They share the same Linux Network namespace created by the pause container.' },
                { id: 'b', text: 'kube-proxy creates an iptables virtual IP for inter-container routing.' },
                { id: 'c', text: 'They share the host root filesystem.' },
              ].map((opt) => (
                <button
                  key={opt.id}
                  onClick={() => {
                    if (!quizSubmitted) setSelectedAnswer(opt.id);
                  }}
                  className={`w-full text-left p-3.5 rounded-xl border text-xs sm:text-sm font-medium transition-all ${
                    selectedAnswer === opt.id
                      ? quizSubmitted
                        ? opt.id === 'a'
                          ? 'bg-emerald-500/10 border-emerald-500 text-emerald-300'
                          : 'bg-rose-500/10 border-rose-500 text-rose-300'
                        : 'bg-indigo-600/20 border-indigo-500 text-white'
                      : 'bg-slate-900/80 border-slate-800 text-slate-300 hover:bg-slate-800'
                  }`}
                >
                  <span className="font-mono font-bold mr-2">{opt.id.toUpperCase()}.</span>
                  {opt.text}
                </button>
              ))}
            </div>

            {!quizSubmitted ? (
              <button
                disabled={!selectedAnswer}
                onClick={() => setQuizSubmitted(true)}
                className="px-4 py-2 rounded-lg bg-cyan-500 hover:bg-cyan-400 disabled:opacity-50 text-slate-950 font-bold text-xs transition-colors"
              >
                Submit Answer
              </button>
            ) : (
              <div className="p-3 rounded-lg bg-emerald-500/10 border border-emerald-500/20 text-xs text-emerald-300 flex items-center justify-between">
                <span>✅ Correct! The pause container maintains the shared network namespace. (+50 XP)</span>
              </div>
            )}
          </div>
        </div>

        {/* Right Sidebar: Key Concepts & Next Action */}
        <div className="lg:col-span-4 space-y-6">
          {/* Hands-On Lab Callout */}
          <div className="glass-panel p-6 space-y-4 border-emerald-500/30 bg-emerald-950/10">
            <div className="flex items-center gap-2 text-emerald-400 text-xs font-mono font-bold">
              <Terminal className="w-4 h-4" />
              <span>HANDS-ON LAB</span>
            </div>
            <h4 className="font-bold text-white text-base">Practice in a Live Sandbox</h4>
            <p className="text-xs text-slate-300 leading-relaxed">
              Deploy your first Pod in a real Kubernetes sandbox, verify port binding, and inspect pod phase events.
            </p>
            <Link
              href="/labs/k8s-pod-basics"
              className="inline-flex items-center justify-center gap-2 w-full px-4 py-2.5 rounded-xl bg-emerald-500 hover:bg-emerald-400 text-slate-950 font-bold text-xs shadow-lg shadow-emerald-500/20 transition-all"
            >
              <Play className="w-3.5 h-3.5 fill-current" />
              <span>Start Lab Session</span>
            </Link>
          </div>

          {/* Concepts Checklist */}
          <div className="glass-panel p-6 space-y-3">
            <h4 className="text-xs font-mono uppercase tracking-wider text-slate-400">Concepts in this Lesson</h4>
            <ul className="space-y-2 text-xs text-slate-300">
              <li className="flex items-center gap-2">
                <CheckCircle2 className="w-3.5 h-3.5 text-cyan-400" />
                <span>Pod atomic lifecycle</span>
              </li>
              <li className="flex items-center gap-2">
                <CheckCircle2 className="w-3.5 h-3.5 text-cyan-400" />
                <span>Pause container network binding</span>
              </li>
              <li className="flex items-center gap-2">
                <CheckCircle2 className="w-3.5 h-3.5 text-cyan-400" />
                <span>Resource requests vs limits</span>
              </li>
              <li className="flex items-center gap-2">
                <CheckCircle2 className="w-3.5 h-3.5 text-cyan-400" />
                <span>Multi-container co-location patterns</span>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}
