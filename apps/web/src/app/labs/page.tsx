'use client';

import React, { useState } from 'react';
import Link from 'next/link';
import {
  FlaskConical,
  Terminal,
  Boxes,
  Network,
  ShieldCheck,
  Package,
  GitBranch,
  Activity,
  Layers,
  Search,
  Clock,
  Zap,
  Play,
  CheckCircle2,
} from 'lucide-react';

const LABS_CATALOG = [
  {
    id: 'k8s-pod-basics',
    title: 'Create and Configure Your First Pod',
    difficulty: 'Beginner',
    duration: '15 mins',
    track: 'Kubernetes Core',
    points: 100,
    tasksCount: 2,
    icon: Boxes,
    description: 'Deploy a stateless web-server Pod, configure containerPort 80, and set custom labels.',
  },
  {
    id: 'k8s-deployments-scaling',
    title: 'Deployments, Scaling and Rolling Updates',
    difficulty: 'Intermediate',
    duration: '20 mins',
    track: 'Kubernetes Core',
    points: 100,
    tasksCount: 2,
    icon: Boxes,
    description: 'Create a multi-replica Deployment, configure RollingUpdate parameters (maxSurge, maxUnavailable).',
  },
  {
    id: 'k8s-services-clusterip',
    title: 'Exposing Pods with ClusterIP & Endpoints',
    difficulty: 'Beginner',
    duration: '15 mins',
    track: 'Networking',
    points: 100,
    tasksCount: 1,
    icon: Network,
    description: 'Expose a microservice using a ClusterIP Service and verify kube-proxy endpoint connectivity.',
  },
  {
    id: 'k8s-rbac-role-binding',
    title: 'Restricting Pod Access with RBAC Roles',
    difficulty: 'Intermediate',
    duration: '20 mins',
    track: 'Security',
    points: 100,
    tasksCount: 2,
    icon: ShieldCheck,
    description: 'Create fine-grained Role and RoleBinding rules for a developer ServiceAccount.',
  },
  {
    id: 'gitops-argocd-drift',
    title: 'Detecting & Self-Healing GitOps Drift with Argo CD',
    difficulty: 'Intermediate',
    duration: '25 mins',
    track: 'GitOps',
    points: 100,
    tasksCount: 1,
    icon: GitBranch,
    description: 'Configure automated reconciliation, self-healing, and pruning policies in Argo CD.',
  },
  {
    id: 'mesh-istio-canary',
    title: 'Istio 90/10 Weighted Canary Traffic Shifting',
    difficulty: 'Advanced',
    duration: '25 mins',
    track: 'Service Mesh',
    points: 100,
    tasksCount: 1,
    icon: Layers,
    description: 'Configure VirtualService and DestinationRule objects for weighted Layer 7 traffic routing.',
  },
];

export default function LabsCatalogPage() {
  const [search, setSearch] = useState('');

  const filteredLabs = LABS_CATALOG.filter(
    (l) =>
      l.title.toLowerCase().includes(search.toLowerCase()) ||
      l.track.toLowerCase().includes(search.toLowerCase()) ||
      l.description.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <div className="container-max py-10 space-y-8">
      {/* Header */}
      <div className="space-y-3">
        <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-cyan-500/10 border border-cyan-500/20 text-cyan-400 text-xs font-semibold uppercase tracking-wider">
          <FlaskConical className="w-3.5 h-3.5" />
          <span>Live Lab Environments</span>
        </div>
        <h1 className="text-3xl sm:text-4xl font-extrabold text-white">
          Hands-On Declarative <span className="gradient-text-cyan">Lab Catalog</span>
        </h1>
        <p className="text-slate-300 text-sm sm:text-base max-w-2xl">
          Every lab provisions a live, sandboxed Kubernetes environment. Solve objectives using real kubectl commands and Monaco YAML manifests.
        </p>
      </div>

      {/* Search Filter */}
      <div className="relative max-w-md">
        <Search className="w-4 h-4 text-slate-400 absolute left-3.5 top-1/2 -translate-y-1/2" />
        <input
          type="text"
          placeholder="Search labs by name, track, or tool..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="w-full bg-slate-900 border border-slate-800 rounded-xl pl-10 pr-4 py-2.5 text-sm text-slate-100 placeholder:text-slate-500 focus:outline-none focus:border-cyan-500 transition-colors"
        />
      </div>

      {/* Labs Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {filteredLabs.map((lab) => {
          const Icon = lab.icon;
          return (
            <div
              key={lab.id}
              className="glass-panel-interactive p-6 space-y-4 flex flex-col justify-between"
            >
              <div className="space-y-3">
                <div className="flex items-center justify-between">
                  <div className="w-10 h-10 rounded-xl bg-slate-800 border border-slate-700 flex items-center justify-center text-cyan-400">
                    <Icon className="w-5 h-5" />
                  </div>
                  <span className="px-2.5 py-0.5 rounded-full bg-slate-800 border border-slate-700 text-slate-300 text-[10px] font-mono uppercase font-bold">
                    {lab.difficulty}
                  </span>
                </div>

                <div className="space-y-1.5">
                  <span className="text-[11px] font-mono text-cyan-400 uppercase font-semibold">
                    {lab.track}
                  </span>
                  <h3 className="text-base font-bold text-white">{lab.title}</h3>
                  <p className="text-xs text-slate-400 leading-relaxed line-clamp-2">
                    {lab.description}
                  </p>
                </div>
              </div>

              <div className="pt-4 border-t border-slate-800/80 flex items-center justify-between">
                <div className="flex items-center gap-3 text-xs text-slate-400 font-mono">
                  <span className="flex items-center gap-1">
                    <Clock className="w-3.5 h-3.5 text-slate-500" />
                    <span>{lab.duration}</span>
                  </span>
                  <span className="flex items-center gap-1 text-amber-400">
                    <Zap className="w-3.5 h-3.5" />
                    <span>{lab.points} PTS</span>
                  </span>
                </div>

                <Link
                  href={`/labs/${lab.id}`}
                  className="inline-flex items-center gap-1.5 px-3.5 py-1.5 rounded-lg bg-cyan-500 hover:bg-cyan-400 text-slate-950 font-bold text-xs shadow-md shadow-cyan-500/20 transition-all"
                >
                  <Play className="w-3 h-3 fill-current" />
                  <span>Launch</span>
                </Link>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
