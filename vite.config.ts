import { defineConfig } from 'vite';
import { execFileSync } from 'node:child_process';

function buildId() {
  if (process.env.GITHUB_SHA) return process.env.GITHUB_SHA.slice(0, 12);
  try {
    return execFileSync('git', ['rev-parse', '--short=12', 'HEAD'], { encoding: 'utf8' }).trim();
  } catch {
    return 'source';
  }
}

export default defineConfig({
  publicDir: 'site/public',
  define: { __BUILD_ID__: JSON.stringify(buildId()) },
  build: {
    target: 'es2022',
    rollupOptions: {
      input: {
        main: new URL('index.html', import.meta.url).pathname,
        notFound: new URL('404.html', import.meta.url).pathname
      }
    }
  }
});
