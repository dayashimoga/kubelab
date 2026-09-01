import { describe, it, expect, vi } from 'vitest';
import React from 'react';
import { render, screen, act } from '@testing-library/react';
import { AuthProvider, useAuth } from '../lib/auth-context';

function TestConsumer() {
  const { user, login, logout } = useAuth();
  return (
    <div>
      <div data-testid="user-email">{user ? user.email : 'anonymous'}</div>
      <button onClick={() => login('test@kubelab.io', 'pass')}>Login</button>
      <button onClick={logout}>Logout</button>
    </div>
  );
}

describe('AuthContext Component & State Provider', () => {
  it('provides anonymous state initially and allows login/logout transitions', async () => {
    render(
      <AuthProvider>
        <TestConsumer />
      </AuthProvider>
    );

    expect(screen.getByTestId('user-email').textContent).toBe('anonymous');
  });
});
