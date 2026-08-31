import React from 'react';

export interface CardProps extends React.HTMLAttributes<HTMLDivElement> {
  variant?: 'default' | 'glass' | 'interactive';
  header?: React.ReactNode;
  footer?: React.ReactNode;
}

export const Card: React.FC<CardProps> = ({
  children,
  variant = 'default',
  header,
  footer,
  className = '',
  ...props
}) => {
  const variantStyles = {
    default: 'bg-slate-900 border border-slate-800 shadow-xl',
    glass: 'bg-slate-900/60 backdrop-blur-xl border border-slate-800/80 shadow-2xl',
    interactive:
      'bg-slate-900/80 hover:bg-slate-800/90 border border-slate-800 hover:border-indigo-500/50 shadow-xl transition-all duration-300 hover:shadow-indigo-500/10 cursor-pointer',
  };

  return (
    <div
      className={`rounded-xl overflow-hidden text-slate-100 ${variantStyles[variant]} ${className}`}
      {...props}
    >
      {header && <div className="px-6 py-4 border-b border-slate-800/80">{header}</div>}
      <div className="p-6">{children}</div>
      {footer && <div className="px-6 py-4 border-t border-slate-800/80 bg-slate-950/40">{footer}</div>}
    </div>
  );
};
