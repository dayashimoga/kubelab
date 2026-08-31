'use client';

import React, { createContext, useContext, useEffect, useState } from 'react';
import { api, User } from './api';

interface AuthContextType {
  user: User | null;
  token: string | null;
  loading: boolean;
  login: (email: string, pass: string) => Promise<void>;
  register: (email: string, name: string, pass: string, role?: string) => Promise<void>;
  logout: () => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [token, setToken] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const savedToken = localStorage.getItem('kubelab_jwt_token');
    if (savedToken) {
      setToken(savedToken);
      api
        .getMe()
        .then((u) => setUser(u))
        .catch(() => {
          localStorage.removeItem('kubelab_jwt_token');
          setToken(null);
        })
        .finally(() => setLoading(false));
    } else {
      setLoading(false);
    }
  }, []);

  const login = async (email: string, pass: string) => {
    const res = await api.login(email, pass);
    localStorage.setItem('kubelab_jwt_token', res.tokens.access_token);
    setToken(res.tokens.access_token);
    setUser(res.user);
  };

  const register = async (email: string, name: string, pass: string, role?: string) => {
    const res = await api.register(email, name, pass, role);
    localStorage.setItem('kubelab_jwt_token', res.tokens.access_token);
    setToken(res.tokens.access_token);
    setUser(res.user);
  };

  const logout = () => {
    localStorage.removeItem('kubelab_jwt_token');
    setToken(null);
    setUser(null);
  };

  return (
    <AuthContext.Provider value={{ user, token, loading, login, register, logout }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};
