'use client';

import React, { useState, useEffect } from 'react';
import {
  AlertTriangle,
  Activity,
  Terminal,
  Clock,
  ShieldAlert,
  CheckCircle2,
  Wifi,
  WifiOff,
  Zap,
} from 'lucide-react';
import { WebTerminal } from '@/components/Terminal';
import { api, IncidentScenario } from '@/lib/api';

/** Default fallback incident for offline/demo mode */
const DEFAULT_SERVICES = [
  { name: 'ingress-gateway', tier: 'Ingress', status: 'Degraded', errors: '14.2% HTTP 503' },
  { name: 'frontend-service', tier: 'Frontend', status: 'Failing', errors: 'DNS NXDOMAIN timeouts' },
  { name: 'order-api', tier: 'Backend', status: 'Degraded', errors: 'Connection Refused' },
  { name: 'coredns', tier: 'Infra', status: 'Failing', errors: 'ConfigMap syntax error' },
  { name: 'postgres-db', tier: 'Database', status: 'Healthy', errors: 'None' },
];

export default function IncidentSimulatorPage() {
  const [activeTab, setActiveTab] = useState<'topology' | 'terminal' | 'metrics'>('topology');
  const [resolved, setResolved] = useState(false);
  const [selectedDiagnosis, setSelectedDiagnosis] = useState<string | null>(null);
  const [services, setServices] = useState(DEFAULT_SERVICES);
  const [apiConnected, setApiConnected] = useState(false);

  useEffect(() => {
    const load = async () => {
      try {
        const data = await api.getIncidents();
        if (data.length > 0 && data[0].services?.length > 0) {
          setServices(data[0].services);
          setApiConnected(true);
        }
      } catch {
        // Use defaults
      }
    };
    load();
  }, []);

  const handleResolve = () => {
    if (selectedDiagnosis === 'coredns_configmap') {
      setResolved(true);
    } else {
      alert('Incorrect diagnosis! CoreDNS deployment logs indicate a corrupted Corefile configuration.');
    }
  };

  return (
    <div className="container-max py-8 space-y-6">
      {/* Incident Header & Live SLA Countdown */}
      <div className="p-6 rounded-2xl bg-rose-950/20 border border-rose-500/30 flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
        <div className="space-y-1.5">
          <div className="flex items-center gap-2">
            <span className="px-2.5 py-0.5 rounded-full bg-rose-500 text-slate-950 font-black text-xs font-mono animate-pulse">
              SEV-1 ALERT
            </span>
            <span className="text-rose-400 font-mono text-xs">INC-2026-0831</span>
          </div>
          <h1 className="text-2xl font-extrabold text-white">CoreDNS Outage & Cascading 503 Errors</h1>
          <p className="text-xs text-slate-300">
            Frontend microservices cannot resolve backend endpoints. Triage the cluster, diagnose the root cause, and restore connectivity.
          </p>
        </div>

        <div className="flex items-center gap-6 font-mono">
          <div className="text-right">
            <div className="text-[10px] text-slate-400">INCIDENT TIME LIMIT</div>
            <div className="text-xl font-extrabold text-rose-400">14:32</div>
          </div>
          <div className="text-right">
            <div className="text-[10px] text-slate-400">MAX BOUNTY</div>
            <div className="text-xl font-extrabold text-amber-400">350 XP</div>
          </div>
        </div>
      </div>

      {/* Main Workspace */}
      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6">
        {/* Left Column: Multi-Service Topology & Metrics */}
        <div className="lg:col-span-7 space-y-4">
          <div className="flex items-center justify-between border-b border-slate-800 pb-2">
            <div className="flex items-center gap-2">
              <button
                onClick={() => setActiveTab('topology')}
                className={`px-3 py-1.5 rounded-lg text-xs font-mono font-semibold transition-colors ${
                  activeTab === 'topology' ? 'bg-indigo-600 text-white' : 'bg-slate-900 text-slate-400'
                }`}
              >
                Service Topology
              </button>
              <button
                onClick={() => setActiveTab('terminal')}
                className={`px-3 py-1.5 rounded-lg text-xs font-mono font-semibold transition-colors ${
                  activeTab === 'terminal' ? 'bg-indigo-600 text-white' : 'bg-slate-900 text-slate-400'
                }`}
              >
                Triage Terminal
              </button>
              <button
                onClick={() => setActiveTab('metrics')}
                className={`px-3 py-1.5 rounded-lg text-xs font-mono font-semibold transition-colors ${
                  activeTab === 'metrics' ? 'bg-indigo-600 text-white' : 'bg-slate-900 text-slate-400'
                }`}
              >
                Prometheus Metrics
              </button>
            </div>

            <div className="flex items-center gap-2 text-xs font-mono text-rose-400">
              <Activity className="w-3.5 h-3.5 animate-pulse" />
              <span>Error Rate: 28.4%</span>
            </div>
          </div>

          {activeTab === 'topology' && (
            <div className="glass-panel p-6 space-y-4">
              <h3 className="text-xs font-mono uppercase text-slate-400 font-bold">Affected Microservices</h3>
              <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
                {services.map((svc) => (
                  <div
                    key={svc.name}
                    className={`p-4 rounded-xl border space-y-2 ${
                      svc.status === 'Failing'
                        ? 'bg-rose-950/20 border-rose-500/40 text-rose-300'
                        : svc.status === 'Degraded'
                        ? 'bg-amber-950/20 border-amber-500/40 text-amber-300'
                        : 'bg-emerald-950/20 border-emerald-500/40 text-emerald-300'
                    }`}
                  >
                    <div className="flex items-center justify-between text-xs font-bold font-mono">
                      <span>{svc.name}</span>
                      <span className="px-2 py-0.5 rounded bg-slate-900 uppercase text-[10px]">
                        {svc.status}
                      </span>
                    </div>
                    <div className="text-xs text-slate-300">{svc.errors}</div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {activeTab === 'terminal' && (
            <div className="h-96">
              <WebTerminal sessionId="incident-coredns" namespace="kube-system" />
            </div>
          )}

          {activeTab === 'metrics' && (
            <div className="glass-panel p-6 space-y-3 font-mono text-xs">
              <div className="flex justify-between text-slate-400">
                <span>PromQL: sum(rate(http_requests_total&#123;status=~&quot;5..&quot;&#125;[1m]))</span>
                <span className="text-rose-400 font-bold">CRITICAL SPIKE</span>
              </div>
              <div className="h-48 bg-slate-950 rounded-xl border border-slate-800 p-4 flex items-end gap-1">
                {[10, 12, 11, 14, 12, 45, 85, 92, 98, 95, 96, 94, 98].map((val, idx) => (
                  <div
                    key={idx}
                    className="flex-1 bg-rose-500 rounded-t transition-all"
                    style={{ height: `${val}%` }}
                  />
                ))}
              </div>
            </div>
          )}
        </div>

        {/* Right Column: Diagnostic Playbook & Post-Mortem Resolution */}
        <div className="lg:col-span-5 glass-panel p-6 space-y-6 flex flex-col justify-between">
          <div className="space-y-4">
            <h3 className="text-sm font-bold text-white uppercase font-mono tracking-wider">
              Diagnostic Resolution Playbook
            </h3>

            <p className="text-xs text-slate-300 leading-relaxed">
              Based on your triage inspection of CoreDNS logs and Prometheus error rates, select the confirmed root cause to execute automated remediation:
            </p>

            <div className="space-y-2.5">
              {[
                { id: 'coredns_configmap', label: 'CoreDNS ConfigMap syntax error causing Pod CrashLoop' },
                { id: 'network_policy', label: 'Egress NetworkPolicy blocking port 53' },
                { id: 'node_oom', label: 'Worker Node OOMKilled evicting CoreDNS pods' },
              ].map((diag) => (
                <button
                  key={diag.id}
                  onClick={() => setSelectedDiagnosis(diag.id)}
                  className={`w-full text-left p-3.5 rounded-xl border text-xs transition-all ${
                    selectedDiagnosis === diag.id
                      ? 'bg-cyan-500/20 border-cyan-500 text-white font-semibold'
                      : 'bg-slate-900 border-slate-800 text-slate-300 hover:bg-slate-850'
                  }`}
                >
                  {diag.label}
                </button>
              ))}
            </div>

            {resolved && (
              <div className="p-4 rounded-xl bg-emerald-500/10 border border-emerald-500/30 text-emerald-300 space-y-1 text-xs">
                <div className="font-bold flex items-center gap-1.5">
                  <CheckCircle2 className="w-4 h-4" />
                  <span>INCIDENT RESOLVED! (+350 XP)</span>
                </div>
                <p className="text-slate-300">
                  CoreDNS rollout completed successfully. Microservice DNS resolution recovered in 4m 18s (MTTR).
                </p>
              </div>
            )}
          </div>

          <div className="pt-4 border-t border-slate-800">
            <button
              onClick={handleResolve}
              disabled={!selectedDiagnosis || resolved}
              className="w-full py-3 rounded-xl bg-rose-600 hover:bg-rose-500 disabled:opacity-50 text-white font-extrabold text-xs uppercase tracking-wider shadow-lg shadow-rose-600/25 transition-all flex items-center justify-center gap-2"
            >
              <ShieldAlert className="w-4 h-4" />
              <span>Apply Fix & Verify Recovery</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
