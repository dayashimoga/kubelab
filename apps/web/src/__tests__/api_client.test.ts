import { describe, it, expect, vi, beforeEach } from 'vitest';
import { api } from '../lib/api';

describe('ApiClient Unit Tests', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
    localStorage.clear();
  });

  it('injects Bearer authorization header when JWT token exists in localStorage', async () => {
    localStorage.setItem('kubelab_jwt_token', 'mocked_jwt_token_value');

    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ id: 'usr-1', email: 'test@kubelab.io', role: 'learner' }),
    });
    global.fetch = fetchMock;

    await api.getMe();

    expect(fetchMock).toHaveBeenCalledTimes(1);
    const headers = fetchMock.mock.calls[0][1].headers;
    expect(headers['Authorization']).toBe('Bearer mocked_jwt_token_value');
  });

  it('performs registration POST request with JSON payload', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({
        user: { id: 'usr-2', email: 'new@kubelab.io', name: 'New User', role: 'learner' },
        tokens: { access_token: 'new_token', token_type: 'Bearer', expires_in: 86400 },
      }),
    });
    global.fetch = fetchMock;

    const res = await api.register('new@kubelab.io', 'New User', 'Password123!', 'learner');

    expect(res.user.email).toBe('new@kubelab.io');
    expect(res.tokens.access_token).toBe('new_token');
    expect(fetchMock).toHaveBeenCalledWith(
      expect.stringContaining('/v1/auth/register'),
      expect.objectContaining({
        method: 'POST',
        body: JSON.stringify({
          email: 'new@kubelab.io',
          name: 'New User',
          password: 'Password123!',
          role: 'learner',
        }),
      })
    );
  });
});
