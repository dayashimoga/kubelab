'use client';

import React, { useState, useEffect } from 'react';
import { FileText, ChevronRight } from 'lucide-react';
import { api, DocSection } from '@/lib/api';

/** Default fallback docs for when API is unreachable */
const DEFAULT_DOC_SECTIONS: DocSection[] = [
  {
    id: 'arch',
    title: 'Architecture & Design',
    content: `## Architecture Overview\n\nKubeLab is built with a high-concurrency Rust/Axum API Gateway, disposable Podman/kind Kubernetes clusters, and state-based deterministic grading.\n\n### Subsystems:\n- **API Gateway**: Handles WebSocket terminal streaming and REST routes.\n- **Validation Engine**: Evaluates live Kubernetes API object states.\n- **Orchestrator**: Manages ephemeral sandbox namespaces with strict NetworkPolicies.`,
  },
  {
    id: 'sdk',
    title: 'Lab Authoring SDK',
    content: `## Authoring Custom Labs\n\nEvery lab is defined in a declarative \`lab.yaml\`:\n\`\`\`yaml\nid: "k8s-pod-basics"\ntitle: "Create Your First Pod"\ndifficulty: "beginner"\nduration_minutes: 15\ntrack: "kubernetes"\nscenario: "Deploy an nginx web server listening on port 80."\ntasks:\n  - id: "task-1"\n    points: 100\n    validation:\n      type: "k8s_resource"\n      resource: "pods"\n      name: "web-server"\n      assertions:\n        - field: "status.phase"\n          operator: "equals"\n          expected: "Running"\n\`\`\``,
  },
  {
    id: 'security',
    title: 'Security Architecture',
    content: `## Security Hardening\n\n- **Non-Root Execution**: Container shells run as UID 10001 with dropped capabilities (\`CAP_DROP ALL\`).\n- **Network Isolation**: Dedicated Kubernetes NetworkPolicies block inter-tenant traffic.\n- **Read-Only Rootfs**: Sandboxes use ephemeral tmpfs mounts for temporary files.`,
  },
  {
    id: 'quickstart',
    title: 'Zero-Host Setup & CLI',
    content: `## Getting Started\n\nKubeLab requires zero host development tools. Everything runs in Podman:\n\`\`\`bash\n# Start everything in one command\n./scripts/up.ps1   # Windows\n./scripts/up.sh    # Linux/macOS\n\n# Run authoritative production gate\n./scripts/validate-production.ps1\n\`\`\``,
  },
];

export default function DocsPage() {
  const [sections, setSections] = useState<DocSection[]>(DEFAULT_DOC_SECTIONS);
  const [activeSection, setActiveSection] = useState(DEFAULT_DOC_SECTIONS[0]);

  useEffect(() => {
    const load = async () => {
      try {
        const data = await api.getDocSections();
        if (data.length > 0) {
          setSections(data);
          setActiveSection(data[0]);
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
          <FileText className="w-3.5 h-3.5" />
          <span>Technical Documentation</span>
        </div>
        <h1 className="text-3xl sm:text-4xl font-extrabold text-white">
          KubeLab Platform <span className="gradient-text-cyan">Documentation</span>
        </h1>
        <p className="text-slate-300 text-sm max-w-2xl">
          Complete engineering reference, lab authoring guide, security specifications, and architecture ADRs.
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-12 gap-8">
        {/* Left Navigation */}
        <div className="lg:col-span-4 space-y-2">
          {sections.map((sec) => (
            <button
              key={sec.id}
              onClick={() => setActiveSection(sec)}
              className={`w-full text-left p-4 rounded-xl border text-sm font-semibold transition-all flex items-center justify-between ${
                activeSection.id === sec.id
                  ? 'bg-cyan-500/10 border-cyan-500 text-cyan-300'
                  : 'bg-slate-900 border-slate-800 text-slate-400 hover:text-white hover:bg-slate-850'
              }`}
            >
              <span>{sec.title}</span>
              <ChevronRight className="w-4 h-4" />
            </button>
          ))}
        </div>

        {/* Right Content Pane */}
        <div className="lg:col-span-8 glass-panel p-8 space-y-6">
          <h2 className="text-2xl font-bold text-white">{activeSection.title}</h2>
          <div className="prose prose-invert max-w-none text-slate-300 text-sm leading-relaxed whitespace-pre-wrap font-sans">
            {activeSection.content}
          </div>
        </div>
      </div>
    </div>
  );
}
