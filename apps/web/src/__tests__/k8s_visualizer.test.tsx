import { describe, it, expect } from 'vitest';
import React from 'react';
import { render, screen } from '@testing-library/react';
import { K8sVisualizer } from '../components/K8sVisualizer';

describe('K8sVisualizer Component', () => {
  it('renders K8s workload topology elements cleanly', () => {
    const mockResources = [
      { kind: 'Deployment', name: 'order-api', status: 'Ready' },
      { kind: 'Service', name: 'order-svc', status: 'Active' },
      { kind: 'Pod', name: 'order-api-7b8c', status: 'Running' },
    ];

    render(<K8sVisualizer resources={mockResources} />);
    expect(screen.getByText('LIVE TOPOLOGY DAG')).toBeDefined();
  });
});
