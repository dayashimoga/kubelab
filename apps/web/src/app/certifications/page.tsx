'use client';

import React from 'react';
import { Award, ShieldCheck, CheckCircle2, Clock, Play, Zap, FileCheck } from 'lucide-react';

const CERTS = [
  {
    id: 'kcpe',
    title: 'Kubernetes Certified Platform Engineer (KCPE)',
    difficulty: 'Advanced',
    duration: '120 mins',
    questions: '15 Real Cluster Tasks',
    status: 'Available',
    desc: 'Demonstrate proficiency in Podman, Kubernetes cluster management, CNI networking, RBAC policies, and live troubleshooting.',
  },
  {
    id: 'kcse',
    title: 'Cloud-Native Security & GitOps Specialist (KCSE)',
    difficulty: 'Expert',
    duration: '90 mins',
    questions: '10 Hands-on Scenarios',
    status: 'Available',
    desc: 'Prove mastery of Argo CD declarative rollbacks, Pod Security Standards, NetworkPolicies, and secret encryption.',
  },
  {
    id: 'kcoe',
    title: 'Cloud-Native Observability & SRE Lead (KCOE)',
    difficulty: 'Expert',
    duration: '90 mins',
    questions: '8 Outage Simulations',
    status: 'Coming Soon',
    desc: 'Verify end-to-end distributed tracing with OpenTelemetry, Prometheus alerting, and SEV-1 incident triage.',
  },
];

export default function CertificationsPage() {
  return (
    <div className="container-max py-10 space-y-10">
      {/* Header */}
      <div className="space-y-3">
        <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-cyan-500/10 border border-cyan-500/20 text-cyan-400 text-xs font-semibold uppercase tracking-wider">
          <Award className="w-3.5 h-3.5" />
          <span>Verifiable Credentials</span>
        </div>
        <h1 className="text-3xl sm:text-4xl font-extrabold text-white">
          Practical Hands-On <span className="gradient-text-cyan">Certifications</span>
        </h1>
        <p className="text-slate-300 text-sm max-w-2xl">
          Earn machine-verifiable digital certifications by solving live cluster tasks in timed exam sandboxes.
        </p>
      </div>

      {/* Certifications Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {CERTS.map((cert) => (
          <div
            key={cert.id}
            className="glass-panel p-6 space-y-5 flex flex-col justify-between border-slate-800 hover:border-cyan-500/30 transition-all"
          >
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <div className="w-12 h-12 rounded-xl bg-slate-800 border border-slate-700 flex items-center justify-center text-cyan-400">
                  <ShieldCheck className="w-6 h-6" />
                </div>
                <span className="px-2.5 py-0.5 rounded-full bg-slate-800 border border-slate-700 text-slate-300 text-[10px] font-mono uppercase font-bold">
                  {cert.difficulty}
                </span>
              </div>

              <h3 className="text-lg font-bold text-white leading-snug">{cert.title}</h3>
              <p className="text-xs text-slate-300 leading-relaxed">{cert.desc}</p>
            </div>

            <div className="space-y-4 pt-4 border-t border-slate-800/80">
              <div className="flex items-center justify-between text-xs text-slate-400 font-mono">
                <span className="flex items-center gap-1">
                  <Clock className="w-3.5 h-3.5 text-slate-500" />
                  <span>{cert.duration}</span>
                </span>
                <span>{cert.questions}</span>
              </div>

              <button
                disabled={cert.status !== 'Available'}
                className="w-full py-2.5 rounded-xl bg-cyan-500 hover:bg-cyan-400 disabled:opacity-40 text-slate-950 font-bold text-xs shadow-md shadow-cyan-500/20 transition-all flex items-center justify-center gap-2"
              >
                <Play className="w-3.5 h-3.5 fill-current" />
                <span>{cert.status === 'Available' ? 'Start Exam Sandbox' : 'Coming Soon'}</span>
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
