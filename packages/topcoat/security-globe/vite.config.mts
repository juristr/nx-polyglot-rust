import { defineConfig } from 'vite';
import dts from 'vite-plugin-dts';
import * as path from 'path';
export default defineConfig(() => ({
  root: import.meta.dirname,
  cacheDir: '../../../node_modules/.vite/packages/topcoat/security-globe',
  plugins: [
    dts({
      entryRoot: 'src',
      tsconfigPath: path.join(import.meta.dirname, 'tsconfig.lib.json'),
    }),
  ],
  // Uncomment this if you are using workers.
  // worker: {
  //  plugins: [],
  // },
  // Configuration for building your library.
  // See: https://vite.dev/guide/build.html#library-mode
  build: {
    emptyOutDir: true,
    transformMixedEsModules: true,
    lib: {
      entry: 'src/index.ts',
      name: 'SecurityGlobe',
      fileName: 'security-globe',
      formats: ['es' as const],
    },
    rolldownOptions: { external: [] },
    outDir: '../../../apps/topcoat-security/assets',
    reportCompressedSize: true,
    commonjsOptions: { transformMixedEsModules: true },
  },
  test: {
    name: 'security-globe',
    watch: false,
    globals: true,
    environment: 'jsdom',
    include: ['src/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
    reporters: ['default'],
    coverage: {
      reportsDirectory: './test-output/vitest/coverage',
      provider: 'v8' as const,
    },
  },
}));
