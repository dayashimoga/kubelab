import { Lesson } from '@kubelab/shared-types';

export const INCIDENT_LESSONS: Lesson[] = [
  {
    id: 'incident-dns-crashloop-triage',
    moduleId: 'mod-incidents-core',
    trackSlug: 'incidents',
    title: 'Incident Response: CoreDNS Outage & Cascading Failures',
    slug: 'incident-coredns-outage',
    order: 1,
    durationMinutes: 30,
    xp: 350,
    summary:
      'Diagnose and resolve a SEV-1 production incident where CoreDNS pod eviction causes cascading timeouts across all frontend and payment services.',
    contentMarkdown: `
# SEV-1 Incident Playbook: CoreDNS Outage & Resolution

When CoreDNS experiences failures or misconfiguration, all service discovery within the Kubernetes cluster fails simultaneously, leading to widespread 503 errors and connection timeouts.

## Triaging Step-by-Step

1. **Verify DNS Pod Health**:
   \`\`\`bash
   kubectl get pods -n kube-system -l k8s-app=kube-dns
   \`\`\`
2. **Inspect CoreDNS Logs**:
   \`\`\`bash
   kubectl logs -n kube-system -l k8s-app=kube-dns --tail=100
   \`\`\`
3. **Execute in-cluster DNS Query Test**:
   \`\`\`bash
   kubectl run dnstest --rm -it --image=busybox:1.28 -- nslookup kubernetes.default
   \`\`\`
`,
    concepts: ['incident-response', 'coredns', 'cascading-failure', 'dns-troubleshooting', 'sev-1-playbook'],
    prerequisites: ['k8s-services-networking'],
    associatedLabId: 'incident-coredns-failure',
    associatedQuizId: 'quiz-incident-dns',
  },
];
