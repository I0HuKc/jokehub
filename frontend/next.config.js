/** @type {import('next').NextConfig} */

module.exports = {
  async rewrites() {
    return [
      {
        source: "/api/:path*",
        destination: "http://jokehub.fun/:path*",
      },
    ];
  },

  reactStrictMode: true,

  env: {
    RECAPTCHA_SECRET: "6LcxSEEgAAAAAArBH21uRsSL8yS9N7MwyGg1AHdd",
  },

  
};
