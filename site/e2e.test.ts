import { afterAll, beforeAll, describe, expect, it } from 'vitest';
import { chromium, type Browser } from 'playwright';
import AxeBuilder from '@axe-core/playwright';
import { createServer, type ViteDevServer } from 'vite';

let server: ViteDevServer;
let browser: Browser;
const base = 'http://127.0.0.1:4179';

beforeAll(async () => {
  server = await createServer({ server: { host: '127.0.0.1', port: 4179 }, logLevel: 'error' });
  await server.listen();
  browser = await chromium.launch({ headless: true });
}, 30_000);
afterAll(async () => { await browser?.close(); await server?.close(); });

describe('site sandbox', () => {
  it('@claim:sample-demo opens the report in one click', async () => {
    const page = await browser.newPage();
    await page.goto(base);
    await page.getByRole('link', { name: 'Try it with sample data' }).click();
    expect(page.url()).toBe(`${base}/demo`);
    expect(await page.getByRole('heading', { name: 'Review this drain sample' }).count()).toBe(1);
    expect(await page.getByText('Demo — sample data, nothing is saved').count()).toBe(1);
  });
  it('@claim:local-only makes no third-party requests in the demo', async () => {
    const page = await browser.newPage(); const requests: string[] = [];
    page.on('request', request => requests.push(request.url()));
    await page.goto(`${base}/demo`);
    expect(requests.every(url => new URL(url).origin === base)).toBe(true);
  });
  it('@claim:discard-default exposes the documented safe default', async () => {
    const page = await browser.newPage();
    await page.goto(base);
    expect(await page.getByText('Discards bodies by default.').count()).toBe(1);
  });
  it('has no serious accessibility violations on mobile demo', async () => {
    const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
    const page = await context.newPage();
    await page.goto(`${base}/demo`);
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations).toEqual([]);
    await context.close();
  });
});
