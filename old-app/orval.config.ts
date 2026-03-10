import { defineConfig } from 'orval';

export default defineConfig({
  petstore: {
    output: {
      mode: 'tags-split',
      target: './src/api/v1/generated-api.ts',
      schemas: './src/api/v1/models',
      client: 'react-query',
    },
    input: {
      target: 'http://localhost:9090/api/openapi.json',
    },
  },
});
