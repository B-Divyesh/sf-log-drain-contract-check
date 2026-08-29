# Independent verification 10 — PASS

**Candidate:** `cd28d94fe407243fa0e194cac7cbdc269ae75972`

**Live URL:** <https://log-drain-contract-check.sociobot.in>

**Verified:** 2026-08-29 UTC

**Environment:** Node 22.23.2, npm 10.9.8, Rust/Cargo 1.98.0,
Playwright 1.58.2, Chromium 145.0.7632.6, Lighthouse 12.8.2

## Decision

**PASS — release this candidate.**

Drain Check completes the researched job end to end. The packaged CLI runs a
bounded loopback receiver, reports volume, event rate, field paths and types,
flags likely sensitive data, supports reviewed suppressions, estimates
retention, and emits a validated forwarding template. Accepted bodies are
discarded unless the operator explicitly saves them.

No critical, high, medium, or low product defects were found. The five review-4
findings are closed: the demo exits to **View local setup**, generic-host
deployment wording is scoped correctly, control characters in `--platform`
are rejected, the privacy claim covers all named browser stores, and the UI
uses “field paths” and “receiver” consistently.

## Mandatory first checks

### Claims gate

`.factory/claims.json` exists and contains 18 claims. After the lockfile install
at the exact candidate, I ran every listed `test` command separately, in file
order, against the demo entry point. Each command selected exactly one tagged
test and passed.

| Claim | Result |
| --- | --- |
| `sample-demo` | PASS |
| `local-only` | PASS |
| `discard-default` | PASS |
| `contract-report` | PASS |
| `forwarding-config` | PASS |
| `source-checkout` | PASS; fresh public GitHub checkout built and ran |
| `false-positive-controls` | PASS |
| `rate-limit` | PASS |
| `request-recovery` | PASS |
| `explicit-save` | PASS |
| `separate-output-paths` | PASS |
| `minimum-sample-duration` | PASS |
| `json-stdout` | PASS |
| `complete-help` | PASS |
| `interrupt-report` | PASS |
| `portable-demo` | PASS |
| `site-build-output` | PASS |
| `mit-license` | PASS |

The landing page, legal pages, CLI help, and README contain no material
claim-like statement missing from this registry.

### Cold first read and one-click demo

**PASS.** In a fresh 390 × 844 browser context, without scrolling:

- What it does: **Check a log drain before forwarding**.
- For whom: platform teams checking volume, field types, and sensitive data.
- What to click first: **Try it with sample data**.
- What happens: “Opens the bundled report. Writes no browser data.”

The action is entirely inside the first viewport. One keyboard-activated click
opens `/?demo=1`, focuses **Review this drain sample**, and immediately shows 3
events, 0.005/sec, 17 field paths, 3 findings in 2 field paths, and 558.1 KiB /
2.3 MiB retention estimates. The persistent banner says **Demo — sample data,
nothing is saved** and exposes **Reset demo** and **View local setup**.

Evidence: `verification-10-artifacts/live-qa.json`, `mobile-home.png`, and
`mobile-_demo.png`.

## Defects by severity

- Critical: none.
- High: none.
- Medium: none.
- Low: none.

Chromium writes its expected failed-main-resource message when the intentionally
missing `/missing` route returns HTTP 404. All normal 200 routes and complete
user flows have zero console or page errors; this is not a product defect.

## Clean-checkout gates

The workspace was detached at the exact candidate. `npm ci` re-created
dependencies from the lockfile before product checks.

| Gate | Result |
| --- | --- |
| `npm ci` | PASS; 96 packages, 0 vulnerabilities |
| 18 literal claim commands | PASS; 18/18 |
| `npm test` | PASS; 40/40 |
| `npm run typecheck` | PASS |
| `npm run build` / exact production build | PASS; `dist/site` produced |
| `npm audit --audit-level=high` | PASS; 0 vulnerabilities |
| `cargo test --all-targets --all-features --locked` | PASS; 26/26 |
| `cargo test --doc --locked` | PASS; 1/1 |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --all-targets --all-features -- -D warnings` | PASS |
| `cargo doc --locked --no-deps` | PASS |
| `cargo package --locked` | PASS; 10 files, 69.1 KiB unpacked / 18.8 KiB compressed |

The 26 Rust targets comprise 6 library tests, 12 binary tests, and 8 release
tests. No separate web lint command exists; TypeScript, Rustfmt, and Clippy are
the repository's available static checks.

Production output is 11,325 bytes of JavaScript (4,283 gzip), 6,784 bytes of
CSS (2,222 gzip), no fonts, and a 62,236-byte hero WebP. All bundle budgets pass.

## Packaged CLI and public API

I installed `target/package/drain-check-0.1.3` into a new Cargo root and ran the
binary from an unrelated temporary directory.

- `drain-check --version` reports 0.1.3; help lists all four commands and both
  global help options.
- Two demo runs created different `/tmp/drain-check-demo-*` directories and
  produced the documented 3-event, 17-path report.
- Empty input produced a valid zero-event report. Malformed NDJSON exited 1 and
  did not create a report, so a partial input cannot be mistaken for success.
- A valid forwarding URL containing spaces and quotes was safely percent
  encoded. An FTP destination exited 2. Platform control characters are
  rejected by the release regression.
- A clean consumer crate using `drain_check::analyse_events` compiled and ran;
  its two-event fixture returned 4 paths and one deduplicated finding.
- The public library doctest passed.

### Receiver, recovery, concurrency, and allowance

Against the installed package, 10 parallel POSTs all returned 202. Malformed
JSON returned 400, an empty body returned 400, GET returned 405, a declared
body over 2 MiB returned 413, and a later valid request returned 202. The final
report contained exactly the 11 accepted events and `bodies_saved: false`.

The independent tagged rate-limit test observed the documented allowance:
**20 accepted requests per rolling second per local receiver**. Request 21
returned **429** with **`Retry-After: 1`**. The listener bound to
`127.0.0.1`; request failures did not end the sample window. The suite also
proved transactional malformed input, explicit-save behavior, output-path
collision refusal, oversized-header rejection, and Ctrl-C report creation.

## Live deployment, privacy, and build identity

The live footer and JavaScript expose `v0.1.3+2462ee3ba365`. That commit is a
later descendant of the candidate containing only factory documentation and a
QA script. `git diff cd28d94..2462ee3` reports zero changes in product, build,
package, README, license, or changelog files.

The candidate build and live deployment therefore match:

- CSS and hero artwork are byte-identical.
- HTML and 404 HTML are byte-identical after normalizing the hashed JavaScript
  filename.
- JavaScript is byte-identical after normalizing its 12-character build token;
  the only concrete difference is `cd28d94fe407` versus `2462ee3ba365`.

A cold landing → demo → reset → exit → Back flow requested only the same-origin
HTML, hashed JS, hashed CSS, and hero image. It created no cookies, session
storage, IndexedDB databases, Cache Storage entries, service workers, or OPFS
entries. Reset removed the seeded `demo:drain-check` key while preserving a
seeded `real:qa` sentinel. Source and runtime inspection found no analytics,
telemetry, third-party fonts/scripts, authentication, billing, AI calls, or
external API requests.

Live headers include HSTS, `X-Content-Type-Options: nosniff`,
`Referrer-Policy: strict-origin-when-cross-origin`, and a same-origin CSP with
`frame-ancestors 'none'`. HTML, non-hashed images, robots, and sitemap revalidate
after 30 seconds. Hashed JS/CSS cache for one year with `immutable`.

`/`, `/?demo=1`, `/demo`, `/privacy`, `/terms`, `robots.txt`, `sitemap.xml`,
icons, artwork, and the external public GitHub repository return 200.
`/missing` returns the designed 404. Evidence is in the five
`verification-10-artifacts/headers-*.txt` files and `live-qa.json`.

## Accessibility, mobile, keyboard, and performance

- The factory `verify-url.sh` passes: title, `lang=en`, one h1, main landmark,
  alt text, labeled buttons, and zero console errors on `/`.
- `/`, both demo URLs, `/privacy`, `/terms`, and `/missing` pass at 1440 × 900
  and 390 × 844 with no horizontal overflow.
- Axe reports zero serious or critical findings across all 12 route/viewport
  combinations; in fact, it reports zero WCAG A/AA violations.
- Every visible mobile link and button is at least 44 × 44 CSS pixels. The skip
  link is first, Enter moves focus to main, and the designed focus ring is a
  visible 3 px amber outline. Route changes and browser Back focus the new h1.
- At 200% root text size, the 390 px layout retains all content and controls
  without horizontal overflow. Reduced motion leaves zero active animations.
- Normal routes have zero console and page errors. No PWA/offline claim is
  made, and no service worker is registered.
- Fresh mobile Lighthouse scores: Performance 100, Accessibility 100, Best
  Practices 100, SEO 100. FCP is 0.8 s, LCP 1.2 s, TBT 60 ms, CLS 0, and first
  transfer is 68 KiB.

Evidence: `verification-10-artifacts/lighthouse-mobile.json`,
`live-qa.json`, `mobile-text-200.png`, and `verify-url/verify.json`.

The hosted product is a static documentation/demo site for a deterministic
local CLI. Hosted persistence, a service-worker update path, health endpoints,
sign-in/Entra, payments, and model-gateway tests do not apply. Adding model use
would conflict with the privacy-first preflight job and offers no necessary
capability here.

No product code was modified during verification.
