/** @type {import('next').NextConfig} */
module.exports = {
  reactStrictMode: true, // Recommended for the `pages` directory, default in `app`.
  swcMinify: true,
  experimental: {
    appDir: true,
    transpilePackages: ["ui"],
  },
};
