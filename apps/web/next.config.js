const withPlugins = require('next-compose-plugins')
const withBundleAnalyzer = require('@next/bundle-analyzer')
const withTM = require('next-transpile-modules')
const withSvgr = require('next-svgr')

/**
 * @type {import('next').NextConfig}
 */
const nextConfig = withPlugins(
  [
    withBundleAnalyzer({ enabled: process.env.ANALYZE === 'true' }),
    withTM(['gsap']),
    withSvgr({
      removeViewbox: false
    })
  ],
  {
    reactStrictMode: true,
    images: {
      domains: ['pbs.twimg.com', 'abs.twimg.com']
    }
  }
)

module.exports = nextConfig
