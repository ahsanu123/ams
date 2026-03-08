import { defineConfig, NamingConvention } from 'orval';

export default defineConfig({
  amsApi: {
    output: {
      mode: 'tags-split',
      target: './src/api/v1/generated-api.ts',
      schemas: './src/api/v1/models',
      client: 'react-query',
      baseUrl: 'http://localhost:9090',
      prettier: true,
      clean: true,
      override: {
        useTypeOverInterfaces: true,
        useNamedParameters: true,
        mutator: {
          path: './src/utilities/fetch-mutator.ts',
          name: 'fetchMutator',
        },
        fetch: {
          includeHttpResponseReturnType: false,
          forceSuccessResponse: false,
        },
      },
    },
    input: {
      target: 'http://localhost:9090/api/openapi.json',
    },
  },
});
