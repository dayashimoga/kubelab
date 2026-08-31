'use client';

import React, { useState, useEffect } from 'react';
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
import { api, TrackSummary } from '@/lib/api';

const ICON_MAP: Record<string, any> = {
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
};

const DEFAULT_TRACKS = [
  {
    id: 'track-foundations',
    slug: 'foundations',
    title: 'Cloud-Native & Linux Foundations',
    description: 'Linux systems engineering, shell scripting, namespaces, cgroups, and OCI container fundamentals with Docker and Podman.',
    icon: 'Terminal',
    difficulty: 'Beginner',
    order: 1,
    total_lessons: 15,
    total_xp: 1500,
  },
  {
    id: 'track-kubernetes',
    slug: 'kubernetes',
    title: 'Kubernetes Core Architecture & Workloads',
    description: 'Master Pods, Deployments, Services, ConfigMaps, Secrets, Storage, and Declarative manifests in real Kubernetes clusters.',
    icon: 'Boxes',
    difficulty: 'Beginner',
    order: 2,
    total_lessons: 20,
    total_xp: 2500,
  },
  {
    id: 'track-k8s-admin',
    slug: 'k8s-admin',
    title: 'Cluster Administration & etcd Operations',
    description: 'Bootstrap clusters with kubeadm, handle control-plane high availability, etcd snapshots, disaster recovery, and node upgrades.',
    icon: 'Server',
    difficulty: 'Intermediate',
    order: 3,
    total_lessons: 12,
    total_xp: 1800,
  },
  {
    id: 'track-networking',
    slug: 'networking',
    title: 'Cloud-Native Networking, CNI & Gateway API',
    description: 'Deep dive into CNI plugins (Calico/Cilium), CoreDNS resolution, Ingress Controllers, eBPF data planes, and Kubernetes Gateway API.',
    icon: 'Network',
    difficulty: 'Intermediate',
    order: 4,
    total_lessons: 14,
    total_xp: 2100,
  },
  {
    id: 'track-security',
    slug: 'security',
    title: 'Kubernetes Security, RBAC & Policy Hardening',
    description: 'Implement RBAC, Pod Security Standards, NetworkPolicies, Seccomp profiles, image vulnerability scanning, and CIS benchmarks.',
    icon: 'ShieldCheck',
    difficulty: 'Intermediate',
    order: 5,
    total_lessons: 15,
    total_xp: 2250,
  },
  {
    id: 'track-helm',
    slug: 'helm',
    title: 'Packaging with Helm & Kustomize',
    description: 'Author production Helm charts, manage chart repositories, leverage Go templates, and apply dry Kustomize overlays.',
    icon: 'Package',
    difficulty: 'Intermediate',
    order: 6,
    total_lessons: 10,
    total_xp: 1500,
  },
  {
    id: 'track-gitops',
    slug: 'gitops',
    title: 'GitOps & Continuous Delivery with Argo CD',
    description: 'Implement declarative GitOps workflows, App-of-Apps pattern, automated sync policies, drift detection, and automated rollbacks.',
    icon: 'GitBranch',
    difficulty: 'Intermediate',
    order: 7,
    total_lessons: 12,
    total_xp: 2000,
  },
  {
    id: 'track-observability',
    slug: 'observability',
    title: 'OpenTelemetry, Prometheus & Grafana',
    description: 'End-to-end distributed tracing with OpenTelemetry, metric collection with Prometheus, PromQL alerting, and Grafana dashboarding.',
    icon: 'Activity',
    difficulty: 'Advanced',
    order: 8,
    total_lessons: 15,
    total_xp: 2400,
  },
  {
    id: 'track-service-mesh',
    slug: 'service-mesh',
    title: 'Service Mesh with Istio & Envoy Proxy',
    description: 'Traffic shifting, canary releases, mutual TLS (mTLS), fault injection, rate limiting, and Envoy sidecar telemetry.',
    icon: 'Layers',
    difficulty: 'Advanced',
    order: 9,
    total_lessons: 14,
    total_xp: 2300,
  },
  {
    id: 'track-sre',
    slug: 'sre',
    title: 'Site Reliability Engineering & SLOs',
    description: 'Define meaningful SLIs/SLOs, error budget burn rates, alerting thresholds, HPA/VPA autoscaling, and capacity planning.',
    icon: 'Gauge',
    difficulty: 'Advanced',
    order: 10,
    total_lessons: 12,
    total_xp: 2000,
  },
  {
    id: 'track-platform-eng',
    slug: 'platform-eng',
    title: 'Platform Engineering & Multi-Cluster',
    description: 'Build Internal Developer Platforms (IDPs), write Kubernetes Operators and CRDs in Go/Rust, and manage multi-cluster fleets.',
    icon: 'Cpu',
    difficulty: 'Expert',
    order: 11,
    total_lessons: 10,
    total_xp: 2500,
  },
  {
    id: 'track-incidents',
    slug: 'incidents',
    title: 'Production Incident Response & Chaos',
    description: 'Real-world break-fix simulations: debug crashloops, network partitions, DNS degradation, expired certs, and GitOps sync deadlocks.',
    icon: 'AlertTriangle',
    difficulty: 'Expert',
    order: 12,
    total_lessons: 10,
    total_xp: 3000,
  },
];

export default function LearnIndexPage() {
  const [tracks, setTracks] = useState<TrackSummary[]>(DEFAULT_TRACKS);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedDifficulty, setSelectedDifficulty] = useState('all');

  useEffect(() => {
    api
      .getTracks()
      .then((data) => {
        if (data && data.length > 0) setTracks(data);
      })
      .catch(() => {});
  }, []);

  const filteredTracks = tracks.filter((track) => {
    const matchesSearch =
      track.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      track.description.toLowerCase().includes(searchQuery.toLowerCase());
    const matchesDifficulty =
      selectedDifficulty === 'all' ||
      track.difficulty.toLowerCase() === selectedDifficulty.toLowerCase();
    return matchesSearch && matchesDifficulty;
  });

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-10 space-y-8">
      {/* Header Banner */}
      <div className="flex flex-col md:flex-row md:items-end justify-between gap-6 pb-6 border-b border-slate-800">
        <div className="space-y-2 max-w-2xl">
          <div className="flex items-center gap-2 text-cyan-400 text-xs font-mono font-bold uppercase tracking-wider">
            <BookOpen className="w-4 h-4" />
            <span>Curriculum Tracks</span>
          </div>
          <h1 className="text-3xl sm:text-4xl font-extrabold text-white tracking-tight">
            Cloud-Native Engineering Tracks
          </h1>
          <p className="text-slate-400 text-sm leading-relaxed">
            12 progressive tracks from Linux kernel primitives to distributed Istio meshes, multi-cluster architectures, and production incident response.
          </p>
        </div>

        {/* Search and Filters */}
        <div className="flex flex-col sm:flex-row gap-3 w-full md:w-auto">
          <div className="relative flex-1 sm:w-64">
            <Search className="w-4 h-4 text-slate-500 absolute left-3 top-1/2 -translate-y-1/2" />
            <input
              type="text"
              placeholder="Search tracks..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full pl-9 pr-4 py-2 bg-slate-900 border border-slate-800 rounded-xl text-xs text-white placeholder:text-slate-500 focus:outline-none focus:border-cyan-500 transition-colors"
            />
          </div>

          <select
            value={selectedDifficulty}
            onChange={(e) => setSelectedDifficulty(e.target.value)}
            className="px-3 py-2 bg-slate-900 border border-slate-800 rounded-xl text-xs text-slate-300 focus:outline-none focus:border-cyan-500"
          >
            <option value="all">All Difficulties</option>
            <option value="beginner">Beginner</option>
            <option value="intermediate">Intermediate</option>
            <option value="advanced">Advanced</option>
            <option value="expert">Expert</option>
          </select>
        </div>
      </div>

      {/* Tracks Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {filteredTracks.map((track) => {
          const Icon = ICON_MAP[track.icon] || Boxes;
          return (
            <div
              key={track.id}
              className="flex flex-col justify-between p-6 rounded-2xl bg-slate-900/60 border border-slate-800 hover:border-cyan-500/40 transition-all group"
            >
              <div className="space-y-4">
                <div className="flex items-center justify-between">
                  <div className="w-12 h-12 rounded-xl bg-cyan-500/10 border border-cyan-500/20 flex items-center justify-center text-cyan-400 group-hover:scale-105 transition-transform">
                    <Icon className="w-6 h-6" />
                  </div>
                  <span className="px-2.5 py-1 rounded-full text-[10px] font-mono font-bold uppercase tracking-wider bg-slate-800 text-slate-300 border border-slate-700">
                    {track.difficulty}
                  </span>
                </div>

                <div className="space-y-2">
                  <h3 className="text-base font-bold text-white group-hover:text-cyan-300 transition-colors">
                    {track.title}
                  </h3>
                  <p className="text-xs text-slate-400 leading-relaxed line-clamp-3">
                    {track.description}
                  </p>
                </div>
              </div>

              <div className="pt-6 border-t border-slate-800/80 mt-6 flex items-center justify-between text-xs">
                <div className="flex items-center gap-3 text-slate-400 font-mono text-[11px]">
                  <span className="flex items-center gap-1">
                    <Clock className="w-3.5 h-3.5 text-slate-500" />
                    <span>{track.total_lessons} Lessons</span>
                  </span>
                  <span className="flex items-center gap-1 text-amber-400">
                    <Zap className="w-3.5 h-3.5" />
                    <span>{track.total_xp} XP</span>
                  </span>
                </div>

                <Link
                  href={`/labs/k8s-pod-basics`}
                  className="flex items-center gap-1 text-cyan-400 hover:text-cyan-300 font-semibold"
                >
                  <span>Start</span>
                  <ArrowRight className="w-3.5 h-3.5" />
                </Link>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
