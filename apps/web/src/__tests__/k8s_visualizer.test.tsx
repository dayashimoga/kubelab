import { describe, it, expect } from 'vitest';
import React from 'react';
import { render, screen } from '@testing-library/react';
import { K8sVisualizer, ResourceItem } from '../components/K8sVisualizer';

describe('K8sVisualizer Component', () => {
  it('renders K8s workload topology elements cleanly', () => {
    const mockResources: ResourceItem[] = [
      { kind: 'Deployment', name: 'order-api', namespace: 'default', status: 'Ready', age: '5m', details: '1/1 replicas' },
      { kind: 'Service', name: 'order-svc', namespace: 'default', status: 'Active', age: '5m', details: 'ClusterIP 10.96.0.1' },
      { kind: 'Pod', name: 'order-api-7b8c', namespace: 'default', status: 'Running', age: '5m', details: 'Ready 1/1' },
    ];

    render(<K8sVisualizer namespace="default" resources={mockResources} />);
    expect(screen.getByText('Kubernetes Cluster Visualizer')).toBeDefined();
    expect(screen.getByText('order-api')).toBeDefined();
  });
});
