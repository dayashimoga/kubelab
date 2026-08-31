import React from 'react';
import Link from 'next/link';
import {
  Terminal,
  Boxes,
  Flame,
  Zap,
  ArrowRight,
  ShieldCheck,
  Network,
  Activity,
  AlertTriangle,
  GitBranch,
  CheckCircle2,
  Play,
  Layers,
  Sparkles,
  Server,
} from 'lucide-react';

export default function HomePage() {
  const quickStats = [
    { label: 'Total XP', value: '1,250', change: '+250 this week', icon: Zap, color: 'text-amber-400' },
    { label: 'Mastery Level', value: 'Level 3', change: '500 XP to L4', icon: Boxes, color: 'text-cyan-400' },
    { label: 'Active Streak', value: '5 Days', change: 'Personal best: 12d', icon: Flame, color: 'text-rose-400' },
    { label: 'Live Sandboxes', value: '1 Active', change: 'k8s-pod-basics', icon: Terminal, color: 'text-emerald-400' },
  ];

  const tracks = [
    { slug: 'foundations', title: 'Linux & OCI Foundations', lessons: '15 Lessons', xp: '1,500 XP', icon: Terminal, progress: 80 },
    { slug: 'kubernetes', title: 'Kubernetes Core Workloads', lessons: '20 Lessons', xp: '2,500 XP', icon: Boxes, progress: 45 },
    { slug: 'k8s-admin', title: 'Cluster Admin & etcd Ops', lessons: '12 Lessons', xp: '1,800 XP', icon: Server, progress: 0 },
    { slug: 'networking', title: 'CNI & Gateway API', lessons: '14 Lessons', xp: '2,100 XP', icon: Network, progress: 20 },
    { slug: 'security', title: 'Security & Policy Hardening', lessons: '15 Lessons', xp: '2,250 XP', icon: ShieldCheck, progress: 10 },
    { slug: 'gitops', title: 'GitOps & Argo CD', lessons: '12 Lessons', xp: '2,000 XP', icon: GitBranch, progress: 30 },
    { slug: 'observability', title: 'OpenTelemetry & Prometheus', lessons: '15 Lessons', xp: '2,400 XP', icon: Activity, progress: 15 },
    { slug: 'service-mesh', title: 'Istio Service Mesh', lessons: '14 Lessons', xp: '2,300 XP', icon: Layers, progress: 0 },
    { slug: 'incidents', title: 'Incident Response & Chaos', lessons: '10 Scenarios', xp: '3,000 XP', icon: AlertTriangle, progress: 5, alert: true },
  ];

  return (
    <div className="container-max py-8 space-y-10">
      {/* Hero Banner */}
      <section className="relative overflow-hidden rounded-2xl bg-gradient-to-r from-slate-900 via-indigo-950/40 to-slate-900 border border-slate-800 p-8 sm:p-12 shadow-2xl">
        <div className="absolute top-0 right-0 -mt-8 -mr-8 w-96 h-96 bg-cyan-500/10 rounded-full blur-3xl pointer-events-none" />
        <div className="relative z-10 max-w-3xl space-y-4">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-cyan-500/10 border border-cyan-500/20 text-cyan-400 text-xs font-semibold tracking-wide uppercase">
            <Sparkles className="w-3.5 h-3.5" />
            <span>Zero Mockups • 100% Real Live Kubernetes Sandboxes</span>
          </div>
          <h1 className="text-3xl sm:text-5xl font-extrabold tracking-tight text-white leading-tight">
            Prove Your Cloud-Native Engineering Skills on <span className="gradient-text-cyan">Real Infrastructure</span>
          </h1>
          <p className="text-base sm:text-lg text-slate-300">
            Learn Linux, Podman, Kubernetes, GitOps, Istio, OpenTelemetry, and SRE. Every task is graded against live system state — not brittle shell regex.
          </p>

          <div className="flex flex-wrap gap-4 pt-2">
            <Link
              href="/labs"
              className="inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-cyan-500 hover:bg-cyan-400 text-slate-950 font-bold text-sm shadow-lg shadow-cyan-500/25 transition-all hover:scale-[1.02]"
            >
              <Terminal className="w-4 h-4" />
              <span>Launch Live Lab</span>
            </Link>
            <Link
              href="/learn"
              className="inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-slate-800/80 hover:bg-slate-700 text-white font-semibold text-sm border border-slate-700 transition-all"
            >
              <span>Explore 12 Curriculum Tracks</span>
              <ArrowRight className="w-4 h-4" />
            </Link>
          </div>
        </div>
      </section>

      {/* Quick Stats Grid */}
      <section className="grid grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6">
        {quickStats.map((stat, idx) => {
          const Icon = stat.icon;
          return (
            <div key={idx} className="glass-panel p-5 space-y-2 relative overflow-hidden">
              <div className="flex items-center justify-between text-slate-400">
                <span className="text-xs font-medium uppercase tracking-wider">{stat.label}</span>
                <Icon className={`w-5 h-5 ${stat.color}`} />
              </div>
              <div className="text-2xl sm:text-3xl font-extrabold text-white">{stat.value}</div>
              <div className="text-xs text-slate-400 font-mono">{stat.change}</div>
            </div>
          );
        })}
      </section>

      {/* Resume Card & Incident Simulator Banner */}
      <section className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Continue Learning */}
        <div className="lg:col-span-2 glass-panel p-6 sm:p-8 space-y-4">
          <div className="flex items-center justify-between">
            <span className="text-xs font-semibold uppercase tracking-wider text-cyan-400 font-mono">
              CONTINUE LEARNING
            </span>
            <span className="text-xs text-slate-400 font-mono">Track: Kubernetes Core</span>
          </div>

          <div>
            <h3 className="text-xl font-bold text-white">Understanding Pods: The Atomic Primitive</h3>
            <p className="text-sm text-slate-300 mt-1">
              Pause containers, shared network namespaces, localhost routing, and declarative resource limits.
            </p>
          </div>

          <div className="space-y-1.5 pt-2">
            <div className="flex justify-between text-xs font-mono text-slate-400">
              <span>Progress</span>
              <span>45% Complete</span>
            </div>
            <div className="w-full h-2 rounded-full bg-slate-800 overflow-hidden">
              <div className="h-full bg-cyan-500 rounded-full w-[45%]" />
            </div>
          </div>

          <div className="pt-2 flex items-center justify-between">
            <div className="flex items-center gap-2 text-xs text-slate-400">
              <CheckCircle2 className="w-4 h-4 text-emerald-400" />
              <span>Matching Lab Sandbox Ready</span>
            </div>
            <Link
              href="/learn/kubernetes/understanding-pods"
              className="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white font-medium text-xs transition-colors"
            >
              <Play className="w-3.5 h-3.5 fill-current" />
              <span>Resume Lesson</span>
            </Link>
          </div>
        </div>

        {/* Incident Alert Banner */}
        <div className="glass-panel p-6 sm:p-8 border-rose-500/30 bg-rose-950/10 space-y-4 flex flex-col justify-between">
          <div className="space-y-2">
            <div className="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full bg-rose-500/20 border border-rose-500/30 text-rose-400 text-xs font-bold font-mono animate-pulse">
              <AlertTriangle className="w-3.5 h-3.5" />
              <span>SEV-1 SIMULATOR</span>
            </div>
            <h3 className="text-lg font-bold text-white">CoreDNS Outage & Cascading 503s</h3>
            <p className="text-xs text-slate-300 leading-relaxed">
              Cluster-wide DNS failure detected. Troubleshoot live pods, restore ConfigMap, and restart the deployment before the timer runs out.
            </p>
          </div>

          <Link
            href="/incidents"
            className="inline-flex items-center justify-center gap-2 w-full px-4 py-2.5 rounded-lg bg-rose-600 hover:bg-rose-500 text-white font-bold text-xs shadow-lg shadow-rose-600/20 transition-colors"
          >
            <span>Enter Incident Room</span>
            <ArrowRight className="w-3.5 h-3.5" />
          </Link>
        </div>
      </section>

      {/* Curriculum Tracks Grid */}
      <section className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-2xl font-bold text-white">Cloud-Native Learning Tracks</h2>
            <p className="text-sm text-slate-400">Progressive competencies from Linux fundamentals to multi-cluster platform engineering.</p>
          </div>
          <Link href="/learn" className="text-sm font-semibold text-cyan-400 hover:text-cyan-300 flex items-center gap-1">
            <span>View All Tracks</span>
            <ArrowRight className="w-4 h-4" />
          </Link>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {tracks.map((t) => {
            const Icon = t.icon;
            return (
              <Link key={t.slug} href={`/learn`}>
                <div className="glass-panel-interactive p-6 space-y-4 h-full flex flex-col justify-between">
                  <div className="space-y-3">
                    <div className="flex items-center justify-between">
                      <div className="w-10 h-10 rounded-xl bg-slate-800 border border-slate-700 flex items-center justify-center text-cyan-400">
                        <Icon className="w-5 h-5" />
                      </div>
                      <span className="text-xs font-mono text-slate-400">{t.xp}</span>
                    </div>
                    <h3 className="text-base font-bold text-white group-hover:text-cyan-400 transition-colors">
                      {t.title}
                    </h3>
                  </div>

                  <div className="space-y-2">
                    <div className="flex justify-between text-xs text-slate-400 font-mono">
                      <span>{t.lessons}</span>
                      <span>{t.progress}% Mastered</span>
                    </div>
                    <div className="w-full h-1.5 rounded-full bg-slate-800 overflow-hidden">
                      <div className="h-full bg-cyan-500 rounded-full" style={{ width: `${t.progress}%` }} />
                    </div>
                  </div>
                </div>
              </Link>
            );
          })}
        </div>
      </section>
    </div>
  );
}
