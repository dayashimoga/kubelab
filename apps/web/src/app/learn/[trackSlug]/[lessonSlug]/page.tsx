'use client';

import React, { useState } from 'react';
import Link from 'next/link';
import { notFound } from 'next/navigation';
import {
  ChevronRight,
  Clock,
  Zap,
  Terminal,
  Play,
  HelpCircle,
  AlertTriangle,
  ShieldCheck,
  CheckCircle2,
  Sparkles,
  RefreshCw,
  Send,
  X,
  MessageSquare,
} from 'lucide-react';
import { REGISTRY, getQuizForLesson } from '@kubelab/curriculum';

export default function LessonViewPage({
  params,
}: {
  params: { trackSlug: string; lessonSlug: string };
}) {
  const track = REGISTRY.tracks.find((t) => t.slug === params.trackSlug);
  if (!track) notFound();

  let lesson: any = null;
  for (const m of track.modules) {
    const found = m.lessons.find(
      (l) => l.slug === params.lessonSlug || l.id === params.lessonSlug || l.associatedLabId === params.lessonSlug
    );
    if (found) {
      lesson = found;
      break;
    }
  }

  if (!lesson) notFound();

  const quiz = getQuizForLesson(lesson.id);

  // Quiz State
  const [showQuizModal, setShowQuizModal] = useState(false);
  const [currentQIndex, setCurrentQIndex] = useState(0);
  const [selectedOption, setSelectedOption] = useState<number | null>(null);
  const [isAnswerSubmitted, setIsAnswerSubmitted] = useState(false);
  const [quizScore, setQuizScore] = useState(0);
  const [isQuizCompleted, setIsQuizCompleted] = useState(false);

  // AI Tutor State
  const [showTutor, setShowTutor] = useState(false);
  const [tutorMode, setTutorMode] = useState<'explain' | 'socratic' | 'hint' | 'diagnose' | 'review'>('explain');
  const [tutorInput, setTutorInput] = useState('');
  const [tutorMessages, setTutorMessages] = useState<Array<{ sender: 'user' | 'tutor'; text: string }>>([
    {
      sender: 'tutor',
      text: `Hello! I am your AI Socratic Tutor for **${lesson.title}**. Ask me anything about the architecture, YAML manifests, or debugging strategies.`,
    },
  ]);

  const handleTutorSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!tutorInput.trim()) return;

    const userText = tutorInput.trim();
    setTutorInput('');
    setTutorMessages((prev) => [...prev, { sender: 'user', text: userText }]);

    // Pedagogical response
    setTimeout(() => {
      let reply = '';
      if (tutorMode === 'explain') {
        reply = `**Conceptual Breakdown**: In ${lesson.title}, Kubernetes uses declarative reconciliation. The API server stores your desired state, and controllers continuously drive current state to match.`;
      } else if (tutorMode === 'socratic') {
        reply = `**Socratic Query**: To solve this, consider: what happens if the container crashes during startup? Which probe (startup, liveness, readiness) prevents traffic from routing to it?`;
      } else if (tutorMode === 'hint') {
        reply = `**Next Step Hint**: Run \`kubectl describe -f manifest.yaml\` and verify the selector labels match your pod template labels.`;
      } else if (tutorMode === 'diagnose') {
        reply = `**Diagnostic Triage**: Common failure modes in ${lesson.title} include selector mismatches, missing RBAC ClusterRole bindings, and OOMKills.`;
      } else {
        reply = `**Architectural Review**: Excellent progress! Always enforce non-root security contexts, define resource requests/limits, and configure PodDisruptionBudgets.`;
      }
      setTutorMessages((prev) => [...prev, { sender: 'tutor', text: reply }]);
    }, 400);
  };

  const handleOptionSelect = (index: number) => {
    if (isAnswerSubmitted) return;
    setSelectedOption(index);
  };

  const handleQuizSubmit = () => {
    if (selectedOption === null || !quiz) return;
    const q = quiz.questions[currentQIndex];
    const isCorrect = selectedOption === q.correctIndex;
    setIsAnswerSubmitted(true);
    if (isCorrect) setQuizScore((prev) => prev + 100);
  };

  const handleQuizNext = () => {
    if (!quiz) return;
    if (currentQIndex < quiz.questions.length - 1) {
      setCurrentQIndex((prev) => prev + 1);
      setSelectedOption(null);
      setIsAnswerSubmitted(false);
    } else {
      setIsQuizCompleted(true);
    }
  };

  return (
    <div className="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 py-8 space-y-8">
      {/* Breadcrumbs */}
      <nav className="flex items-center gap-2 text-xs text-slate-400 font-mono">
        <Link href="/learn" className="hover:text-cyan-400 transition-colors">
          Tracks
        </Link>
        <ChevronRight className="w-3.5 h-3.5 text-slate-600" />
        <Link href={`/learn/${track.slug}`} className="hover:text-cyan-400 transition-colors">
          {track.title}
        </Link>
        <ChevronRight className="w-3.5 h-3.5 text-slate-600" />
        <span className="text-cyan-400 font-bold truncate">{lesson.title}</span>
      </nav>

      {/* Lesson Header */}
      <div className="p-6 sm:p-8 rounded-2xl bg-slate-900/80 border border-slate-800 space-y-4">
        <div className="flex flex-wrap items-center justify-between gap-4">
          <div className="flex items-center gap-3">
            <span className="px-3 py-1 rounded-full text-[10px] font-mono font-bold uppercase tracking-wider bg-cyan-950/60 text-cyan-400 border border-cyan-800/60">
              {track.title}
            </span>
            <span className="flex items-center gap-1 text-xs text-amber-400 font-mono font-bold bg-amber-950/40 px-2.5 py-1 rounded-md border border-amber-900/40">
              <Zap className="w-3.5 h-3.5" />
              <span>+{lesson.xp} XP</span>
            </span>
          </div>

          <div className="flex items-center gap-2">
            <button
              onClick={() => setShowTutor(!showTutor)}
              className="flex items-center gap-2 px-3.5 py-2 rounded-xl bg-slate-800 hover:bg-slate-700 text-cyan-400 text-xs font-bold transition-colors border border-slate-700"
            >
              <Sparkles className="w-3.5 h-3.5" />
              <span>AI Tutor</span>
            </button>
            <button
              onClick={() => {
                setShowQuizModal(true);
                setCurrentQIndex(0);
                setSelectedOption(null);
                setIsAnswerSubmitted(false);
                setIsQuizCompleted(false);
                setQuizScore(0);
              }}
              className="flex items-center gap-2 px-4 py-2 rounded-xl bg-cyan-500 hover:bg-cyan-400 text-slate-950 text-xs font-bold transition-all shadow-md shadow-cyan-500/20"
            >
              <HelpCircle className="w-3.5 h-3.5" />
              <span>Take Quiz</span>
            </button>
          </div>
        </div>

        <h1 className="text-2xl sm:text-3xl font-extrabold text-white tracking-tight">
          {lesson.title}
        </h1>
        <p className="text-slate-400 text-sm leading-relaxed">{lesson.summary}</p>
      </div>

      {/* Main Content & Markdown View */}
      <div className="p-8 rounded-2xl bg-slate-900/40 border border-slate-800 space-y-6 text-slate-300 text-sm leading-relaxed prose prose-invert max-w-none">
        <div className="whitespace-pre-wrap font-sans">{lesson.contentMarkdown}</div>
      </div>

      {/* Common Mistakes & Guidance */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {lesson.commonMistakes && lesson.commonMistakes.length > 0 && (
          <div className="p-6 rounded-2xl bg-amber-950/20 border border-amber-900/40 space-y-3">
            <div className="flex items-center gap-2 text-amber-400 font-bold text-xs font-mono uppercase tracking-wider">
              <AlertTriangle className="w-4 h-4" />
              <span>Common Production Mistakes</span>
            </div>
            <ul className="space-y-2 text-xs text-slate-300">
              {lesson.commonMistakes.map((m: string, idx: number) => (
                <li key={idx} className="flex items-start gap-2">
                  <span className="text-amber-400 font-bold">•</span>
                  <span>{m}</span>
                </li>
              ))}
            </ul>
          </div>
        )}

        {lesson.productionGuidance && (
          <div className="p-6 rounded-2xl bg-emerald-950/20 border border-emerald-900/40 space-y-3">
            <div className="flex items-center gap-2 text-emerald-400 font-bold text-xs font-mono uppercase tracking-wider">
              <ShieldCheck className="w-4 h-4" />
              <span>Production Hardening & Security</span>
            </div>
            <p className="text-xs text-slate-300 leading-relaxed">{lesson.productionGuidance}</p>
          </div>
        )}
      </div>

      {/* Terminal Lab Callout */}
      {lesson.associatedLabId && (
        <div className="p-6 rounded-2xl bg-gradient-to-r from-cyan-950/40 to-slate-900 border border-cyan-500/30 flex flex-col sm:flex-row sm:items-center justify-between gap-4">
          <div className="space-y-1">
            <div className="flex items-center gap-2 text-cyan-400 text-xs font-mono font-bold uppercase tracking-wider">
              <Terminal className="w-4 h-4" />
              <span>Interactive Hands-On Lab</span>
            </div>
            <div className="text-base font-bold text-white">
              Launch Sandbox: <code className="text-cyan-300 font-mono">{lesson.associatedLabId}</code>
            </div>
            <div className="text-xs text-slate-400">
              Practice this topic in a live isolated disposable Kubernetes terminal.
            </div>
          </div>

          <Link
            href={`/labs/${lesson.associatedLabId}`}
            className="flex items-center justify-center gap-2 px-6 py-3 rounded-xl bg-cyan-500 hover:bg-cyan-400 text-slate-950 text-xs font-bold tracking-wider uppercase transition-all shadow-lg shadow-cyan-500/20 shrink-0"
          >
            <Play className="w-4 h-4 fill-current" />
            <span>Launch Lab</span>
          </Link>
        </div>
      )}

      {/* AI Tutor Sidebar / Sheet */}
      {showTutor && (
        <div className="fixed inset-y-0 right-0 w-full sm:w-96 bg-slate-900 border-l border-slate-800 p-6 z-50 flex flex-col shadow-2xl space-y-4">
          <div className="flex items-center justify-between pb-3 border-b border-slate-800">
            <div className="flex items-center gap-2 text-cyan-400 font-bold text-sm">
              <Sparkles className="w-4 h-4" />
              <span>AI Socratic Tutor</span>
            </div>
            <button
              onClick={() => setShowTutor(false)}
              className="p-1 rounded-lg text-slate-400 hover:text-white"
            >
              <X className="w-4 h-4" />
            </button>
          </div>

          {/* Mode Selector */}
          <div className="grid grid-cols-3 gap-1.5 text-[10px] font-mono font-bold">
            {(['explain', 'socratic', 'hint', 'diagnose', 'review'] as const).map((mode) => (
              <button
                key={mode}
                onClick={() => setTutorMode(mode)}
                className={`px-2 py-1.5 rounded-lg uppercase transition-colors ${
                  tutorMode === mode
                    ? 'bg-cyan-500 text-slate-950'
                    : 'bg-slate-800 text-slate-400 hover:text-white'
                }`}
              >
                {mode}
              </button>
            ))}
          </div>

          {/* Messages */}
          <div className="flex-1 overflow-y-auto space-y-3 pr-1 text-xs">
            {tutorMessages.map((msg, idx) => (
              <div
                key={idx}
                className={`p-3 rounded-xl ${
                  msg.sender === 'user'
                    ? 'bg-cyan-500/20 text-cyan-200 border border-cyan-500/30 ml-4'
                    : 'bg-slate-800/80 text-slate-300 border border-slate-700/60 mr-4'
                }`}
              >
                <div className="font-bold text-[10px] uppercase font-mono mb-1 text-slate-400">
                  {msg.sender === 'user' ? 'You' : `Tutor (${tutorMode})`}
                </div>
                <div>{msg.text}</div>
              </div>
            ))}
          </div>

          {/* Input Form */}
          <form onSubmit={handleTutorSubmit} className="flex gap-2">
            <input
              type="text"
              placeholder={`Ask ${tutorMode} question...`}
              value={tutorInput}
              onChange={(e) => setTutorInput(e.target.value)}
              className="flex-1 px-3 py-2 bg-slate-950 border border-slate-800 rounded-xl text-xs text-white placeholder:text-slate-500 focus:outline-none focus:border-cyan-500"
            />
            <button
              type="submit"
              className="px-3 py-2 bg-cyan-500 text-slate-950 rounded-xl font-bold hover:bg-cyan-400 transition-colors"
            >
              <Send className="w-4 h-4" />
            </button>
          </form>
        </div>
      )}

      {/* Quiz Modal */}
      {showQuizModal && quiz && (
        <div className="fixed inset-0 bg-slate-950/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
          <div className="w-full max-w-xl bg-slate-900 border border-slate-800 rounded-3xl p-6 sm:p-8 space-y-6 shadow-2xl">
            <div className="flex items-center justify-between pb-4 border-b border-slate-800">
              <div className="space-y-1">
                <div className="text-[11px] font-mono font-bold uppercase tracking-wider text-cyan-400">
                  Lesson Quiz • {quiz.title}
                </div>
                <div className="text-xs text-slate-400">
                  Question {currentQIndex + 1} of {quiz.questions.length}
                </div>
              </div>
              <button
                onClick={() => setShowQuizModal(false)}
                className="p-1 text-slate-400 hover:text-white"
              >
                <X className="w-5 h-5" />
              </button>
            </div>

            {!isQuizCompleted ? (
              <div className="space-y-5">
                <div className="text-base font-bold text-white leading-relaxed">
                  {quiz.questions[currentQIndex].prompt}
                </div>

                <div className="space-y-2.5">
                  {quiz.questions[currentQIndex].options.map((opt, idx) => {
                    const isSelected = selectedOption === idx;
                    const isCorrect = idx === quiz.questions[currentQIndex].correctIndex;

                    let btnClass = 'border-slate-800 bg-slate-950/60 text-slate-300 hover:border-slate-700';
                    if (isAnswerSubmitted) {
                      if (isCorrect) {
                        btnClass = 'border-emerald-500 bg-emerald-950/30 text-emerald-300';
                      } else if (isSelected) {
                        btnClass = 'border-red-500 bg-red-950/30 text-red-300';
                      }
                    } else if (isSelected) {
                      btnClass = 'border-cyan-500 bg-cyan-950/30 text-cyan-300';
                    }

                    return (
                      <button
                        key={idx}
                        onClick={() => handleOptionSelect(idx)}
                        className={`w-full text-left p-3.5 rounded-xl border text-xs font-medium transition-all flex items-center justify-between ${btnClass}`}
                      >
                        <span>{opt}</span>
                        <span className="w-6 h-6 rounded-lg bg-slate-800/80 flex items-center justify-center text-[10px] font-mono font-bold text-slate-400">
                          {String.fromCharCode(65 + idx)}
                        </span>
                      </button>
                    );
                  })}
                </div>

                {isAnswerSubmitted && (
                  <div className="p-4 rounded-xl bg-slate-950 border border-slate-800 space-y-1 text-xs">
                    <div className="font-bold text-cyan-400">Explanation</div>
                    <div className="text-slate-300">{quiz.questions[currentQIndex].explanation}</div>
                  </div>
                )}

                <div className="flex justify-end gap-3 pt-2">
                  {!isAnswerSubmitted ? (
                    <button
                      onClick={handleQuizSubmit}
                      disabled={selectedOption === null}
                      className="px-6 py-2.5 rounded-xl bg-cyan-500 hover:bg-cyan-400 disabled:opacity-50 text-slate-950 font-bold text-xs uppercase tracking-wider transition-all"
                    >
                      Submit Answer
                    </button>
                  ) : (
                    <button
                      onClick={handleQuizNext}
                      className="px-6 py-2.5 rounded-xl bg-cyan-500 hover:bg-cyan-400 text-slate-950 font-bold text-xs uppercase tracking-wider transition-all"
                    >
                      {currentQIndex < quiz.questions.length - 1 ? 'Next Question' : 'View Results'}
                    </button>
                  )}
                </div>
              </div>
            ) : (
              <div className="text-center py-6 space-y-4">
                <CheckCircle2 className="w-12 h-12 text-emerald-400 mx-auto" />
                <h3 className="text-xl font-bold text-white">Quiz Completed!</h3>
                <p className="text-sm text-slate-400">
                  You scored <span className="text-cyan-400 font-bold font-mono">{quizScore} XP</span> out of{' '}
                  {quiz.questions.length * 100} XP
                </p>
                <div className="flex justify-center gap-3 pt-4">
                  <button
                    onClick={() => {
                      setCurrentQIndex(0);
                      setSelectedOption(null);
                      setIsAnswerSubmitted(false);
                      setIsQuizCompleted(false);
                      setQuizScore(0);
                    }}
                    className="px-4 py-2 rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-bold"
                  >
                    Retry Quiz
                  </button>
                  <button
                    onClick={() => setShowQuizModal(false)}
                    className="px-6 py-2 rounded-xl bg-cyan-500 hover:bg-cyan-400 text-slate-950 text-xs font-bold uppercase tracking-wider"
                  >
                    Continue
                  </button>
                </div>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
