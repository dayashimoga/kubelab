import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { MonacoYamlEditor } from '../components/MonacoYamlEditor';

describe('MonacoYamlEditor Component', () => {
  const initialYaml = 'apiVersion: v1\nkind: Pod\nmetadata:\n  name: test-pod';

  it('renders YAML editor header, schema badge, and content', () => {
    const onApply = vi.fn();
    render(<MonacoYamlEditor initialYaml={initialYaml} onApply={onApply} />);

    expect(screen.getByText('manifest.yaml')).toBeInTheDocument();
    expect(screen.getByText('k8s-schema-v1.30')).toBeInTheDocument();
    expect(screen.getByDisplayValue(initialYaml)).toBeInTheDocument();
  });

  it('triggers onApply callback when kubectl apply button is clicked', async () => {
    const onApply = vi.fn();
    render(<MonacoYamlEditor initialYaml={initialYaml} onApply={onApply} />);

    const applyButton = screen.getByRole('button', { name: /kubectl apply/i });
    fireEvent.click(applyButton);

    expect(onApply).toHaveBeenCalledWith(initialYaml);
  });
});
