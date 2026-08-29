# Independent verification 6 — PASS

**Candidate:** `d9231ed433c7c173f5987e4ab574b23f153e2400`  
**Live URL:** <https://log-drain-contract-check.sociobot.in>  
**Verified:** 2026-08-29 (fresh install in `/work/repo`, no product-code changes)

## Decision

**PASS.** The live static assets are byte-for-byte identical to this candidate's
production build. The local CLI and the deployed demonstration satisfy the
researched brief: a bounded, loopback-only receiver samples logs, reports
volume/field types/sensitive-data findings and retention, and emits a separate
forwarding configuration without retaining accepted bodies by default.

## Required first checks

`npm ci` completed with no vulnerabilities. Every literal command named in
`.factory/claims.json` was run separately from this checkout and passed:

| Claim IDs | Result |
| --- | --- |
| `sample-demo`, `local-only`, `discard-default`, `contract-report` | pass |
| `forwarding-config`, `source-checkout`, `false-positive-controls`, `rate-limit` | pass |
| `request-recovery`, `explicit-save`, `separate-output-paths`, `minimum-sample-duration` | pass |
| `json-stdout`, `complete-help`, `interrupt-report`, `portable-demo`, `mit-license` | pass |

The slowest claim, `npm test -- -t @claim:source-checkout`, passed in 28.26 s
and cloned the documented public GitHub source before running `cargo run --locked -- --help`.
The complete suite, `npm test`, subsequently exited `0`.

Cold live-page first read (new browser context) answered all three required
questions plainly: it says it inspects a log drain before forwarding, names
platform teams checking volume/field types/sensitive data, and offers **Try it
with sample data**. That single click opens the bundled report at `/?demo=1`;
it displays the sample banner, 17 paths, 558.1 KiB/7 days, 2.3 MiB/30 days, and
does not write browser storage.

## Local quality gates

All passed:

```sh
npm run typecheck
npm run build
cargo test --all-targets --all-features --locked
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --locked
```

The production build created `dist/site`; its initial JS is 11.33 kB (4.26 kB
gzip) and CSS is 6.71 kB (2.21 kB gzip). `cargo package --locked` packaged and
verified the 10-file crate (58.4 kB; 16.4 kB compressed).

## Independent product exercise

- Installed the CLI into `/tmp/drain-check-consumer-qa` with `cargo install --path . --root … --locked`, then used it from fresh temporary consumer directories.
- `drain-check demo --json` produced the embedded three-event report in a new `/tmp/drain-check-demo-*` directory with the expected 17 paths and 558.1 KiB/2.3 MiB retention estimates.
- `forwarding --url 'https://receiver.example/logs?token=a b'` emitted a percent-encoded safe configuration. `ftp://…` was rejected; `inspect --sample-seconds 0` was rejected with Clap range validation.
- Started the installed listener with `--rate-limit 1`: valid NDJSON received **202**, the next request **429** with `Retry-After: 1`, malformed NDJSON **400**, and a following valid request **202**. The report held two accepted events and `bodies_saved: false`; submitted `secret-value` and `person@example.test` strings were absent.
- The documented default allowance is 20 accepted requests per rolling second (covered by the exact `@claim:rate-limit` regression); the explicit one-request allowance above was independently observed end to end.

## Live deployment, privacy, accessibility, and performance

- Fetched live `main-D6KaQv_k.js` and `main-DHXSyaAv.css` and `cmp`-matched both to this candidate's `dist/site` outputs. The deployed footer reports `v0.1.0+d9231ed433c7`.
- Cold Playwright request log contained only the origin document, JS, CSS, and local `drain-console.webp`; no cookies, localStorage, or sessionStorage writes occurred during demo/reset. CSP permits only `self` for connections/scripts/styles/images.
- `/`, `/demo`, `/privacy`, `/terms`, and `/missing` were exercised at 1440×900 and 390×844. Each rendered title, one `h1`, and one `main`; Axe found **zero serious or critical violations**. Normal routes had no console or page errors.
- On a 390 px viewport, the first-screen action was fully visible (`y=423.45`, height `46.80`) with no horizontal overflow. Keyboard Enter moved from the sample action to the demo and focused its `h1`; Back focused the home `h1`. The visible focus ring is a 3 px amber outline. Reduced-motion mode reduces banner/body animation duration to `0.00001s`.
- Live headers include HSTS, `X-Content-Type-Options: nosniff`, strict referrer policy, and a restrictive CSP with `frame-ancestors 'none'`. Hashed JS/CSS have `max-age=31536000, immutable`; HTML has a short 30-second revalidation cache. `/missing` correctly returns HTTP 404.
- Mobile Lighthouse report: Performance **99**, Accessibility **100**, Best Practices **100**, SEO **100**; FCP 1.1 s, LCP 1.3 s, CLS 0, TBT 90 ms. Lighthouse emitted its report before a Chromium target-crash message during teardown; independent Playwright runs were clean.

## Defects by severity

- Critical: none.
- High: none.
- Medium: none.
- Low: none.

The browser reports the expected network-console message when directly loading
the intentionally HTTP-404 `/missing` document. This is a consequence of the
required real 404 response, not a product runtime error; all normal routes are
console-clean.
