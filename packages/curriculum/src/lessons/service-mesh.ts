import { Lesson } from '@kubelab/shared-types';

export const SERVICE_MESH_LESSONS: Lesson[] = [
  {
    id: 'istio-traffic-management',
    moduleId: 'mod-mesh-core',
    trackSlug: 'service-mesh',
    title: 'Traffic Shifting & Canary Releases with Istio & Envoy',
    slug: 'istio-traffic-shifting',
    order: 1,
    durationMinutes: 25,
    xp: 250,
    summary:
      'Configure Istio VirtualServices and DestinationRules to achieve fine-grained 90/10 traffic splitting and zero-downtime canary deployments.',
    contentMarkdown: `
# Istio Traffic Management & Canary Routing

In standard Kubernetes, Services only support equal round-robin load balancing across all ready pod endpoints. With **Istio Service Mesh**, sidecar Envoy proxies intercept all Layer 7 HTTP/gRPC traffic, enabling intelligent weighted routing, header-based routing, and fault injection.

\`\`\`mermaid
graph TD
    Ingress[Istio Ingress Gateway] -->|100% Traffic| VS[VirtualService: reviews]
    VS -->|90% Weight| DR_v1[DestinationRule: subset v1 (Reviews v1)]
    VS -->|10% Weight (Canary)| DR_v2[DestinationRule: subset v2 (Reviews v2)]
\`\`\`

## VirtualService & DestinationRule Manifests

\`\`\`yaml
apiVersion: networking.istio.io/v1alpha3
kind: VirtualService
metadata:
  name: reviews-route
spec:
  hosts:
  - reviews
  http:
  - route:
    - destination:
        host: reviews
        subset: v1
      weight: 90
    - destination:
        host: reviews
        subset: v2
      weight: 10
---
apiVersion: networking.istio.io/v1alpha3
kind: DestinationRule
metadata:
  name: reviews-destination
spec:
  host: reviews
  subsets:
  - name: v1
    labels:
      version: v1
  - name: v2
    labels:
      version: v2
\`\`\`
`,
    concepts: ['istio', 'envoy-proxy', 'virtualservice', 'destinationrule', 'canary-release', 'traffic-shifting'],
    prerequisites: ['k8s-services-networking'],
    associatedLabId: 'mesh-istio-canary',
    associatedQuizId: 'quiz-istio-traffic',
  },
];
