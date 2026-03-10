import { defineConfig } from "vitest/config";
import react from '@vitejs/plugin-react'
import { resolve } from "node:path";

export default defineConfig({
  plugins: [
    react(),
  ],
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src')
    }
  },
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: './test/setup-test.ts',
    reporters: [
      [
        "default",
        {
          summary: false, // removes summary, matches "basic"
        },
      ],
    ],
    silent: false,
  }
});

