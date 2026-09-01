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
  it('renders terminal shell banner, namespace badge, and actions', () => {
    render(<WebTerminal sessionId="test-session-001" namespace="lab-test-namespace" />);

    expect(screen.getByText(/sandbox-shell \(lab-test-namespace\)/i)).toBeInTheDocument();
    expect(screen.getByTitle('Clear Terminal')).toBeInTheDocument();
  });

  it('allows clicking clear terminal button without crashing', () => {
    render(<WebTerminal sessionId="test-session-001" namespace="lab-test-namespace" />);

    const clearButton = screen.getByTitle('Clear Terminal');
    expect(clearButton).toBeInTheDocument();
    fireEvent.click(clearButton);
  });
});
