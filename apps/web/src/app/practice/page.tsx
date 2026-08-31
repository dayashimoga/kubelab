'use client';

import React, { useState } from 'react';
import {
  Flame,
  Award,
  Clock,
  Zap,
  CheckCircle2,
  AlertCircle,
  Play,
  RotateCcw,
  Sparkles,
  Check,
} from 'lucide-react';

const SAMPLE_QUESTIONS = [
  {
    id: 'q1',
    topic: 'Kubernetes Workloads',
    difficulty: 'Medium',
    points: 50,
    prompt: 'During a Deployment RollingUpdate with replicas=4, maxSurge=1, and maxUnavailable=0, what is the maximum number of Pods that can be running concurrently?',
    options: [
      { id: 'a', text: '4 Pods' },
      { id: 'b', text: '5 Pods' },
      { id: 'c', text: '6 Pods' },
      { id: 'd', text: '8 Pods' },
    ],
    correct: 'b',
    explanation: 'replicas (4) + maxSurge (1) = 5 total Pods maximum during the transition.',
  },
  {
    id: 'q2',
    topic: 'Linux & OCI',
    difficulty: 'Easy',
    points: 50,
    prompt: 'Which Linux kernel feature is responsible for isolating process tree visibility (PID namespace)?',
    options: [
      { id: 'a', text: 'cgroups v2' },
      { id: 'b', text: 'PID namespaces' },
      { id: 'c', text: 'Seccomp-BPF' },
      { id: 'd', text: 'AppArmor' },
    ],
    correct: 'b',
    explanation: 'PID namespaces provide process isolation, giving the root container process PID 1 inside the container.',
  },
  {
    id: 'q3',
    topic: 'Service Mesh',
    difficulty: 'Hard',
    points: 75,
    prompt: 'In Istio, which CRD defines the Layer 7 destination subsets and load balancing / mTLS policies?',
    options: [
      { id: 'a', text: 'VirtualService' },
      { id: 'b', text: 'DestinationRule' },
      { id: 'c', text: 'Gateway' },
      { id: 'd', text: 'EnvoyFilter' },
    ],
    correct: 'b',
    explanation: 'DestinationRule defines subsets (versions) and traffic policies (mTLS, circuit breaker, load balancer algorithm) applied after VirtualService routing.',
  },
];

export default function PracticeQuizPage() {
  const [currentIdx, setCurrentIdx] = useState(0);
  const [selectedAnswers, setSelectedAnswers] = useState<Record<string, string>>({});
  const [submitted, setSubmitted] = useState(false);

  const currentQ = SAMPLE_QUESTIONS[currentIdx];

  const handleSelect = (optId: string) => {
    if (submitted) return;
    setSelectedAnswers({ ...selectedAnswers, [currentQ.id]: optId });
  };

  const calculateScore = () => {
    let score = 0;
    SAMPLE_QUESTIONS.forEach((q) => {
      if (selectedAnswers[q.id] === q.correct) {
        score += q.points;
      }
    });
    return score;
  };

  return (
    <div className="container-max py-10 space-y-8 max-w-4xl">
      {/* Header */}
      <div className="space-y-3">
        <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-amber-500/10 border border-amber-500/20 text-amber-400 text-xs font-semibold uppercase tracking-wider">
          <Flame className="w-3.5 h-3.5" />
          <span>Adaptive Practice & Trivia</span>
        </div>
        <h1 className="text-3xl font-extrabold text-white">
          Cloud-Native <span className="gradient-text-cyan">Skill Practice Drills</span>
        </h1>
        <p className="text-slate-300 text-sm">
          Test your conceptual understanding with scenario-based cloud-native questions.
        </p>
      </div>

      {/* Quiz Workspace Card */}
      <div className="glass-panel p-6 sm:p-8 space-y-6">
        {/* Progress Bar & Header */}
        <div className="flex items-center justify-between border-b border-slate-800 pb-4">
          <div className="flex items-center gap-3">
            <span className="px-2.5 py-1 rounded bg-indigo-500/20 text-indigo-300 text-xs font-mono font-bold">
              QUESTION {currentIdx + 1} OF {SAMPLE_QUESTIONS.length}
            </span>
            <span className="text-xs text-slate-400 font-mono">{currentQ.topic}</span>
          </div>

          <div className="flex items-center gap-2 text-xs font-mono text-amber-400">
            <Zap className="w-3.5 h-3.5" />
            <span>{currentQ.points} XP</span>
          </div>
        </div>

        {/* Prompt */}
        <div className="space-y-4">
          <h2 className="text-lg font-bold text-white leading-relaxed">{currentQ.prompt}</h2>

          {/* Options */}
          <div className="space-y-3">
            {currentQ.options.map((opt) => {
              const isSelected = selectedAnswers[currentQ.id] === opt.id;
              const isCorrect = opt.id === currentQ.correct;

              let style = 'bg-slate-900/80 border-slate-800 text-slate-300 hover:bg-slate-800';
              if (submitted) {
                if (isCorrect) {
                  style = 'bg-emerald-500/10 border-emerald-500 text-emerald-300 font-bold';
                } else if (isSelected && !isCorrect) {
                  style = 'bg-rose-500/10 border-rose-500 text-rose-300';
                }
              } else if (isSelected) {
                style = 'bg-indigo-600/30 border-indigo-500 text-white font-semibold';
              }

              return (
                <button
                  key={opt.id}
                  onClick={() => handleSelect(opt.id)}
                  className={`w-full text-left p-4 rounded-xl border text-sm transition-all flex items-center justify-between ${style}`}
                >
                  <div className="flex items-center gap-3">
                    <span className="w-6 h-6 rounded-lg bg-slate-800 flex items-center justify-center font-mono font-bold text-xs">
                      {opt.id.toUpperCase()}
                    </span>
                    <span>{opt.text}</span>
                  </div>

                  {submitted && isCorrect && <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />}
                </button>
              );
            })}
          </div>

          {/* Explanation Banner */}
          {submitted && (
            <div className="p-4 rounded-xl bg-slate-900 border border-slate-800 text-xs text-slate-300 space-y-1">
              <span className="font-bold text-cyan-400 font-mono uppercase">Explanation:</span>
              <p>{currentQ.explanation}</p>
            </div>
          )}
        </div>

        {/* Navigation & Submit Action */}
        <div className="flex items-center justify-between pt-4 border-t border-slate-800">
          <button
            disabled={currentIdx === 0}
            onClick={() => setCurrentIdx(currentIdx - 1)}
            className="px-4 py-2 rounded-lg bg-slate-900 border border-slate-800 hover:bg-slate-800 disabled:opacity-30 text-xs font-semibold text-slate-300 transition-colors"
          >
            Previous
          </button>

          <div className="flex items-center gap-3">
            {!submitted ? (
              <button
                onClick={() => setSubmitted(true)}
                className="px-5 py-2.5 rounded-xl bg-cyan-500 hover:bg-cyan-400 text-slate-950 font-extrabold text-xs shadow-lg shadow-cyan-500/20 transition-all"
              >
                Submit Practice Drill
              </button>
            ) : (
              <div className="flex items-center gap-3">
                <span className="text-xs font-mono text-emerald-400 font-bold">
                  Score: {calculateScore()} / 175 XP Earned!
                </span>
                <button
                  onClick={() => {
                    setSubmitted(false);
                    setSelectedAnswers({});
                    setCurrentIdx(0);
                  }}
                  className="px-4 py-2 rounded-lg bg-slate-800 hover:bg-slate-700 text-white text-xs font-semibold"
                >
                  Retry Drill
                </button>
              </div>
            )}

            {currentIdx < SAMPLE_QUESTIONS.length - 1 && (
              <button
                onClick={() => setCurrentIdx(currentIdx + 1)}
                className="px-4 py-2 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white text-xs font-semibold transition-colors"
              >
                Next
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
