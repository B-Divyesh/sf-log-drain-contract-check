import { mkdirSync, writeFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { chromium } from 'playwright';
import AxeBuilder from '@axe-core/playwright';

const [base, evidenceDirectory, expectedBuildId] = process.argv.slice(2);
if (!base || !evidenceDirectory) {
  throw new Error('Usage: node scripts/live-audit.mjs <base-url> <evidence-directory> [expected-build-id]');
}

const origin = new URL(base).origin;
const buildId = expectedBuildId ?? execFileSync('git', ['rev-parse', '--short=12', 'HEAD'], { encoding: 'utf8' }).trim();
const routes = [
  { path: '/', status: 200, title: 'Drain Check — check a log drain sample', h1: 'Check a log drain before forwarding', file: 'home' },
  { path: '/?demo=1', status: 200, title: 'Demo — Drain Check', h1: 'Review this drain sample', file: 'demo-query' },
  { path: '/demo', status: 200, title: 'Demo — Drain Check', h1: 'Review this drain sample', file: 'demo' },
  { path: '/privacy', status: 200, title: 'Privacy — Drain Check', h1: 'Privacy for local drain samples', file: 'privacy' },
  { path: '/terms', status: 200, title: 'Terms — Drain Check', h1: 'Terms for using Drain Check', file: 'terms' },
  { path: '/missing', status: 404, title: 'Not found — Drain Check', h1: 'Page not found', file: 'missing' },
];

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

async function storageSnapshot(page) {
  return page.evaluate(async () => {
    const idbFactory = indexedDB;
    const indexedDb = idbFactory.databases ? await idbFactory.databases() : [];
    const cacheNames = await caches.keys();
    const registrations = await navigator.serviceWorker.getRegistrations();
    const opfsEntries = [];
    if (navigator.storage.getDirectory) {
      const directory = await navigator.storage.getDirectory();
      for await (const [name] of directory.entries()) opfsEntries.push(name);
    }
    return {
      cookies: document.cookie,
      local: Object.keys(localStorage),
      session: Object.keys(sessionStorage),
      indexedDb: indexedDb.map(({ name }) => name ?? ''),
      cacheNames,
      registrations: registrations.map(({ scope }) => scope),
      opfsEntries,
    };
  });
}

const emptyStorage = {
  cookies: '', local: [], session: [], indexedDb: [], cacheNames: [], registrations: [], opfsEntries: [],
};

mkdirSync(evidenceDirectory, { recursive: true });
const browser = await chromium.launch({ headless: true });
const audit = { base, routes: [], firstScreen: {}, demo: {}, privacy: {}, headers: {}, sourceReadme: {} };

for (const viewport of [{ width: 1440, height: 900 }, { width: 390, height: 844 }]) {
  const context = await browser.newContext({ viewport });
  for (const route of routes) {
    const page = await context.newPage();
    const errors = [];
    if (route.status === 200) {
      page.on('pageerror', (error) => errors.push(error.message));
      page.on('console', (message) => { if (message.type() === 'error') errors.push(message.text()); });
    }
    const response = await page.goto(`${origin}${route.path}`, { waitUntil: 'networkidle' });
    assert(response?.status() === route.status, `${route.path} returned ${response?.status()}`);
    assert(await page.title() === route.title, `${route.path} title mismatch`);
    assert(await page.getByRole('heading', { name: route.h1 }).count() === 1, `${route.path} h1 mismatch`);
    assert(await page.locator('main').count() === 1, `${route.path} main mismatch`);
    assert(await page.locator('link[rel="canonical"]').getAttribute('href') === `${origin}${route.path}`, `${route.path} canonical mismatch`);
    if (route.path === '/') assert((await page.locator('footer').innerText()).includes(`v0.1.3+${buildId}`), 'Live footer build ID mismatch');
    assert(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), `${route.path} overflows at ${viewport.width}px`);
    const axe = await new AxeBuilder({ page }).analyze();
    assert(axe.violations.filter(({ impact }) => impact === 'serious' || impact === 'critical').length === 0, `${route.path} has serious or critical Axe findings`);
    assert(errors.length === 0, `${route.path} console errors: ${errors.join('; ')}`);
    await page.screenshot({ path: `${evidenceDirectory}/${route.file}-${viewport.width}.png`, fullPage: true });
    audit.routes.push({ path: route.path, viewport: viewport.width, status: response?.status(), title: await page.title(), axeViolations: axe.violations.length, errors });
    await page.close();
  }
  await context.close();
}

const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
const page = await context.newPage();
const requests = [];
const errors = [];
page.on('request', (request) => requests.push(request.url()));
page.on('pageerror', (error) => errors.push(error.message));
page.on('console', (message) => { if (message.type() === 'error') errors.push(message.text()); });
await page.goto(origin, { waitUntil: 'networkidle' });
const primary = await page.getByRole('link', { name: 'Try it with sample data' }).boundingBox();
assert(primary && primary.y + primary.height <= 844, 'The primary demo action is not in the mobile first screen');
assert(await page.evaluate(() => scrollY === 0), 'The first-screen check scrolled');
await page.getByRole('link', { name: 'Try it with sample data' }).click();
assert(page.url() === `${origin}/?demo=1`, 'The landing action did not open ?demo=1');
assert(await page.getByText('Demo — sample data, nothing is saved').count() === 1, 'Demo banner missing');
assert(await page.getByRole('button', { name: 'Reset demo' }).count() === 1, 'Reset demo missing');
assert(await page.getByRole('link', { name: 'View local setup' }).count() === 1, 'Demo exit label is wrong');
assert(await page.locator('.metrics div').nth(2).innerText() === '17\nfield paths', 'Field-path metric is wrong');
assert(await page.locator('.metrics div').nth(3).innerText() === '3\nfindings in 2 field paths', 'Finding metric is wrong');
assert(JSON.stringify(await storageSnapshot(page)) === JSON.stringify(emptyStorage), 'Demo wrote browser storage');
await page.evaluate(() => localStorage.setItem('real:round4', 'keep'));
await page.getByRole('button', { name: 'Reset demo' }).click();
assert(await page.evaluate(() => localStorage.getItem('real:round4')) === 'keep', 'Reset removed real-data storage');
assert(await page.evaluate(() => localStorage.getItem('demo:drain-check')) === null, 'Reset kept demo storage');
await page.getByRole('link', { name: 'View local setup' }).click();
assert(page.url() === `${origin}/`, 'Demo exit did not return to local setup');
assert(await page.evaluate(() => document.activeElement?.tagName) === 'H1', 'Demo exit did not focus the landing heading');
await page.evaluate(() => localStorage.removeItem('real:round4'));
assert(JSON.stringify(await storageSnapshot(page)) === JSON.stringify(emptyStorage), 'Browser storage remained after the demo flow');
assert(requests.every((url) => new URL(url).origin === origin), 'Demo made a third-party request');
assert(errors.length === 0, `Demo console errors: ${errors.join('; ')}`);
audit.firstScreen = { primaryBottom: primary.y + primary.height, viewportHeight: 844 };
audit.demo = { requests, storage: await storageSnapshot(page), exitFocus: await page.evaluate(() => document.activeElement?.tagName), errors };
await context.close();

const home = await fetch(`${origin}/`);
audit.headers.homeStatus = home.status;
audit.headers.csp = home.headers.get('content-security-policy');
audit.headers.hsts = home.headers.get('strict-transport-security');
assert(audit.headers.csp?.includes("frame-ancestors 'none'"), 'Live CSP lacks frame-ancestors response header');
assert(audit.headers.hsts, 'Live HSTS header missing');
const sourceReadme = await fetch('https://raw.githubusercontent.com/B-Divyesh/sf-log-drain-contract-check/main/README.md');
const sourceText = await sourceReadme.text();
assert(sourceReadme.status === 200, 'Live source README is unavailable');
assert(sourceText.includes('For Azure Static Web Apps, keep `staticwebapp.config.json` at the deployment'), 'README Azure deployment guidance missing');
assert(sourceText.includes('On other hosts, recreate its rewrites, 404 response, headers, and cache'), 'README host-specific deployment guidance missing');
audit.sourceReadme = { status: sourceReadme.status, deploymentGuidance: true };

audit.buildId = buildId;
writeFileSync(`${evidenceDirectory}/live-audit.json`, `${JSON.stringify(audit, null, 2)}\n`);
await browser.close();
console.log(JSON.stringify(audit));
