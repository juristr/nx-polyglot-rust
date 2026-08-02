import { defineConfig } from 'vite';
import { fileURLToPath } from 'node:url';

import { tanstackStart } from '@tanstack/react-start/plugin/vite';

import viteReact from '@vitejs/plugin-react';

const appRoot = fileURLToPath(new URL('.', import.meta.url));

const config = defineConfig({
  root: appRoot,
  resolve: { tsconfigPaths: true },
  plugins: [tanstackStart(), viteReact()],
});

export default config;
