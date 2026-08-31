import './globals.css';
import React from 'react';
import Link from 'next/link';
import {
  BookOpen,
  FlaskConical,
  Flame,
  AlertTriangle,
  GitFork,
  Award,
  FileText,
  Boxes,
  Zap,
} from 'lucide-react';
import { AuthProvider } from '@/lib/auth-context';

export const metadata = {
  title: 'KubeLab — Cloud-Native Learning & Lab Platform',
  description:
    'Production-grade open-source platform for learning, practicing, and proving cloud-native skills with real clusters, deterministic grading, and incident simulation.',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  const navLinks = [
    { href: '/learn', label: 'Learn', icon: BookOpen },
    { href: '/labs', label: 'Labs', icon: FlaskConical },
    { href: '/practice', label: 'Practice', icon: Flame },
    { href: '/incidents', label: 'Incidents', icon: AlertTriangle, badge: 'SEV' },
    { href: '/skills', label: 'Skill Tree', icon: GitFork },
    { href: '/progress', label: 'Progress', icon: Zap },
    { href: '/certifications', label: 'Certifications', icon: Award },
    { href: '/docs', label: 'Docs', icon: FileText },
  ];

  return (
    <html lang="en">
      <body className="flex flex-col min-h-screen bg-[#0a0e17] text-slate-100 selection:bg-cyan-500 selection:text-black">
        <AuthProvider>
          {/* Navigation Bar */}
          <header className="sticky top-0 z-40 bg-[#0f172a]/90 backdrop-blur-md border-b border-slate-800/80 px-4 lg:px-8 py-3">
            <div className="max-w-[1500px] mx-auto flex items-center justify-between">
              {/* Logo */}
              <Link href="/" className="flex items-center gap-3 group focus:outline-none">
                <div className="w-10 h-10 rounded-xl bg-gradient-to-tr from-cyan-500 via-indigo-500 to-violet-500 p-0.5 shadow-lg shadow-cyan-500/20 group-hover:scale-105 transition-transform">
                  <div className="w-full h-full bg-slate-950 rounded-[10px] flex items-center justify-center">
                    <Boxes className="w-5 h-5 text-cyan-400" />
                  </div>
                </div>
                <div className="flex flex-col">
                  <span className="font-extrabold text-lg tracking-tight flex items-center gap-1.5">
                    <span className="text-white">KUBE</span>
                    <span className="gradient-text-cyan">LAB</span>
                  </span>
                  <span className="text-[10px] font-mono text-slate-400 tracking-wider -mt-1">
                    CLOUD-NATIVE OS
                  </span>
                </div>
              </Link>

              {/* Desktop Navigation Links */}
              <nav className="hidden lg:flex items-center gap-1 xl:gap-2">
                {navLinks.map((link) => {
                  const Icon = link.icon;
                  return (
                    <Link
                      key={link.href}
                      href={link.href}
                      className="flex items-center gap-2 px-3.5 py-2 rounded-lg text-sm font-medium text-slate-300 hover:text-white hover:bg-slate-800/60 transition-all focus:outline-none focus:ring-1 focus:ring-cyan-500 relative group"
                    >
                      <Icon className="w-4 h-4 text-slate-400 group-hover:text-cyan-400 transition-colors" />
                      <span>{link.label}</span>
                      {link.badge && (
                        <span className="px-1.5 py-0.2 text-[10px] font-bold font-mono bg-rose-500/20 text-rose-400 border border-rose-500/30 rounded-md animate-pulse">
                          {link.badge}
                        </span>
                      )}
                    </Link>
                  );
                })}
              </nav>

              {/* Cluster Status & Profile / Auth Actions */}
              <div className="flex items-center gap-3">
                <div className="hidden sm:flex items-center gap-2 px-3 py-1.5 rounded-full bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 text-xs font-mono">
                  <span className="w-2 h-2 rounded-full bg-emerald-400 animate-ping" />
                  <span>k8s: ready (v1.30)</span>
                </div>

                <Link
                  href="/login"
                  className="px-3.5 py-1.5 rounded-xl bg-slate-900 border border-slate-800 hover:border-cyan-500/40 text-slate-300 hover:text-white text-xs font-semibold transition-all"
                >
                  Sign In
                </Link>

                <Link
                  href="/progress"
                  className="flex items-center gap-2.5 px-3 py-1.5 rounded-xl bg-slate-900 border border-slate-800 hover:border-slate-700 transition-colors"
                >
                  <div className="w-7 h-7 rounded-lg bg-indigo-600 flex items-center justify-center font-bold text-xs text-white">
                    L1
                  </div>
                  <div className="hidden md:flex flex-col text-left">
                    <span className="text-xs font-semibold text-slate-200">Learner</span>
                    <span className="text-[10px] text-cyan-400 font-mono">0 XP</span>
                  </div>
                </Link>
              </div>
            </div>
          </header>

          {/* Main Content Area */}
          <main className="flex-1 w-full">{children}</main>

          {/* Footer */}
          <footer className="bg-slate-950 border-t border-slate-800/80 py-8 px-4 text-center text-xs text-slate-500">
            <div className="max-w-7xl mx-auto flex flex-col sm:flex-row items-center justify-between gap-4">
              <div className="flex items-center gap-2">
                <Boxes className="w-4 h-4 text-cyan-500" />
                <span>KubeLab Engine v1.0.0 — 100% Real Live Infrastructure</span>
              </div>
              <div className="flex gap-6 font-medium">
                <Link href="/docs" className="hover:text-slate-300">
                  Documentation
                </Link>
                <Link href="/security" className="hover:text-slate-300">
                  Security Policy
                </Link>
                <Link href="/certifications" className="hover:text-slate-300">
                  Verify Credentials
                </Link>
                <a
                  href="https://github.com/kubelab/kubelab"
                  target="_blank"
                  rel="noreferrer"
                  className="hover:text-slate-300"
                >
                  GitHub (Apache 2.0)
                </a>
              </div>
            </div>
          </footer>
        </AuthProvider>
      </body>
    </html>
  );
}
