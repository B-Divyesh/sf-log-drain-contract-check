import { chromium } from 'playwright';
import AxeBuilder from '@axe-core/playwright';
import { writeFile } from 'node:fs/promises';

const base = 'https://log-drain-contract-check.sociobot.in';
const out = new URL('./', import.meta.url);
const browser = await chromium.launch({ headless: true });
const routes = ['/', '/?demo=1', '/demo', '/privacy', '/terms', '/missing'];
const viewports = {
  desktop: { width: 1440, height: 900 },
  mobile: { width: 390, height: 844 },
};
const result = { auditedAt: new Date().toISOString(), base, routes: {}, flows: {} };

for (const [viewportName, viewport] of Object.entries(viewports)) {
  result.routes[viewportName] = {};
  for (const route of routes) {
    const context = await browser.newContext({ viewport, reducedMotion: 'reduce' });
    const page = await context.newPage();
    const requests = [];
    const consoleErrors = [];
    const pageErrors = [];
    page.on('request', request => requests.push(request.url()));
    page.on('console', message => {
      if (message.type() === 'error') consoleErrors.push(message.text());
    });
    page.on('pageerror', error => pageErrors.push(error.message));
    const response = await page.goto(base + route, { waitUntil: 'networkidle' });
    const audit = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa']).analyze();
    const facts = await page.evaluate(() => {
      const interactives = [...document.querySelectorAll('a, button, input, select, textarea')]
        .filter(element => {
          const style = getComputedStyle(element);
          const rect = element.getBoundingClientRect();
          return style.display !== 'none' && style.visibility !== 'hidden' && rect.width > 0 && rect.height > 0;
        })
        .map(element => {
          const rect = element.getBoundingClientRect();
          return { text: element.textContent?.trim() || element.getAttribute('aria-label') || '', width: rect.width, height: rect.height };
        });
      return {
        title: document.title,
        lang: document.documentElement.lang,
        h1Count: document.querySelectorAll('h1').length,
        h1: document.querySelector('h1')?.textContent?.trim(),
        main: Boolean(document.querySelector('main')),
        skip: document.querySelector('a.skip')?.getAttribute('href'),
        missingAlt: [...document.querySelectorAll('img')].filter(image => !image.hasAttribute('alt')).length,
        horizontalOverflow: document.documentElement.scrollWidth - document.documentElement.clientWidth,
        activeAnimations: document.getAnimations().length,
        interactives,
      };
    });
    const slug = route === '/' ? 'home' : route.replace(/[/?=&]/g, '_');
    await page.screenshot({ path: new URL(`${viewportName}-${slug}.png`, out).pathname, fullPage: true });
    result.routes[viewportName][route] = {
      status: response?.status(),
      mainHeaders: response?.headers(),
      ...facts,
      smallTargets: viewportName === 'mobile' ? facts.interactives.filter(item => item.width < 44 || item.height < 44) : [],
      requestUrls: requests,
      thirdPartyRequests: requests.filter(url => new URL(url).origin !== base),
      consoleErrors,
      pageErrors,
      axeViolations: audit.violations.map(v => ({ id: v.id, impact: v.impact, nodes: v.nodes.length })),
      axeSeriousCritical: audit.violations.filter(v => ['serious', 'critical'].includes(v.impact)).map(v => v.id),
    };
    await context.close();
  }
}

{
  const context = await browser.newContext({ viewport: viewports.mobile });
  const page = await context.newPage();
  const requests = [];
  const consoleErrors = [];
  const pageErrors = [];
  page.on('request', request => requests.push(request.url()));
  page.on('console', message => { if (message.type() === 'error') consoleErrors.push(message.text()); });
  page.on('pageerror', error => pageErrors.push(error.message));
  await page.goto(base + '/', { waitUntil: 'networkidle' });
  const firstRead = await page.evaluate(() => {
    const cta = document.querySelector('a.primary');
    const rect = cta?.getBoundingClientRect();
    return {
      h1: document.querySelector('h1')?.textContent?.trim(),
      lede: document.querySelector('.lede')?.textContent?.trim(),
      action: cta?.textContent?.trim(),
      actionNote: document.querySelector('.button-note')?.textContent?.trim(),
      actionInFirstViewport: Boolean(rect && rect.top >= 0 && rect.bottom <= innerHeight),
      facts: [...document.querySelectorAll('.facts li')].map(item => item.textContent?.trim()),
    };
  });
  await page.locator('body').press('Tab');
  const firstTab = await page.evaluate(() => ({
    text: document.activeElement?.textContent?.trim(),
    href: document.activeElement?.getAttribute('href'),
    outline: getComputedStyle(document.activeElement).outline,
  }));
  await page.keyboard.press('Enter');
  const skipFocus = await page.evaluate(() => ({ id: document.activeElement?.id, tag: document.activeElement?.tagName }));
  await page.locator('a.primary').focus();
  const primaryFocus = await page.evaluate(() => ({
    text: document.activeElement?.textContent?.trim(),
    outline: getComputedStyle(document.activeElement).outline,
    outlineColor: getComputedStyle(document.activeElement).outlineColor,
  }));
  await page.keyboard.press('Enter');
  await page.waitForLoadState('networkidle');
  const demoAfterOneClick = await page.evaluate(() => ({
    url: location.href,
    banner: document.querySelector('.demo-banner span')?.textContent?.trim(),
    h1: document.querySelector('h1')?.textContent?.trim(),
    metrics: [...document.querySelectorAll('.metrics strong')].map(item => item.textContent?.trim()),
  }));
  await page.evaluate(() => {
    localStorage.setItem('demo:drain-check', 'discard-me');
    localStorage.setItem('real:qa', 'keep-me');
  });
  await page.getByRole('button', { name: 'Reset demo' }).click();
  const afterReset = await page.evaluate(async () => ({
    demo: localStorage.getItem('demo:drain-check'),
    real: localStorage.getItem('real:qa'),
    cookies: document.cookie,
    sessionKeys: Object.keys(sessionStorage),
    indexedDB: (await indexedDB.databases()).map(db => db.name),
    caches: await caches.keys(),
    serviceWorkers: (await navigator.serviceWorker.getRegistrations()).length,
    opfs: await (async () => {
      if (!navigator.storage?.getDirectory) return [];
      const root = await navigator.storage.getDirectory();
      const names = [];
      for await (const [name] of root.entries()) names.push(name);
      return names;
    })(),
  }));
  const exitLabel = await page.locator('.demo-banner a').textContent();
  await page.locator('.demo-banner a').click();
  const afterExit = await page.evaluate(() => ({
    path: location.pathname + location.search,
    focus: document.activeElement?.tagName + ':' + document.activeElement?.textContent?.trim(),
    real: localStorage.getItem('real:qa'),
  }));
  await page.goBack();
  const afterBack = await page.evaluate(() => ({
    path: location.pathname + location.search,
    h1: document.querySelector('h1')?.textContent?.trim(),
    focus: document.activeElement?.tagName + ':' + document.activeElement?.textContent?.trim(),
    real: localStorage.getItem('real:qa'),
  }));
  result.flows.firstReadDemoKeyboardPrivacy = {
    firstRead, firstTab, skipFocus, primaryFocus, demoAfterOneClick, afterReset,
    exitLabel: exitLabel?.trim(), afterExit, afterBack, requests,
    thirdPartyRequests: requests.filter(url => new URL(url).origin !== base), consoleErrors, pageErrors,
  };
  await context.close();
}

{
  const context = await browser.newContext({ viewport: viewports.mobile });
  const page = await context.newPage();
  await page.goto(base + '/', { waitUntil: 'networkidle' });
  await page.evaluate(() => { document.documentElement.style.fontSize = '200%'; });
  result.flows.mobileText200 = await page.evaluate(() => ({
    horizontalOverflow: document.documentElement.scrollWidth - document.documentElement.clientWidth,
    h1Visible: Boolean(document.querySelector('h1')?.getBoundingClientRect().height),
    actionVisible: Boolean(document.querySelector('a.primary')?.getBoundingClientRect().height),
  }));
  await page.screenshot({ path: new URL('mobile-text-200.png', out).pathname, fullPage: true });
  await context.close();
}

{
  const context = await browser.newContext();
  const page = await context.newPage();
  await page.goto(base + '/', { waitUntil: 'networkidle' });
  const hrefs = await page.locator('a').evaluateAll(links => [...new Set(links.map(link => link.href))]);
  const crawled = [];
  for (const href of hrefs) {
    if (!href.startsWith(base)) continue;
    const response = await context.request.get(href);
    crawled.push({ href, status: response.status() });
  }
  for (const path of ['/robots.txt', '/sitemap.xml', '/favicon.svg', '/apple-touch-icon.png', '/og-drain-console.webp']) {
    const response = await context.request.get(base + path);
    crawled.push({ href: base + path, status: response.status(), headers: response.headers() });
  }
  result.flows.linkCrawl = crawled;
  await context.close();
}

await browser.close();
await writeFile(new URL('live-qa.json', out), JSON.stringify(result, null, 2));
console.log(JSON.stringify({
  axeSeriousCritical: Object.values(result.routes).flatMap(routesByViewport => Object.values(routesByViewport).flatMap(route => route.axeSeriousCritical)).length,
  consoleErrors: Object.values(result.routes).flatMap(routesByViewport => Object.values(routesByViewport).flatMap(route => route.consoleErrors)).length + result.flows.firstReadDemoKeyboardPrivacy.consoleErrors.length,
  pageErrors: Object.values(result.routes).flatMap(routesByViewport => Object.values(routesByViewport).flatMap(route => route.pageErrors)).length + result.flows.firstReadDemoKeyboardPrivacy.pageErrors.length,
  thirdPartyRequests: Object.values(result.routes).flatMap(routesByViewport => Object.values(routesByViewport).flatMap(route => route.thirdPartyRequests)).length + result.flows.firstReadDemoKeyboardPrivacy.thirdPartyRequests.length,
  firstRead: result.flows.firstReadDemoKeyboardPrivacy.firstRead,
  demo: result.flows.firstReadDemoKeyboardPrivacy.demoAfterOneClick,
  storage: result.flows.firstReadDemoKeyboardPrivacy.afterReset,
  exitLabel: result.flows.firstReadDemoKeyboardPrivacy.exitLabel,
}, null, 2));
