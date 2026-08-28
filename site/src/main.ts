import './style.css';

const app = document.querySelector<HTMLDivElement>('#app')!;
const shell = (content: string) => `<a class="skip" href="#main">Skip to content</a><header><a class="wordmark" href="/" data-route>DRΛIN<br>CHECK</a><nav aria-label="Main navigation"><a href="/demo" data-route>Demo</a><a href="/#how">How it works</a><a href="/privacy" data-route>Privacy</a></nav></header>${content}<footer><p>Drain Check samples a log drain before you forward it.</p><p><a href="/privacy" data-route>Privacy</a> · <a href="/terms" data-route>Terms</a> · Built by Param Factory · v0.1.0</p></footer><div id="route-status" class="sr-only" aria-live="polite"></div>`;

function landing() {
  document.title = 'Drain Check — inspect a log drain sample';
  return shell(`<main id="main" tabindex="-1"><section class="hero"><div class="hero-copy"><p class="eyebrow">LOCAL PRE-FLIGHT / 10-MINUTE WINDOW</p><h1>Inspect a log drain before forwarding</h1><p class="lede">For platform teams who need volume, fields, and privacy risks before a drain stays on.</p><p><a class="button primary" href="/demo" data-route>Try it with sample data</a><span class="button-note">Opens a sample report. Nothing is saved.</span></p><ul class="facts"><li>Listens on your machine.</li><li>Discards bodies by default.</li><li>Free and open source.</li></ul></div><figure class="hero-art"><img src="/drain-console.webp" width="1024" height="1024" fetchpriority="high" alt="A pixel-art local receiver sorts log packets into review lanes." /><figcaption>THE RECEIVER STAYS LOCAL</figcaption></figure></section><section class="terminal" aria-labelledby="run-title"><div class="terminal-bar"><span>●</span><span id="run-title">bundled sample / recorded run</span></div><pre><code>$ cargo run -- demo
Reviewed 3 events in 600s. 17 fields. 3 possible risks.
Report: /tmp/drain-check-demo-report.json

$ cargo run -- forwarding --url https://receiver.example/logs
# generic-http
url = "https://receiver.example/logs"
method = "POST"</code></pre></section><section id="how" class="steps" aria-labelledby="how-title"><p class="eyebrow">THE SHORT PATH</p><h2 id="how-title">Review a drain in three steps</h2><ol><li><strong>Listen locally.</strong><br>Run one bounded window.</li><li><strong>Read the contract.</strong><br>Check fields and likely secrets.</li><li><strong>Forward with intent.</strong><br>Use the generated template.</li></ol></section><section class="limits" aria-labelledby="limits-title"><h2 id="limits-title">What Drain Check does not do</h2><p>It does not store logs, search logs, or send them to a vendor.</p><p>Use <code>--save-sample</code> only when you choose to retain raw bodies.</p></section><section class="install" aria-labelledby="install-title"><p class="eyebrow">RUN IT LOCALLY</p><h2 id="install-title">Start a bounded receiver</h2><pre><code>cargo install drain-check
# or clone this repository
cargo run -- listen --duration 600 --port 8787</code></pre><p>Then point your temporary HTTP drain to <code>http://127.0.0.1:8787/</code>.</p></section></main>`);
}

function demo() {
  document.title = 'Demo — Drain Check';
  return shell(`<div class="demo-banner" role="status"><span>Demo — sample data, nothing is saved</span><button id="reset-demo">Reset demo</button><a href="/" data-route>Start for real</a></div><main id="main" tabindex="-1" class="demo"><p class="eyebrow">SAMPLE REPORT / 10-MINUTE WINDOW</p><h1>Review this drain sample</h1><p class="lede">This report uses bundled checkout logs. It is separate from any real run.</p><section class="metrics" aria-label="Sample summary"><div><strong>3</strong><span>events</span></div><div><strong>0.005 / sec</strong><span>event rate</span></div><div><strong>17</strong><span>field paths</span></div><div><strong>2</strong><span>review fields</span></div></section><section class="report" aria-labelledby="risk-title"><div><h2 id="risk-title">Review possible sensitive fields</h2><p>Detectors are conservative. A match needs a human decision.</p><article><p><code>$.request.authorization</code> <span class="badge">HIGH</span></p><p>secret-shaped value</p><p>Mask this value or exclude the field.</p></article><article><p><code>$.request.user_email</code> <span class="badge">MEDIUM</span></p><p>email-shaped value</p><p>Confirm that this identifier belongs in the drain.</p></article></div><div><h2>Retention estimate</h2><p>At this sample rate and size:</p><dl><dt>7 days</dt><dd>about 5.7 KiB</dd><dt>30 days</dt><dd>about 24.5 KiB</dd></dl><h2>Forwarding template</h2><pre><code>url = "https://receiver.example/logs"
method = "POST"
content_type = "application/json"</code></pre></div></section><p><a class="button" href="/" data-route>Read local setup</a></p></main>`);
}

function policy(kind: 'privacy' | 'terms') {
  const privacy = kind === 'privacy'; document.title = `${privacy ? 'Privacy' : 'Terms'} — Drain Check`;
  const copy = privacy ? '<p>Drain Check binds to 127.0.0.1 by default. It sends no analytics or log bodies anywhere.</p><p>The receiver aggregates field names, types, counts, and detector results. It discards each body after aggregation unless you pass <code>--save-sample</code>.</p><p>The website has no accounts, cookies, or tracking.</p>' : '<p>Drain Check is free software under the MIT License. You decide where to point your drain and which data to save.</p><p>Secret detectors are warnings, not a guarantee. Review every match before forwarding production logs.</p>';
  return shell(`<main id="main" tabindex="-1" class="prose"><p class="eyebrow">${privacy ? 'LOCAL DATA POLICY' : 'TERMS OF USE'}</p><h1>${privacy ? 'Your drain stays on your machine' : 'Use Drain Check at your own boundary'}</h1>${copy}</main>`);
}

function notFound() { document.title = 'Not found — Drain Check'; return shell('<main id="main" tabindex="-1" class="prose"><p class="eyebrow">404 / SIGNAL LOST</p><h1>That screen is not in this receiver</h1><p>Return to the local pre-flight guide.</p><p><a class="button" href="/" data-route>Return home</a></p></main>'); }
function render() { const path = location.pathname.replace(/\/$/, '') || '/'; const page = path === '/' ? landing : path === '/demo' ? demo : path === '/privacy' ? () => policy('privacy') : path === '/terms' ? () => policy('terms') : notFound; app.innerHTML = page(); app.querySelectorAll<HTMLAnchorElement>('[data-route]').forEach(link => link.addEventListener('click', event => { event.preventDefault(); history.pushState({}, '', link.href); render(); })); app.querySelector('#reset-demo')?.addEventListener('click', () => { localStorage.removeItem('demo:drain-check'); render(); }); const h1 = app.querySelector<HTMLElement>('h1'); h1?.focus(); document.querySelector('#route-status')!.textContent = document.title; }
addEventListener('popstate', render); render();
