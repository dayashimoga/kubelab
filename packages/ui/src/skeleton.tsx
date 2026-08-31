import React from 'react';

export interface SkeletonProps extends React.HTMLAttributes<HTMLDivElement> {
  variant?: 'text' | 'circular' | 'rectangular';
  width?: string | number;
  height?: string | number;
}

export const Skeleton: React.FC<SkeletonProps> = ({
  variant = 'text',
  width,
  height,
  className = '',
  style,
  ...props
}) => {
  const variantStyles = {
    text: 'h-4 rounded',
    circular: 'rounded-full',
    rectangular: 'rounded-xl',
  };

  const customStyle: React.CSSProperties = {
    width,
    height,
    ...style,
  };

  return (
    <div
      className={`animate-pulse bg-slate-800/80 ${variantStyles[variant]} ${className}`}
      style={customStyle}
      {...props}
    />
  );
};
