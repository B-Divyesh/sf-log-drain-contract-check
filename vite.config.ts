import { defineConfig } from 'vite';
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';

function buildId() {
  if (process.env.GITHUB_SHA) return process.env.GITHUB_SHA.slice(0, 12);
  try {
    return execFileSync('git', ['rev-parse', '--short=12', 'HEAD'], { encoding: 'utf8' }).trim();
  } catch {
    return 'source';
  }
}

function releaseVersion() {
  const cargo = readFileSync(new URL('Cargo.toml', import.meta.url), 'utf8');
  const version = cargo.match(/^version = "([^"]+)"$/m)?.[1];
  if (!version) throw new Error('Could not read the Cargo package version.');
  return version;
}

export default defineConfig({
  publicDir: 'site/public',
  define: {
    __BUILD_ID__: JSON.stringify(buildId()),
    __SITE_VERSION__: JSON.stringify(releaseVersion())
  },
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
