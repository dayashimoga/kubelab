'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';
import {
  GitFork,
  Terminal,
  Boxes,
  Network,
  ShieldCheck,
  GitBranch,
  Layers,
  Activity,
  AlertTriangle,
  ArrowRight,
  Sparkles,
} from 'lucide-react';
import { api, SkillNode } from '@/lib/api';

const ICON_MAP: Record<string, any> = {
  terminal: Terminal,
  boxes: Boxes,
  network: Network,
  shieldcheck: ShieldCheck,
  gitbranch: GitBranch,
  layers: Layers,
  activity: Activity,
  alerttriangle: AlertTriangle,
};

/** Default fallback skill nodes for when API is unreachable */
const DEFAULT_SKILL_NODES: (SkillNode & { icon?: any })[] = [
  {
    id: 'skill-linux',
    name: 'Linux Systems & CLI',
    category: 'Foundations',
    level: 3,
    xp: 350,
    description: 'Linux systems engineering, bash scripting, process management, and permissions.',
    prerequisites: [],
    recommended_lab: 'k8s-pod-basics',
  },
  {
    id: 'skill-containers',
    name: 'OCI Containers & Podman',
    category: 'Foundations',
    level: 2,
    xp: 250,
    description: 'Linux namespaces, cgroups v2, multi-stage builds, and rootless Podman.',
    prerequisites: ['Linux Systems & CLI'],
    recommended_lab: 'k8s-pod-basics',
  },
  {
    id: 'skill-k8s-workloads',
    name: 'Kubernetes Workloads',
    category: 'Kubernetes Core',
    level: 4,
    xp: 600,
    description: 'Pods, Deployments, ReplicaSets, StatefulSets, DaemonSets, and Jobs.',
    prerequisites: ['OCI Containers & Podman'],
    recommended_lab: 'k8s-deployments-scaling',
  },
  {
    id: 'skill-networking',
    name: 'Cloud-Native Networking',
    category: 'Networking',
    level: 3,
    xp: 450,
    description: 'Services, Endpoints, CNI plugins (Calico/Cilium), CoreDNS, and Ingress.',
    prerequisites: ['Kubernetes Workloads'],
    recommended_lab: 'k8s-services-clusterip',
  },
  {
    id: 'skill-gitops',
    name: 'GitOps & Argo CD',
    category: 'GitOps',
    level: 3,
    xp: 400,
    description: 'Continuous Delivery with Argo CD, automated sync policies, and drift detection.',
    prerequisites: ['Kubernetes Workloads'],
    recommended_lab: 'gitops-argocd-drift',
  },
  {
    id: 'skill-service-mesh',
    name: 'Istio Service Mesh',
    category: 'Service Mesh',
    level: 2,
    xp: 300,
    description: 'Envoy sidecar injection, VirtualServices, DestinationRules, and mTLS.',
    prerequisites: ['Cloud-Native Networking'],
    recommended_lab: 'mesh-istio-canary',
  },
  {
    id: 'skill-observability',
    name: 'OpenTelemetry & Prometheus',
    category: 'Observability',
    level: 3,
    xp: 500,
    description: 'Distributed tracing, metric scraping, PromQL alert rules, and Grafana.',
    prerequisites: ['Kubernetes Workloads'],
    recommended_lab: 'otel-tracing-prometheus',
  },
  {
    id: 'skill-incidents',
    name: 'Incident Response & SRE',
    category: 'SRE & Chaos',
    level: 2,
    xp: 350,
    description: 'Triage live outages under pressure, diagnose crashloops, and resolve SEV-1 alerts.',
    prerequisites: ['OpenTelemetry & Prometheus', 'Cloud-Native Networking'],
    recommended_lab: 'incident-coredns-failure',
  },
];

/** Resolve an icon component from name or category */
function resolveIcon(node: SkillNode): any {
  // Try category-based mapping
  const cat = node.category.toLowerCase();
  if (cat.includes('foundation')) return Terminal;
  if (cat.includes('kubernetes')) return Boxes;
  if (cat.includes('networking')) return Network;
  if (cat.includes('gitops')) return GitBranch;
  if (cat.includes('service mesh')) return Layers;
  if (cat.includes('observability')) return Activity;
  if (cat.includes('sre') || cat.includes('chaos') || cat.includes('incident')) return AlertTriangle;
  if (cat.includes('security')) return ShieldCheck;
  return Boxes;
}

export default function SkillTreePage() {
  const [skillNodes, setSkillNodes] = useState<SkillNode[]>(DEFAULT_SKILL_NODES);
  const [selectedSkill, setSelectedSkill] = useState<SkillNode>(DEFAULT_SKILL_NODES[2]);

  useEffect(() => {
    const load = async () => {
      try {
        const data = await api.getSkillNodes();
        if (data.length > 0) {
          setSkillNodes(data);
          setSelectedSkill(data[0]);
        }
      } catch {
        // Use defaults
      }
    };
    load();
  }, []);

  return (
    <div className="container-max py-10 space-y-8">
      {/* Header */}
      <div className="space-y-3">
        <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-cyan-500/10 border border-cyan-500/20 text-cyan-400 text-xs font-semibold uppercase tracking-wider">
          <GitFork className="w-3.5 h-3.5" />
          <span>Competency Mastery Graph</span>
        </div>
        <h1 className="text-3xl sm:text-4xl font-extrabold text-white">
          Cloud-Native <span className="gradient-text-cyan">Skill Tree DAG</span>
        </h1>
        <p className="text-slate-300 text-sm max-w-2xl">
          Visual representation of your engineering competencies. Master foundational nodes to unlock advanced multi-cluster and incident triage paths.
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-12 gap-8">
        {/* Visual Graph View */}
        <div className="lg:col-span-8 glass-panel p-6 sm:p-8 space-y-6">
          <div className="flex items-center justify-between border-b border-slate-800 pb-4">
            <span className="text-xs font-mono uppercase text-slate-400 font-bold">
              DAG Visualizer (Click a node to inspect)
            </span>
            <div className="flex items-center gap-3 text-xs font-mono text-cyan-400">
              <Sparkles className="w-3.5 h-3.5" />
              <span>{skillNodes.length} Competencies Tracked</span>
            </div>
          </div>

          {/* Node Grid */}
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
            {skillNodes.map((node) => {
              const Icon = resolveIcon(node);
              const isSelected = selectedSkill.id === node.id;
              return (
                <div
                  key={node.id}
                  onClick={() => setSelectedSkill(node)}
                  className={`p-5 rounded-2xl border transition-all cursor-pointer space-y-3 ${
                    isSelected
                      ? 'bg-indigo-950/40 border-cyan-500 shadow-xl shadow-cyan-500/10 scale-[1.02]'
                      : 'bg-slate-900/70 border-slate-800 hover:border-slate-700'
                  }`}
                >
                  <div className="flex items-center justify-between">
                    <div className="w-10 h-10 rounded-xl bg-slate-800 flex items-center justify-center text-cyan-400">
                      <Icon className="w-5 h-5" />
                    </div>
                    <div className="flex items-center gap-1">
                      {[1, 2, 3, 4, 5].map((lvl) => (
                        <span
                          key={lvl}
                          className={`w-2.5 h-2.5 rounded-full ${
                            lvl <= node.level ? 'bg-cyan-400 shadow-sm shadow-cyan-400/50' : 'bg-slate-800'
                          }`}
                        />
                      ))}
                    </div>
                  </div>

                  <div>
                    <h3 className="text-sm font-bold text-white">{node.name}</h3>
                    <span className="text-[11px] font-mono text-slate-400">{node.category}</span>
                  </div>

                  <div className="flex justify-between items-center text-xs font-mono text-slate-400 pt-2 border-t border-slate-800/80">
                    <span>Level {node.level} / 5</span>
                    <span className="text-amber-400">{node.xp} XP</span>
                  </div>
                </div>
              );
            })}
          </div>
        </div>

        {/* Right Sidebar: Selected Skill Node Details */}
        <div className="lg:col-span-4 glass-panel p-6 sm:p-8 space-y-6 flex flex-col justify-between">
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <span className="text-[11px] font-mono text-cyan-400 uppercase font-semibold">
                {selectedSkill.category}
              </span>
              <span className="px-2.5 py-0.5 rounded-full bg-cyan-500/10 border border-cyan-500/20 text-cyan-300 font-mono text-xs font-bold">
                Level {selectedSkill.level} Mastery
              </span>
            </div>

            <h2 className="text-xl font-bold text-white">{selectedSkill.name}</h2>
            <p className="text-xs text-slate-300 leading-relaxed">{selectedSkill.description}</p>

            {/* Prerequisites */}
            <div className="space-y-2 pt-2">
              <span className="text-xs font-mono font-bold text-slate-400 uppercase">
                Prerequisites
              </span>
              {selectedSkill.prerequisites.length === 0 ? (
                <p className="text-xs text-emerald-400 font-mono">None (Foundational Node)</p>
              ) : (
                <div className="flex flex-wrap gap-1.5">
                  {selectedSkill.prerequisites.map((p, idx) => (
                    <span
                      key={idx}
                      className="px-2.5 py-1 rounded bg-slate-800 border border-slate-700 text-slate-300 text-[11px] font-mono"
                    >
                      {p}
                    </span>
                  ))}
                </div>
              )}
            </div>
          </div>

          {/* Action CTA */}
          <div className="pt-4 border-t border-slate-800 space-y-3">
            <Link
              href={`/labs/${selectedSkill.recommended_lab}`}
              className="inline-flex items-center justify-center gap-2 w-full px-4 py-3 rounded-xl bg-cyan-500 hover:bg-cyan-400 text-slate-950 font-bold text-xs shadow-lg shadow-cyan-500/20 transition-all"
            >
              <span>Level Up in Sandbox Lab</span>
              <ArrowRight className="w-4 h-4" />
            </Link>
          </div>
        </div>
      </div>
    </div>
  );
}
