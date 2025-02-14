import { defineConfig, defaultPlugins } from '@hey-api/openapi-ts';

export default defineConfig({
  client: '@hey-api/client-fetch',
  input: 'http://localhost:5136/swagger/v1/swagger.json',
  output: {
    format: 'prettier',
    path: './app/api/generated/',
  },
  plugins: [
    ...defaultPlugins,
    '@hey-api/schemas',
    {
      name: '@hey-api/transformers',
      dates: true,
    },
    {
      name: '@hey-api/typescript'
    },
    {
      name: '@hey-api/sdk',
      transformer: true,
    },
    {
      asClass: true,
      name: '@hey-api/sdk'
    }
  ],
});
