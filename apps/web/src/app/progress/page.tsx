'use client';

import React from 'react';
import {
  Zap,
  Flame,
  Award,
  Calendar,
  CheckCircle2,
  Boxes,
  ShieldCheck,
  Terminal,
  Trophy,
} from 'lucide-react';

export default function ProgressDashboardPage() {
  const badges = [
    { name: 'Pod Pilot', icon: Boxes, desc: 'Created your first live Kubernetes Pod', unlocked: true, date: 'Aug 29, 2026' },
    { name: '5-Day Streak', icon: Flame, desc: 'Logged in and completed lessons 5 consecutive days', unlocked: true, date: 'Aug 31, 2026' },
    { name: 'GitOps Sentinel', icon: ShieldCheck, desc: 'Configured automated Argo CD drift self-healing', unlocked: true, date: 'Aug 30, 2026' },
    { name: 'Chaos Commander', icon: Award, desc: 'Resolved a SEV-1 live production incident within SLA', unlocked: false, date: 'Locked' },
    { name: 'Service Mesh Master', icon: Trophy, desc: 'Completed 10 Istio & Envoy routing challenges', unlocked: false, date: 'Locked' },
  ];

  return (
    <div className="container-max py-10 space-y-10">
      {/* Header */}
      <div className="space-y-3">
        <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-cyan-500/10 border border-cyan-500/20 text-cyan-400 text-xs font-semibold uppercase tracking-wider">
          <Zap className="w-3.5 h-3.5" />
          <span>Learner Profile & Stats</span>
        </div>
        <h1 className="text-3xl sm:text-4xl font-extrabold text-white">
          Progress & <span className="gradient-text-cyan">Achievement Hub</span>
        </h1>
        <p className="text-slate-300 text-sm max-w-2xl">
          Track your XP, streak history, skill levels, and unlockable cloud-native digital credentials.
        </p>
      </div>

      {/* Top Stats Overview */}
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-6">
        <div className="glass-panel p-6 space-y-2">
          <span className="text-xs text-slate-400 font-mono uppercase">Cumulative Experience</span>
          <div className="text-3xl font-extrabold text-white flex items-center gap-2">
            <Zap className="w-6 h-6 text-amber-400" />
            <span>1,250 XP</span>
          </div>
          <p className="text-xs text-slate-400 font-mono">Level 3 Cloud-Native Practitioner</p>
        </div>

        <div className="glass-panel p-6 space-y-2">
          <span className="text-xs text-slate-400 font-mono uppercase">Current Learning Streak</span>
          <div className="text-3xl font-extrabold text-white flex items-center gap-2">
            <Flame className="w-6 h-6 text-rose-500 animate-pulse" />
            <span>5 Days</span>
          </div>
          <p className="text-xs text-slate-400 font-mono">Longest streak: 12 days</p>
        </div>

        <div className="glass-panel p-6 space-y-2">
          <span className="text-xs text-slate-400 font-mono uppercase">Labs Completed</span>
          <div className="text-3xl font-extrabold text-white flex items-center gap-2">
            <CheckCircle2 className="w-6 h-6 text-emerald-400" />
            <span>12 / 20</span>
          </div>
          <p className="text-xs text-slate-400 font-mono">60% completion rate</p>
        </div>
      </div>

      {/* Activity Heatmap Grid */}
      <div className="glass-panel p-6 sm:p-8 space-y-4">
        <div className="flex items-center justify-between">
          <span className="text-xs font-mono font-bold uppercase text-slate-300 flex items-center gap-2">
            <Calendar className="w-4 h-4 text-cyan-400" />
            <span>Learning Activity Heatmap</span>
          </span>
          <span className="text-xs text-slate-400 font-mono">Past 12 Weeks</span>
        </div>

        {/* Heatmap blocks */}
        <div className="grid grid-cols-12 gap-2 pt-2">
          {Array.from({ length: 84 }).map((_, idx) => {
            const intensity = idx % 5 === 0 ? 'bg-cyan-500' : idx % 3 === 0 ? 'bg-cyan-600/60' : idx % 2 === 0 ? 'bg-indigo-900/40' : 'bg-slate-900';
            return (
              <div
                key={idx}
                className={`h-4 rounded-sm ${intensity} hover:scale-125 transition-transform cursor-pointer`}
                title={`Activity recorded`}
              />
            );
          })}
        </div>
      </div>

      {/* Badges Section */}
      <div className="space-y-6">
        <h2 className="text-2xl font-bold text-white">Earned Badges & Milestones</h2>
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
          {badges.map((b, idx) => {
            const Icon = b.icon;
            return (
              <div
                key={idx}
                className={`p-6 rounded-2xl border flex items-start gap-4 transition-all ${
                  b.unlocked
                    ? 'glass-panel border-cyan-500/30'
                    : 'bg-slate-900/40 border-slate-800/80 opacity-50'
                }`}
              >
                <div
                  className={`w-12 h-12 rounded-xl flex items-center justify-center shrink-0 ${
                    b.unlocked
                      ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30'
                      : 'bg-slate-800 text-slate-600'
                  }`}
                >
                  <Icon className="w-6 h-6" />
                </div>

                <div className="space-y-1">
                  <h3 className="text-sm font-bold text-white">{b.name}</h3>
                  <p className="text-xs text-slate-400">{b.desc}</p>
                  <span className="text-[10px] font-mono text-cyan-400 block pt-1">{b.date}</span>
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}
