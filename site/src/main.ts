import './style.css';
import sampleReport from './sample-report.json';

declare const __BUILD_ID__: string;

const app = document.querySelector<HTMLDivElement>('#app')!;
const routeStatus = () => document.querySelector<HTMLElement>('#route-status');
const buildId = typeof __BUILD_ID__ === 'string' ? __BUILD_ID__ : 'dev';

const shell = (content: string) => `<a class="skip" href="#main">Skip to content</a>
  <header>
    <a class="wordmark" href="/" data-route aria-label="Drain Check home">DRΛIN<br>CHECK</a>
    <nav aria-label="Main navigation">
      <a href="/demo" data-route>Demo</a><a href="/#how">How it works</a><a href="/privacy" data-route>Privacy</a>
    </nav>
  </header>
  ${content}
  <footer>
    <p>Drain Check samples a log drain before you forward it.</p>
    <p><a href="/privacy" data-route>Privacy</a><span aria-hidden="true"> · </span><a href="/terms" data-route>Terms</a><span aria-hidden="true"> · </span>Built by Param Factory<span aria-hidden="true"> · </span>v0.1.0+${buildId}</p>
  </footer>
  <div id="route-status" class="sr-only" aria-live="polite"></div>`;

function setMetadata(title: string, description: string, path: string) {
  document.title = title;
  document.querySelector<HTMLMetaElement>('meta[name="description"]')!.content = description;
  document.querySelector<HTMLLinkElement>('link[rel="canonical"]')!.href = `https://log-drain-contract-check.sociobot.in${path}`;
  document.querySelector<HTMLMetaElement>('meta[property="og:title"]')!.content = title;
  document.querySelector<HTMLMetaElement>('meta[property="og:description"]')!.content = description;
  document.querySelector<HTMLMetaElement>('meta[name="twitter:title"]')!.content = title;
  document.querySelector<HTMLMetaElement>('meta[name="twitter:description"]')!.content = description;
}

function landing() {
  setMetadata('Drain Check — inspect a log drain sample', 'Inspect a bounded local log drain sample before forwarding it.', '/');
  return shell(`<main id="main" tabindex="-1">
    <section class="hero">
      <div class="hero-copy">
        <p class="eyebrow">LOCAL PRE-FLIGHT / 10-MINUTE WINDOW</p>
        <h1 tabindex="-1">Inspect a log drain before forwarding</h1>
        <p class="lede">For platform teams who need volume, fields, and privacy risks before a drain stays on.</p>
        <p class="hero-action"><a class="button primary" href="/demo" data-route>Try it with sample data</a><span class="button-note">Opens a sample report. Nothing is saved.</span></p>
        <ul class="facts"><li>Listens only on your machine.</li><li>Discards bodies after aggregation.</li><li>Free under the MIT License.</li></ul>
      </div>
      <figure class="hero-art"><img src="/drain-console.webp" width="1024" height="1024" fetchpriority="high" alt="A pixel-art local receiver sorts log packets into review lanes."><figcaption>THE RECEIVER STAYS LOCAL</figcaption></figure>
    </section>
    <section class="terminal" tabindex="0" aria-labelledby="run-title" aria-describedby="run-help">
      <div class="terminal-bar"><span aria-hidden="true">●</span><span id="run-title">bundled sample / recorded run</span><button id="replay-demo" type="button">Replay recording</button></div>
      <p id="run-help" class="sr-only">A text recording of the bundled CLI demo. Use the replay button to play it again.</p>
      <pre tabindex="0"><code id="terminal-output">$ drain-check demo
Reviewed 3 events in 600s. 17 fields. 3 possible risks.
Report: /tmp/drain-check-demo-[unique]/report.json

$ drain-check forwarding --url https://receiver.example/logs
# generic-http
url = "https://receiver.example/logs"
method = "POST"</code></pre>
    </section>
    <section id="how" class="steps" aria-labelledby="how-title">
      <p class="eyebrow">THE SHORT PATH</p><h2 id="how-title">Review a drain in three steps</h2>
      <ol><li><strong>Listen locally.</strong><br>Run one bounded window.</li><li><strong>Read the contract.</strong><br>Check fields and likely secrets.</li><li><strong>Forward with intent.</strong><br>Use the generated template.</li></ol>
    </section>
    <section class="limits" aria-labelledby="limits-title"><h2 id="limits-title">What Drain Check does not do</h2><p>It does not store logs, search logs, or send them to a vendor.</p><p>Use <code>--save-sample</code> only when you choose to retain accepted bodies.</p></section>
    <section class="install" aria-labelledby="install-title"><p class="eyebrow">RUN IT LOCALLY</p><h2 id="install-title">Start a bounded receiver</h2><pre tabindex="0"><code>cargo install drain-check
# or clone this repository
cargo run -- listen --duration 600 --port 8787</code></pre><p>Then point your temporary HTTP drain to <code>http://127.0.0.1:8787/</code>.</p><p>Use <code>--ignore-field '$.request_id'</code> to suppress a reviewed false positive.</p></section>
  </main>`);
}

function findingMarkup() {
  const byPath = new Map<string, (typeof sampleReport.findings)[number]>();
  for (const finding of sampleReport.findings) {
    byPath.set(finding.path, finding);
  }
  return [...byPath.values()].map((finding) => `<article><p><code>${finding.path}</code> <span class="badge">${finding.confidence.toUpperCase()}</span></p><p>${finding.detector}</p><p>${finding.action}</p></article>`).join('');
}

function demo() {
  setMetadata('Demo — Drain Check', 'Review the bundled Drain Check sample report without saving data.', '/demo');
  const sevenDays = sampleReport.retention.find((estimate) => estimate.days === 7)!;
  const thirtyDays = sampleReport.retention.find((estimate) => estimate.days === 30)!;
  const reviewFields = new Set(sampleReport.findings.map((finding) => finding.path)).size;
  return shell(`<div class="demo-banner" role="status"><span>Demo — sample data, nothing is saved</span><button id="reset-demo" type="button">Reset demo</button><a href="/" data-route>Start for real</a></div>
    <main id="main" tabindex="-1" class="demo"><p class="eyebrow">SAMPLE REPORT / 10-MINUTE WINDOW</p><h1 tabindex="-1">Review this drain sample</h1><p class="lede">This report uses bundled checkout logs. It is separate from any real run.</p>
      <section class="metrics" aria-label="Sample summary"><div><strong>${sampleReport.events}</strong><span>events</span></div><div><strong>${sampleReport.events_per_second} / sec</strong><span>event rate</span></div><div><strong>${sampleReport.fields.length}</strong><span>field paths</span></div><div><strong>${reviewFields}</strong><span>review fields</span></div></section>
      <section class="report" aria-labelledby="risk-title"><div><h2 id="risk-title">Review possible sensitive fields</h2><p>Detectors are conservative. A match needs a human decision.</p>${findingMarkup()}</div><div><h2>Retention estimate</h2><p>At this sample rate and average event size:</p><dl><dt>7 days</dt><dd>about ${sevenDays.display}</dd><dt>30 days</dt><dd>about ${thirtyDays.display}</dd></dl><h2>Forwarding template</h2><pre tabindex="0"><code>url = "https://receiver.example/logs"
method = "POST"
content_type = "application/json"</code></pre></div></section><p><a class="button" href="/" data-route>Read local setup</a></p>
    </main>`);
}

function policy(kind: 'privacy' | 'terms') {
  const privacy = kind === 'privacy';
  const title = `${privacy ? 'Privacy' : 'Terms'} — Drain Check`;
  setMetadata(title, privacy ? 'How Drain Check handles drain samples and website data.' : 'Terms for using Drain Check.', `/${kind}`);
  const copy = privacy
    ? '<p>Drain Check binds to 127.0.0.1. It sends no analytics or log bodies anywhere.</p><p>The receiver aggregates field names, types, counts, and detector results. It discards each body after aggregation unless you pass <code>--save-sample</code>.</p><p>The website has no accounts, cookies, or tracking.</p>'
    : '<p>Drain Check is free software under the MIT License. You decide where to point your drain and which data to save.</p><p>Secret detectors are warnings, not a guarantee. Review every match before forwarding production logs.</p>';
  return shell(`<main id="main" tabindex="-1" class="prose"><p class="eyebrow">${privacy ? 'LOCAL DATA POLICY' : 'TERMS OF USE'}</p><h1 tabindex="-1">${privacy ? 'Your drain stays on your machine' : 'Use Drain Check at your own boundary'}</h1>${copy}</main>`);
}

function notFound() {
  setMetadata('Not found — Drain Check', 'Return to the Drain Check local receiver guide.', location.pathname);
  return shell('<main id="main" tabindex="-1" class="prose"><p class="eyebrow">404 / SIGNAL LOST</p><h1 tabindex="-1">That screen is not in this receiver</h1><p>Return to the local pre-flight guide.</p><p><a class="button" href="/" data-route>Return home</a></p></main>');
}

function replayRecording() {
  const output = document.querySelector<HTMLElement>('#terminal-output');
  if (!output) return;
  const fullText = output.textContent ?? '';
  if (matchMedia('(prefers-reduced-motion: reduce)').matches) {
    output.textContent = fullText;
    return;
  }
  output.textContent = '';
  let index = 0;
  const timer = window.setInterval(() => {
    output.textContent = fullText.slice(0, index += 4);
    if (index >= fullText.length) window.clearInterval(timer);
  }, 16);
}

function render(restoreFocus = true) {
  const path = location.pathname.replace(/\/$/, '') || '/';
  const page = path === '/' ? landing : path === '/demo' ? demo : path === '/privacy' ? () => policy('privacy') : path === '/terms' ? () => policy('terms') : notFound;
  app.innerHTML = page();
  app.querySelectorAll<HTMLAnchorElement>('[data-route]').forEach((link) => link.addEventListener('click', (event) => {
    event.preventDefault();
    history.pushState({}, '', link.href);
    render();
  }));
  app.querySelector('#reset-demo')?.addEventListener('click', () => {
    localStorage.removeItem('demo:drain-check');
    render();
  });
  app.querySelector('#replay-demo')?.addEventListener('click', replayRecording);
  if (restoreFocus) app.querySelector<HTMLElement>('h1')?.focus();
  routeStatus()!.textContent = document.title;
  scrollTo({ top: 0, behavior: 'instant' });
}

addEventListener('popstate', () => render());
render(false);
