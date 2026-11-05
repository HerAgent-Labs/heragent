export default () => ({
  app: {
    name: process.env.APP_NAME || 'HerAgent API',
    version: process.env.APP_VERSION || '0.1.0',
    environment: process.env.NODE_ENV || 'development',
    port: Number(process.env.PORT || 4000),
  },

  cors: {
    origin: process.env.CORS_ORIGIN || 'http://localhost:3000',
  },

  stellar: {
    network: process.env.STELLAR_NETWORK || 'testnet',
  },
});