import React from 'react';
import Link from 'next/link';
import { notFound } from 'next/navigation';
import {
  Terminal,
  Boxes,
  Database,
  Network,
  Package,
  Server,
  ShieldCheck,
  GitBranch,
  Layers,
  Activity,
  Wrench,
  Gauge,
  Cpu,
  AlertTriangle,
  Award,
  BookOpen,
  ArrowRight,
  Clock,
  Zap,
  CheckCircle2,
  ChevronRight,
  Play,
} from 'lucide-react';
import { REGISTRY } from '@kubelab/curriculum';

const ICON_MAP: Record<string, any> = {
  Terminal,
  Boxes,
  Database,
  Network,
  Package,
  Server,
  ShieldCheck,
  GitBranch,
  Layers,
  Activity,
  Wrench,
  Gauge,
  Cpu,
  AlertTriangle,
  Award,
};

export default function TrackDetailPage({ params }: { params: { trackSlug: string } }) {
  const track = REGISTRY.tracks.find((t) => t.slug === params.trackSlug);

  if (!track) {
    notFound();
  }

  const Icon = ICON_MAP[track.icon] || Boxes;
  const firstLesson = track.modules[0]?.lessons[0];

  return (
    <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-10 space-y-10">
      {/* Breadcrumbs */}
      <nav className="flex items-center gap-2 text-xs text-slate-400 font-mono">
        <Link href="/learn" className="hover:text-cyan-400 transition-colors">
          Tracks
        </Link>
        <ChevronRight className="w-3.5 h-3.5 text-slate-600" />
        <span className="text-cyan-400 font-bold">{track.title}</span>
      </nav>

      {/* Hero Card */}
      <div className="p-8 rounded-3xl bg-slate-900/80 border border-slate-800 space-y-6 relative overflow-hidden">
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-6 relative z-10">
          <div className="flex items-start gap-5">
            <div className="w-16 h-16 rounded-2xl bg-cyan-500/10 border border-cyan-500/20 flex items-center justify-center text-cyan-400 shrink-0">
              <Icon className="w-8 h-8" />
            </div>
            <div className="space-y-2">
              <div className="flex items-center gap-3">
                <span className="px-3 py-1 rounded-full text-[10px] font-mono font-bold uppercase tracking-wider bg-slate-800 text-cyan-400 border border-slate-700">
                  {track.difficulty}
                </span>
                <span className="text-xs text-slate-400 font-mono">Track #{track.order}</span>
              </div>
              <h1 className="text-2xl sm:text-3xl font-extrabold text-white tracking-tight">
                {track.title}
              </h1>
              <p className="text-slate-400 text-sm max-w-2xl leading-relaxed">
                {track.description}
              </p>
            </div>
          </div>

          <div className="flex flex-col sm:flex-row md:flex-col items-start md:items-end gap-3 shrink-0">
            <div className="flex items-center gap-4 text-xs font-mono text-slate-300">
              <span className="flex items-center gap-1.5 bg-slate-800/80 px-3 py-1.5 rounded-lg border border-slate-700">
                <Clock className="w-4 h-4 text-cyan-400" />
                <span>{track.totalLessons} Lessons</span>
              </span>
              <span className="flex items-center gap-1.5 bg-slate-800/80 px-3 py-1.5 rounded-lg border border-slate-700 text-amber-400 font-bold">
                <Zap className="w-4 h-4" />
                <span>{track.totalXp} XP</span>
              </span>
            </div>

            {firstLesson && (
              <Link
                href={`/learn/${track.slug}/${firstLesson.slug}`}
                className="flex items-center gap-2 px-6 py-3 rounded-xl bg-cyan-500 hover:bg-cyan-400 text-slate-950 font-bold text-xs tracking-wider uppercase transition-all shadow-lg shadow-cyan-500/20"
              >
                <Play className="w-4 h-4 fill-current" />
                <span>Start First Lesson</span>
              </Link>
            )}
          </div>
        </div>
      </div>

      {/* Modules & Lessons Accordion */}
      <div className="space-y-8">
        <h2 className="text-xl font-bold text-white tracking-tight flex items-center gap-2">
          <BookOpen className="w-5 h-5 text-cyan-400" />
          <span>Curriculum Modules & Syllabus</span>
        </h2>

        <div className="space-y-6">
          {track.modules.map((mod) => (
            <div
              key={mod.id}
              className="p-6 rounded-2xl bg-slate-900/60 border border-slate-800 space-y-4"
            >
              <div className="space-y-1 pb-3 border-b border-slate-800/80">
                <div className="text-[11px] font-mono font-bold uppercase tracking-wider text-cyan-400">
                  Module {mod.order}
                </div>
                <h3 className="text-lg font-bold text-white">{mod.title}</h3>
                <p className="text-xs text-slate-400">{mod.description}</p>
              </div>

              {/* Lessons List */}
              <div className="grid grid-cols-1 gap-3">
                {mod.lessons.map((lesson) => (
                  <Link
                    key={lesson.id}
                    href={`/learn/${track.slug}/${lesson.slug}`}
                    className="flex items-center justify-between p-4 rounded-xl bg-slate-950/60 border border-slate-800/80 hover:border-cyan-500/40 hover:bg-slate-900/60 transition-all group"
                  >
                    <div className="flex items-center gap-4">
                      <div className="w-8 h-8 rounded-lg bg-slate-800 border border-slate-700 flex items-center justify-center text-xs font-mono font-bold text-slate-300 group-hover:text-cyan-400 group-hover:border-cyan-500/40 transition-colors">
                        {lesson.order}
                      </div>
                      <div className="space-y-1">
                        <div className="text-sm font-semibold text-white group-hover:text-cyan-300 transition-colors">
                          {lesson.title}
                        </div>
                        <div className="flex items-center gap-3 text-[11px] text-slate-500 font-mono">
                          <span>{lesson.durationMinutes} mins</span>
                          <span>•</span>
                          <span className="text-amber-400 font-medium">+{lesson.xp} XP</span>
                          <span>•</span>
                          <span className="text-cyan-400/80 bg-cyan-950/40 px-2 py-0.5 rounded border border-cyan-900/40">
                            Lab: {lesson.associatedLabId}
                          </span>
                        </div>
                      </div>
                    </div>

                    <div className="flex items-center gap-2 text-cyan-400 opacity-0 group-hover:opacity-100 transition-opacity">
                      <span className="text-xs font-semibold">Start</span>
                      <ArrowRight className="w-4 h-4" />
                    </div>
                  </Link>
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
