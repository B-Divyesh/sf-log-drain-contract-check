import { chromium } from 'playwright';
import AxeBuilder from '@axe-core/playwright';

const origin = 'https://log-drain-contract-check.sociobot.in';
const routes = ['/', '/demo', '/privacy', '/terms', '/definitely-missing'];
const viewports = [
  { name: 'desktop', width: 1440, height: 900 },
  { name: 'mobile', width: 390, height: 844 },
];

const browser = await chromium.launch({ headless: true });
const routeResults = [];

for (const viewport of viewports) {
  for (const route of routes) {
    const context = await browser.newContext({ viewport });
    const page = await context.newPage();
    const requests = [];
    const errors = [];
    const responseHeaders = [];
    page.on('request', (request) => requests.push(request.url()));
    page.on('response', async (response) => responseHeaders.push({ url: response.url(), status: response.status(), headers: await response.allHeaders() }));
    page.on('console', (message) => { if (message.type() === 'error') errors.push(`console: ${message.text()}`); });
    page.on('pageerror', (error) => errors.push(`page: ${error.message}`));
    const response = await page.goto(`${origin}${route}`, { waitUntil: 'networkidle' });
    const axe = await new AxeBuilder({ page }).analyze();
    const severe = axe.violations.filter((violation) => ['serious', 'critical'].includes(violation.impact ?? ''));
    const layout = await page.evaluate(() => ({
      lang: document.documentElement.lang,
      title: document.title,
      h1Count: document.querySelectorAll('h1').length,
      h1: [...document.querySelectorAll('h1')].map((node) => node.textContent?.trim()),
      mainCount: document.querySelectorAll('main').length,
      imgAlts: [...document.images].map((image) => ({ src: image.currentSrc, alt: image.getAttribute('alt') })),
      width: { client: document.documentElement.clientWidth, scroll: document.documentElement.scrollWidth },
      storage: { local: Object.keys(localStorage), session: Object.keys(sessionStorage), cookies: document.cookie },
      targets: [...document.querySelectorAll('a,button')].filter((element) => {
        const style = getComputedStyle(element);
        const rect = element.getBoundingClientRect();
        return style.display !== 'none' && style.visibility !== 'hidden' && rect.width > 0 && rect.height > 0;
      }).map((element) => {
        const rect = element.getBoundingClientRect();
        return { text: element.textContent?.trim(), tag: element.tagName, width: rect.width, height: rect.height };
      }),
    }));
    if (route === '/' || route === '/demo') {
      await page.screenshot({ path: `/work/repo/.factory/verification-artifacts/live-${viewport.name}-${route === '/' ? 'home' : 'demo'}.png`, fullPage: true });
    }
    routeResults.push({ viewport, route, status: response?.status(), ...layout, requests, thirdParty: requests.filter((url) => new URL(url).origin !== origin), errors, severeAxe: severe.map((item) => ({ id: item.id, impact: item.impact, nodes: item.nodes.length, help: item.help })), responseHeaders });
    await context.close();
  }
}

const flowContext = await browser.newContext({ viewport: { width: 390, height: 844 } });
const flowPage = await flowContext.newPage();
const flowRequests = [];
const flowErrors = [];
flowPage.on('request', (request) => flowRequests.push(request.url()));
flowPage.on('console', (message) => { if (message.type() === 'error') flowErrors.push(`console: ${message.text()}`); });
flowPage.on('pageerror', (error) => flowErrors.push(`page: ${error.message}`));
await flowPage.goto(origin, { waitUntil: 'networkidle' });
const firstScreen = await flowPage.evaluate(() => {
  const action = [...document.querySelectorAll('a,button')].find((element) => element.textContent?.trim() === 'Try it with sample data');
  const rect = action?.getBoundingClientRect();
  return { text: document.body.innerText, action: rect && { top: rect.top, bottom: rect.bottom, width: rect.width, height: rect.height }, viewport: { width: innerWidth, height: innerHeight } };
});

await flowPage.keyboard.press('Tab');
const keyboardSteps = [];
for (let step = 0; step < 12; step += 1) {
  const focus = await flowPage.evaluate(() => {
    const element = document.activeElement;
    const style = getComputedStyle(element);
    return { tag: element?.tagName, text: element?.textContent?.trim(), href: element?.getAttribute?.('href'), outline: `${style.outlineWidth} ${style.outlineStyle} ${style.outlineColor}`, boxShadow: style.boxShadow };
  });
  keyboardSteps.push(focus);
  if (focus.text === 'Try it with sample data') break;
  await flowPage.keyboard.press('Tab');
}
await flowPage.keyboard.press('Enter');
await flowPage.waitForURL('**/demo');
const demoAfterKeyboard = await flowPage.evaluate(() => ({
  url: location.href,
  active: { tag: document.activeElement?.tagName, text: document.activeElement?.textContent?.trim() },
  banner: document.querySelector('[class*="demo"]')?.textContent?.replace(/\s+/g, ' ').trim(),
  body: document.body.innerText,
  storage: { local: Object.keys(localStorage), session: Object.keys(sessionStorage), cookies: document.cookie },
}));

const reset = flowPage.getByRole('button', { name: 'Reset demo' });
await reset.focus();
await flowPage.keyboard.press('Space');
const resetResult = await flowPage.evaluate(() => ({ active: document.activeElement?.textContent?.trim(), local: Object.keys(localStorage), session: Object.keys(sessionStorage) }));

const startReal = flowPage.getByRole('link', { name: 'Start for real' });
await startReal.focus();
await flowPage.keyboard.press('Enter');
await flowPage.waitForURL(origin + '/');
const startRealResult = { url: flowPage.url(), active: await flowPage.evaluate(() => ({ tag: document.activeElement?.tagName, text: document.activeElement?.textContent?.trim() })) };

await flowPage.goto(origin, { waitUntil: 'networkidle' });
await flowPage.evaluate(() => { document.documentElement.style.fontSize = '200%'; });
const textResize = await flowPage.evaluate(() => ({ clientWidth: document.documentElement.clientWidth, scrollWidth: document.documentElement.scrollWidth, bodyScrollWidth: document.body.scrollWidth }));
await flowContext.close();

const reducedContext = await browser.newContext({ viewport: { width: 390, height: 844 }, reducedMotion: 'reduce' });
const reducedPage = await reducedContext.newPage();
await reducedPage.goto(origin, { waitUntil: 'networkidle' });
const reducedMotion = await reducedPage.evaluate(() => ({
  media: matchMedia('(prefers-reduced-motion: reduce)').matches,
  animations: document.getAnimations().map((animation) => ({ playState: animation.playState, duration: animation.effect?.getTiming().duration })),
  maxDurations: [...document.querySelectorAll('*')].map((element) => {
    const style = getComputedStyle(element);
    return { animation: style.animationDuration, transition: style.transitionDuration };
  }).filter((value) => value.animation !== '0s' || value.transition !== '0s').slice(0, 30),
}));
await reducedContext.close();

console.log(JSON.stringify({ routeResults, firstScreen, keyboardSteps, demoAfterKeyboard, resetResult, startRealResult, textResize, flowRequests, flowThirdParty: flowRequests.filter((url) => new URL(url).origin !== origin), flowErrors, reducedMotion }, null, 2));
await browser.close();
