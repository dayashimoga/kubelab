/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  output: 'standalone',
  transpilePackages: [
    '@kubelab/shared-types',
    '@kubelab/ui',
    '@kubelab/curriculum',
    '@kubelab/lab-sdk',
  ],
};

module.exports = nextConfig;
