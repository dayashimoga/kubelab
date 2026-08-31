'use client';

import React, { useState } from 'react';
import Link from 'next/link';
import {
  Terminal,
  Boxes,
  Server,
  Network,
  ShieldCheck,
  Package,
  GitBranch,
  Activity,
  Layers,
  Gauge,
  Cpu,
  AlertTriangle,
  Search,
  BookOpen,
  ArrowRight,
  Clock,
  Zap,
} from 'lucide-react';

const TRACKS_DATA = [
  {
    id: 'foundations',
    title: 'Cloud-Native & Linux Foundations',
    description: 'Linux systems engineering, shell scripting, namespaces, cgroups, and OCI container fundamentals with Docker and Podman.',
    icon: Terminal,
    difficulty: 'Beginner',
    duration: '6 hours',
    lessonsCount: 15,
    xp: 1500,
    color: 'from-cyan-500/20 to-blue-500/20',
    tags: ['Linux', 'Bash', 'Docker', 'Podman', 'cgroups'],
  },
  {
    id: 'kubernetes',
    title: 'Kubernetes Core Architecture & Workloads',
    description: 'Master Pods, Deployments, Services, ConfigMaps, Secrets, Storage, and Declarative manifests in real Kubernetes clusters.',
    icon: Boxes,
    difficulty: 'Beginner',
    duration: '8 hours',
    lessonsCount: 20,
    xp: 2500,
    color: 'from-indigo-500/20 to-cyan-500/20',
    tags: ['Pods', 'Deployments', 'Services', 'ConfigMaps', 'Storage'],
  },
  {
    id: 'k8s-admin',
    title: 'Cluster Administration & etcd Operations',
    description: 'Bootstrap clusters with kubeadm, handle control-plane high availability, etcd snapshots, disaster recovery, and node upgrades.',
    icon: Server,
    difficulty: 'Intermediate',
    duration: '5 hours',
    lessonsCount: 12,
    xp: 1800,
    color: 'from-blue-500/20 to-indigo-500/20',
    tags: ['kubeadm', 'etcd', 'HA Control Plane', 'Backup & Restore'],
  },
  {
    id: 'networking',
    title: 'Cloud-Native Networking, CNI & Gateway API',
    description: 'Deep dive into CNI plugins (Calico/Cilium), CoreDNS resolution, Ingress Controllers, eBPF data planes, and Gateway API.',
    icon: Network,
    difficulty: 'Intermediate',
    duration: '7 hours',
    lessonsCount: 14,
    xp: 2100,
    color: 'from-emerald-500/20 to-cyan-500/20',
    tags: ['CNI', 'CoreDNS', 'Gateway API', 'eBPF', 'Calico', 'Cilium'],
  },
  {
    id: 'security',
    title: 'Kubernetes Security, RBAC & Policy Hardening',
    description: 'Implement RBAC, Pod Security Standards, NetworkPolicies, Seccomp profiles, image vulnerability scanning, and CIS benchmarks.',
    icon: ShieldCheck,
    difficulty: 'Intermediate',
    duration: '7 hours',
    lessonsCount: 15,
    xp: 2250,
    color: 'from-rose-500/20 to-amber-500/20',
    tags: ['RBAC', 'PSS/PSA', 'NetworkPolicy', 'Seccomp', 'Trivy'],
  },
  {
    id: 'helm',
    title: 'Packaging with Helm & Kustomize',
    description: 'Author production Helm charts, manage chart repositories, leverage Go templates, and apply dry Kustomize overlays.',
    icon: Package,
    difficulty: 'Intermediate',
    duration: '4 hours',
    lessonsCount: 10,
    xp: 1500,
    color: 'from-amber-500/20 to-orange-500/20',
    tags: ['Helm', 'Charts', 'Go Templates', 'Kustomize', 'Overlays'],
  },
  {
    id: 'gitops',
    title: 'GitOps & Continuous Delivery with Argo CD',
    description: 'Implement declarative GitOps workflows, App-of-Apps pattern, automated sync policies, drift detection, and automated rollbacks.',
    icon: GitBranch,
    difficulty: 'Intermediate',
    duration: '5 hours',
    lessonsCount: 12,
    xp: 2000,
    color: 'from-purple-500/20 to-indigo-500/20',
    tags: ['Argo CD', 'GitOps', 'Drift Detection', 'Self-Healing', 'App-of-Apps'],
  },
  {
    id: 'observability',
    title: 'OpenTelemetry, Prometheus & Grafana',
    description: 'End-to-end distributed tracing with OpenTelemetry, metric collection with Prometheus, PromQL alerting, and Grafana dashboarding.',
    icon: Activity,
    difficulty: 'Advanced',
    duration: '7 hours',
    lessonsCount: 15,
    xp: 2400,
    color: 'from-cyan-500/20 to-emerald-500/20',
    tags: ['OTel', 'Prometheus', 'Grafana', 'PromQL', 'Tracing', 'Logs'],
  },
  {
    id: 'service-mesh',
    title: 'Service Mesh with Istio & Envoy Proxy',
    description: 'Traffic shifting, canary releases, mutual TLS (mTLS), fault injection, rate limiting, and Envoy sidecar telemetry.',
    icon: Layers,
    difficulty: 'Advanced',
    duration: '6 hours',
    lessonsCount: 14,
    xp: 2300,
    color: 'from-indigo-500/20 to-purple-500/20',
    tags: ['Istio', 'Envoy', 'mTLS', 'VirtualService', 'Canary'],
  },
  {
    id: 'sre',
    title: 'Site Reliability Engineering & SLOs',
    description: 'Define meaningful SLIs/SLOs, error budget burn rates, alerting thresholds, HPA/VPA autoscaling, and capacity planning.',
    icon: Gauge,
    difficulty: 'Advanced',
    duration: '5 hours',
    lessonsCount: 12,
    xp: 2000,
    color: 'from-blue-500/20 to-cyan-500/20',
    tags: ['SLI', 'SLO', 'Error Budgets', 'HPA', 'VPA', 'Alerting'],
  },
  {
    id: 'platform-eng',
    title: 'Platform Engineering & Multi-Cluster',
    description: 'Build Internal Developer Platforms (IDPs), write Kubernetes Operators and CRDs in Go/Rust, and manage multi-cluster fleets.',
    icon: Cpu,
    difficulty: 'Expert',
    duration: '8 hours',
    lessonsCount: 10,
    xp: 2500,
    color: 'from-violet-500/20 to-fuchsia-500/20',
    tags: ['IDP', 'Operators', 'CRDs', 'Crossplane', 'Multi-Cluster'],
  },
  {
    id: 'incidents',
    title: 'Production Incident Response & Chaos',
    description: 'Real-world break-fix simulations: debug crashloops, network partitions, DNS degradation, expired certs, and GitOps sync deadlocks.',
    icon: AlertTriangle,
    difficulty: 'Expert',
    duration: '6 hours',
    lessonsCount: 10,
    xp: 3000,
    color: 'from-rose-500/20 to-red-500/20',
    tags: ['Incident Triage', 'Chaos', 'Post-Mortem', 'MTTD', 'MTTR'],
  },
];

export default function LearnCatalogPage() {
  const [search, setSearch] = useState('');
  const [difficultyFilter, setDifficultyFilter] = useState('all');

  const filteredTracks = TRACKS_DATA.filter((track) => {
    const matchesSearch =
      track.title.toLowerCase().includes(search.toLowerCase()) ||
      track.description.toLowerCase().includes(search.toLowerCase()) ||
      track.tags.some((t) => t.toLowerCase().includes(search.toLowerCase()));

    const matchesDiff =
      difficultyFilter === 'all' ||
      track.difficulty.toLowerCase() === difficultyFilter.toLowerCase();

    return matchesSearch && matchesDiff;
  });

  return (
    <div className="container-max py-10 space-y-10">
      {/* Header */}
      <div className="space-y-4">
        <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-cyan-500/10 border border-cyan-500/20 text-cyan-400 text-xs font-semibold uppercase tracking-wider">
          <BookOpen className="w-3.5 h-3.5" />
          <span>Curriculum Catalog</span>
        </div>
        <h1 className="text-3xl sm:text-4xl font-extrabold text-white">
          Cloud-Native Engineering <span className="gradient-text-cyan">Mastery Tracks</span>
        </h1>
        <p className="text-slate-300 text-base max-w-3xl">
          Comprehensive curriculum from Linux foundations through Kubernetes architecture, networking, security, GitOps, observability, service mesh, and live production incidents.
        </p>
      </div>

      {/* Search & Filter Bar */}
      <div className="flex flex-col sm:flex-row gap-4 items-center justify-between">
        <div className="relative w-full sm:w-96">
          <Search className="w-4 h-4 text-slate-400 absolute left-3.5 top-1/2 -translate-y-1/2" />
          <input
            type="text"
            placeholder="Search tracks, concepts, tools (e.g. Istio, RBAC, eBPF)..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="w-full bg-slate-900 border border-slate-800 rounded-xl pl-10 pr-4 py-2.5 text-sm text-slate-100 placeholder:text-slate-500 focus:outline-none focus:border-cyan-500 transition-colors"
          />
        </div>

        {/* Difficulty Pill Filters */}
        <div className="flex items-center gap-2 w-full sm:w-auto overflow-x-auto pb-2 sm:pb-0">
          {['all', 'beginner', 'intermediate', 'advanced', 'expert'].map((d) => (
            <button
              key={d}
              onClick={() => setDifficultyFilter(d)}
              className={`px-3.5 py-1.5 rounded-lg text-xs font-medium capitalize whitespace-nowrap transition-colors ${
                difficultyFilter === d
                  ? 'bg-cyan-500 text-slate-950 font-bold'
                  : 'bg-slate-900 text-slate-400 hover:text-slate-200 border border-slate-800'
              }`}
            >
              {d}
            </button>
          ))}
        </div>
      </div>

      {/* Tracks Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {filteredTracks.map((track) => {
          const Icon = track.icon;
          return (
            <Link key={track.id} href={`/learn/kubernetes/understanding-pods`}>
              <div className="glass-panel-interactive p-6 space-y-5 h-full flex flex-col justify-between group">
                <div className="space-y-4">
                  {/* Top row */}
                  <div className="flex items-center justify-between">
                    <div className="w-12 h-12 rounded-xl bg-slate-800/80 border border-slate-700/80 flex items-center justify-center text-cyan-400 group-hover:scale-105 group-hover:border-cyan-500/50 transition-all">
                      <Icon className="w-6 h-6" />
                    </div>
                    <span className="px-2.5 py-1 rounded-full bg-slate-800 border border-slate-700 text-slate-300 text-[11px] font-mono font-semibold uppercase">
                      {track.difficulty}
                    </span>
                  </div>

                  {/* Title & Description */}
                  <div className="space-y-2">
                    <h3 className="text-lg font-bold text-white group-hover:text-cyan-400 transition-colors">
                      {track.title}
                    </h3>
                    <p className="text-xs text-slate-400 leading-relaxed line-clamp-3">
                      {track.description}
                    </p>
                  </div>

                  {/* Tags */}
                  <div className="flex flex-wrap gap-1.5">
                    {track.tags.slice(0, 4).map((tag, idx) => (
                      <span
                        key={idx}
                        className="px-2 py-0.5 rounded-md bg-slate-800/60 border border-slate-700/50 text-slate-400 text-[10px] font-mono"
                      >
                        {tag}
                      </span>
                    ))}
                  </div>
                </div>

                {/* Footer Metadata */}
                <div className="pt-4 border-t border-slate-800/80 flex items-center justify-between text-xs text-slate-400 font-mono">
                  <div className="flex items-center gap-3">
                    <span className="flex items-center gap-1">
                      <Clock className="w-3.5 h-3.5 text-slate-500" />
                      <span>{track.duration}</span>
                    </span>
                    <span className="flex items-center gap-1">
                      <Zap className="w-3.5 h-3.5 text-amber-400" />
                      <span>{track.xp} XP</span>
                    </span>
                  </div>
                  <span className="text-cyan-400 font-semibold group-hover:translate-x-1 transition-transform flex items-center gap-1">
                    <span>Enter Track</span>
                    <ArrowRight className="w-3.5 h-3.5" />
                  </span>
                </div>
              </div>
            </Link>
          );
        })}
      </div>
    </div>
  );
}
