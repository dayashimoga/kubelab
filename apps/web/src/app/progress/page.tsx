'use client';

import React, { useState, useEffect } from 'react';
import {
  Zap,
  Flame,
  Award,
  CheckCircle2,
  Boxes,
  ShieldCheck,
  Terminal,
  Trophy,
} from 'lucide-react';
import { api, UserProgressState } from '@/lib/api';

const DEFAULT_PROGRESS: UserProgressState = {
  user_id: 'test-learner-1',
  total_xp: 0,
  level: 1,
  current_streak_days: 0,
  longest_streak_days: 0,
  last_active_date: '2026-08-31',
  completed_lesson_ids: [],
  completed_lab_ids: [],
  unlocked_badges: [],
  skills: {
    'Linux Primitives': 1,
    'Container Runtimes': 1,
    'Kubernetes Workloads': 1,
    'CNI & CoreDNS': 1,
    'GitOps & Argo CD': 0,
    'Service Mesh & Envoy': 0,
    'Prometheus & OTel': 0,
    'Incident Triage': 0,
  },
};

export default function ProgressDashboardPage() {
  const [progress, setProgress] = useState<UserProgressState>(DEFAULT_PROGRESS);

  useEffect(() => {
    api
      .getProgress()
      .then((data) => {
        if (data) setProgress(data);
      })
      .catch(() => {});
  }, []);

  const badges = [
    {
      name: 'Pod Pilot',
      icon: Boxes,
      desc: 'Created your first live Kubernetes Pod in an ephemeral sandbox',
      unlocked: progress.unlocked_badges.some((b) => b.slug === 'first-pod'),
      date: 'Earned',
    },
    {
      name: '5-Day Streak',
      icon: Flame,
      desc: 'Logged in and completed lessons 5 consecutive days',
      unlocked: progress.current_streak_days >= 5,
      date: '5-Day Milestone',
    },
    {
      name: 'GitOps Sentinel',
      icon: ShieldCheck,
      desc: 'Configured automated Argo CD drift self-healing',
      unlocked: progress.completed_lab_ids.includes('gitops-argocd-drift'),
      date: 'Advanced Challenge',
    },
    {
      name: 'Chaos Commander',
      icon: Award,
      desc: 'Resolved a SEV-1 live production incident within SLA',
      unlocked: progress.completed_lab_ids.includes('incident-coredns-failure'),
      date: 'Incident Simulator',
    },
    {
      name: 'Service Mesh Master',
      icon: Trophy,
      desc: 'Completed 10 Istio & Envoy traffic routing challenges',
      unlocked: progress.completed_lab_ids.includes('mesh-istio-canary'),
      date: 'Mastery',
    },
  ];

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-10 space-y-10">
      {/* Header */}
      <div className="space-y-3 pb-6 border-b border-slate-800">
        <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-cyan-500/10 border border-cyan-500/20 text-cyan-400 text-xs font-semibold uppercase tracking-wider">
          <Zap className="w-3.5 h-3.5" />
          <span>Learner Profile & Stats</span>
        </div>
        <h1 className="text-3xl sm:text-4xl font-extrabold text-white tracking-tight">
          Progress & Achievement Hub
        </h1>
        <p className="text-slate-400 text-sm max-w-2xl">
          Track your real accumulated XP, streak history, skill mastery levels, and verifiable credentials.
        </p>
      </div>

      {/* Top Stats Overview */}
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-6">
        <div className="p-6 rounded-2xl bg-slate-900/70 border border-slate-800 space-y-2">
          <span className="text-xs text-slate-400 font-mono uppercase">Cumulative Experience</span>
          <div className="text-3xl font-extrabold text-white flex items-center gap-2">
            <Zap className="w-6 h-6 text-amber-400" />
            <span>{progress.total_xp} XP</span>
          </div>
          <p className="text-xs text-slate-400 font-mono">Level {progress.level} Cloud-Native Engineer</p>
        </div>

        <div className="p-6 rounded-2xl bg-slate-900/70 border border-slate-800 space-y-2">
          <span className="text-xs text-slate-400 font-mono uppercase">Current Learning Streak</span>
          <div className="text-3xl font-extrabold text-white flex items-center gap-2">
            <Flame className="w-6 h-6 text-rose-500 animate-pulse" />
            <span>{progress.current_streak_days} Days</span>
          </div>
          <p className="text-xs text-slate-400 font-mono">Longest streak: {progress.longest_streak_days} days</p>
        </div>

        <div className="p-6 rounded-2xl bg-slate-900/70 border border-slate-800 space-y-2">
          <span className="text-xs text-slate-400 font-mono uppercase">Completed Sandboxes</span>
          <div className="text-3xl font-extrabold text-white flex items-center gap-2">
            <CheckCircle2 className="w-6 h-6 text-emerald-400" />
            <span>{progress.completed_lab_ids.length} Labs</span>
          </div>
          <p className="text-xs text-slate-400 font-mono">{progress.completed_lesson_ids.length} Lessons Finished</p>
        </div>
      </div>

      {/* Skill Mastery Grid */}
      <div className="space-y-4">
        <h2 className="text-xl font-bold text-white">Skill Competencies</h2>
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          {Object.entries(progress.skills).map(([skill, lvl]) => (
            <div key={skill} className="p-4 rounded-xl bg-slate-900 border border-slate-800 space-y-2">
              <div className="flex justify-between text-xs">
                <span className="font-semibold text-slate-200">{skill}</span>
                <span className="font-mono text-cyan-400">Tier {lvl}/5</span>
              </div>
              <div className="w-full bg-slate-950 rounded-full h-2 overflow-hidden border border-slate-800">
                <div
                  className="bg-gradient-to-r from-cyan-500 to-indigo-500 h-full rounded-full transition-all duration-500"
                  style={{ width: `${Math.min(100, lvl * 20)}%` }}
                />
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Unlocked Badges */}
      <div className="space-y-4">
        <h2 className="text-xl font-bold text-white">Digital Badges & Credentials</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {badges.map((badge, idx) => {
            const Icon = badge.icon;
            return (
              <div
                key={idx}
                className={`p-6 rounded-2xl border transition-all ${
                  badge.unlocked
                    ? 'bg-slate-900/80 border-cyan-500/30'
                    : 'bg-slate-950/40 border-slate-800/60 opacity-60'
                }`}
              >
                <div className="flex items-center gap-4">
                  <div
                    className={`w-12 h-12 rounded-xl flex items-center justify-center ${
                      badge.unlocked
                        ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/40 shadow-lg shadow-cyan-500/10'
                        : 'bg-slate-900 text-slate-600 border border-slate-800'
                    }`}
                  >
                    <Icon className="w-6 h-6" />
                  </div>
                  <div>
                    <h3 className="font-bold text-sm text-white">{badge.name}</h3>
                    <span className="text-[11px] text-cyan-400 font-mono">{badge.date}</span>
                  </div>
                </div>
                <p className="text-xs text-slate-400 mt-3 leading-relaxed">{badge.desc}</p>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}
