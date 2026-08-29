import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { afterAll, beforeAll, describe, expect, it } from 'vitest';
import { chromium, type Browser } from 'playwright';
import AxeBuilder from '@axe-core/playwright';
import { createServer, type ViteDevServer } from 'vite';

let server: ViteDevServer;
let browser: Browser;
const base = 'http://127.0.0.1:4179';
const routes = ['/', '/demo', '/privacy', '/terms', '/missing'];
// Each published Cargo-backed claim must pass from a cold clone. Rust compilation
// legitimately takes longer than Vitest's 5s default, while the behavior checks
// below still run unchanged after compilation completes.
const CARGO_CLAIM_TIMEOUT_MS = 60_000;

beforeAll(async () => {
  server = await createServer({ server: { host: '127.0.0.1', port: 4179 }, logLevel: 'error' });
  await server.listen();
  browser = await chromium.launch({ headless: true });
}, 30_000);

afterAll(async () => {
  await browser?.close();
  await server?.close();
});

function cargoTest(name: string) {
  execFileSync('cargo', ['test', '--locked', name], { cwd: process.cwd(), stdio: 'pipe' });
}

describe('published claims', () => {
  it('@claim:sample-demo opens a quantitatively correct report and saves nothing', async () => {
    const context = await browser.newContext();
    const page = await context.newPage();
    await page.goto(base);
    await page.getByRole('link', { name: 'Try it with sample data' }).click();
    expect(page.url()).toBe(`${base}/demo`);
    expect(await page.getByRole('heading', { name: 'Review this drain sample' }).count()).toBe(1);
    expect(await page.getByText('Demo — sample data, nothing is saved').count()).toBe(1);
    expect(await page.getByText('558.1 KiB').count()).toBe(1);
    expect(await page.getByText('2.3 MiB').count()).toBe(1);
    expect(await page.getByText('17', { exact: true }).count()).toBe(1);
    expect(await page.evaluate(() => ({ local: localStorage.length, session: sessionStorage.length }))).toEqual({ local: 0, session: 0 });
    await context.close();
  });

  it('@claim:local-only makes no third-party request or browser storage write', async () => {
    const context = await browser.newContext();
    const page = await context.newPage();
    const requests: string[] = [];
    page.on('request', (request) => requests.push(request.url()));
    await page.goto(`${base}/demo`);
    await page.getByRole('button', { name: 'Reset demo' }).click();
    expect(requests.every((url) => new URL(url).origin === base)).toBe(true);
    expect(await context.cookies()).toEqual([]);
    expect(await page.evaluate(() => localStorage.length + sessionStorage.length)).toBe(0);
    await context.close();
    cargoTest('listener_binds_to_loopback');
  }, CARGO_CLAIM_TIMEOUT_MS);

  it('@claim:discard-default proves accepted values are absent from the report', () => {
    cargoTest('detects_without_storing_values');
    cargoTest('receiver_rejects_bad_requests_and_keeps_prior_events');
  }, CARGO_CLAIM_TIMEOUT_MS);

  it('@claim:contract-report proves fields, types, findings, and retention values', () => {
    cargoTest('documented_sample_has_exact_metrics');
  }, CARGO_CLAIM_TIMEOUT_MS);

  it('@claim:false-positive-controls proves custom and ignored field handling', () => {
    cargoTest('supports_custom_patterns_and_explicit_suppression');
  }, CARGO_CLAIM_TIMEOUT_MS);

  it('@claim:rate-limit proves the rolling request threshold', () => {
    cargoTest('rate_limit_returns_429_with_retry_after');
  }, CARGO_CLAIM_TIMEOUT_MS);

  it('@claim:request-recovery proves bad requests do not end the window', () => {
    cargoTest('receiver_rejects_bad_requests_and_keeps_prior_events');
  }, CARGO_CLAIM_TIMEOUT_MS);

  it('@claim:explicit-save writes only accepted bodies when requested', () => {
    cargoTest('save_sample_writes_only_accepted_bodies_when_requested');
  }, CARGO_CLAIM_TIMEOUT_MS);

  it('@claim:interrupt-report writes a report after Ctrl-C', () => {
    cargoTest('interrupt_writes_the_partial_report');
  }, CARGO_CLAIM_TIMEOUT_MS);

  it('@claim:portable-demo runs the embedded sample outside the repository', () => {
    cargoTest('installed_demo_runs_outside_repository');
  }, CARGO_CLAIM_TIMEOUT_MS);

  it('@claim:mit-license proves the stated license is present', () => {
    expect(readFileSync('LICENSE', 'utf8')).toContain('Permission is hereby granted, free of charge');
  });
});

describe('responsive and accessible site', () => {
  for (const viewport of [{ width: 1440, height: 900 }, { width: 390, height: 844 }]) {
    for (const route of routes) {
      it(`has no serious accessibility violations at ${viewport.width}px on ${route}`, async () => {
        const context = await browser.newContext({ viewport });
        const page = await context.newPage();
        const errors: string[] = [];
        page.on('console', (message) => { if (message.type() === 'error') errors.push(message.text()); });
        page.on('pageerror', (error) => errors.push(error.message));
        await page.goto(`${base}${route}`);
        const results = await new AxeBuilder({ page }).analyze();
        expect(results.violations.filter(({ impact }) => impact === 'serious' || impact === 'critical')).toEqual([]);
        expect(errors).toEqual([]);
        expect(await page.locator('h1').count()).toBe(1);
        expect(await page.locator('main').count()).toBe(1);
        await context.close();
      });
    }
  }

  it('keeps the primary action visible on a 390px first screen', async () => {
    const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
    const page = await context.newPage();
    await page.goto(base);
    const box = await page.getByRole('link', { name: 'Try it with sample data' }).boundingBox();
    expect(box).not.toBeNull();
    expect(box!.y + box!.height).toBeLessThanOrEqual(844);
    await context.close();
  });

  it('moves focus to the route heading after keyboard navigation', async () => {
    const page = await browser.newPage();
    await page.goto(base);
    const link = page.getByRole('link', { name: 'Try it with sample data' });
    await link.focus();
    await link.press('Enter');
    expect(await page.evaluate(() => document.activeElement?.tagName)).toBe('H1');
    await page.close();
  });

  it('keeps links touch-sized and content visible at 200% text size', async () => {
    const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
    const page = await context.newPage();
    await page.goto(base);
    await page.evaluate(() => { document.documentElement.style.fontSize = '32px'; });
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
    const targets = await page.locator('header a, footer a').evaluateAll((elements) => elements.map((element) => {
      const box = element.getBoundingClientRect();
      return { width: box.width, height: box.height };
    }));
    expect(targets.every(({ width, height }) => width >= 44 && height >= 44)).toBe(true);
    await context.close();
  });

  it('supports keyboard reset and reduced motion', async () => {
    const context = await browser.newContext({ reducedMotion: 'reduce' });
    const page = await context.newPage();
    await page.goto(`${base}/demo`);
    await page.evaluate(() => localStorage.setItem('demo:drain-check', 'test'));
    const reset = page.getByRole('button', { name: 'Reset demo' });
    await reset.focus();
    await page.keyboard.press('Space');
    expect(await page.evaluate(() => localStorage.getItem('demo:drain-check'))).toBeNull();
    expect(await page.locator('.demo-banner').evaluate((element) => getComputedStyle(element).animationDuration)).not.toBe('.25s');
    await context.close();
  });
});
