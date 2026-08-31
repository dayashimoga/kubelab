import { Lesson } from '@kubelab/shared-types';

export const GITOPS_LESSONS: Lesson[] = [
  {
    id: 'gitops-argocd-fundamentals',
    moduleId: 'mod-gitops-core',
    trackSlug: 'gitops',
    title: 'Argo CD Architecture: Declarative Continuous Delivery',
    slug: 'argocd-architecture',
    order: 1,
    durationMinutes: 20,
    xp: 200,
    summary:
      'Learn the GitOps operating model, how the Argo CD Application Controller detects live cluster drift, and automated sync policies.',
    contentMarkdown: `
# GitOps Principles & Argo CD Core Architecture

**GitOps** is a paradigm where the entire system desired state is version-controlled in Git, and an automated continuous delivery engine continuously reconciles the actual state of the cluster with the desired state in Git.

\`\`\`mermaid
graph LR
    Dev[Developer] -->|git push manifest| GitRepo[(Git Repository)]
    ArgoCtrl[Argo CD Application Controller] -->|Watches| GitRepo
    ArgoCtrl -->|Compares & Reconciles| K8s[Kubernetes Cluster API]
    K8s -->|Live State Drift Detected| OutOfSync[Status: OutOfSync]
    ArgoCtrl -->|Auto-Sync Apply| K8s
\`\`\`

## Core Argo CD CRD: \`Application\`

\`\`\`yaml
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: guestbook-prod
  namespace: argocd
spec:
  project: default
  source:
    repoURL: https://github.com/argoproj/argocd-example-apps.git
    targetRevision: HEAD
    path: guestbook
  destination:
    server: https://kubernetes.default.svc
    namespace: guestbook
  syncPolicy:
    automated:
      prune: true
      selfHeal: true
    syncOptions:
    - CreateNamespace=true
\`\`\`

## Key Sync Options
- **\`prune\`**: Automatically deletes resources in Kubernetes if their manifests were deleted in Git.
- **\`selfHeal\`**: Automatically overwrites manual cluster edits (e.g. \`kubectl edit\`) back to the state declared in Git.
`,
    concepts: ['gitops', 'argocd', 'application-controller', 'drift-detection', 'self-heal', 'sync-policy'],
    prerequisites: ['k8s-deployments-rollouts'],
    associatedLabId: 'gitops-argocd-drift',
    associatedQuizId: 'quiz-gitops-argocd',
  },
];
