import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { WebTerminal } from '../components/Terminal';

// Mock WebSocket
class MockWebSocket {
  static OPEN = 1;
  readyState = MockWebSocket.OPEN;
  send = vi.fn();
  close = vi.fn();
  onopen = vi.fn();
  onmessage = vi.fn();
  onclose = vi.fn();
  onerror = vi.fn();
}

(global as any).WebSocket = MockWebSocket;

describe('WebTerminal Component', () => {
  it('renders terminal shell banner and namespace badge', () => {
    render(<WebTerminal sessionId="test-session-001" namespace="lab-test-namespace" />);

    expect(screen.getByText(/sandbox-shell \(lab-test-namespace\)/i)).toBeInTheDocument();
    expect(screen.getByText(/learner@kubelab:~\$/i)).toBeInTheDocument();
  });

  it('allows command input and submission', () => {
    render(<WebTerminal sessionId="test-session-001" namespace="lab-test-namespace" />);

    const input = screen.getByRole('textbox');
    fireEvent.change(input, { target: { value: 'kubectl get pods' } });
    expect(input).toHaveValue('kubectl get pods');

    fireEvent.submit(input.closest('form')!);
    expect(screen.getByText(/learner@kubelab:~\$ kubectl get pods/i)).toBeInTheDocument();
  });
});
