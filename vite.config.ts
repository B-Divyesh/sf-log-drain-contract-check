import { defineConfig } from 'vite';

export default defineConfig({
  publicDir: 'site/public',
  build: { target: 'es2022' }
});
