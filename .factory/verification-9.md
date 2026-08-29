# Independent verification 9 — PASS

**Candidate:** `fe76429af3a81f52fa103916afceeb71206df564`  
**Live URL:** <https://log-drain-contract-check.sociobot.in>  
**Verified:** 2026-08-29 UTC  
**Environment:** Node 22.23.2, npm 10.9.8, Rust/Cargo 1.98.0,
Playwright 1.58.2, Chromium 145.0.7632.6, Lighthouse 12.8.2

## Decision

**PASS — release this candidate.**

The original release blockers are fixed. Fresh tests confirm that Drain Check
measures received event bytes, keeps punctuated source keys distinct, detects
sensitive punctuated keys, rejects oversized headers, and exposes an accessible
wordmark. The candidate completes the brief's smallest useful job: run a local
bounded receiver, inspect volume/types/privacy risk, review false positives, and
generate a forwarding template without retaining bodies by default.

One low-severity input-hardening defect remains in the optional `--platform`
label. It does not affect the documented/default flow or any acceptance claim.

## Mandatory first checks

### Claims gate

`.factory/claims.json` exists with 18 entries. From the initially clean candidate
checkout, I ran `npm ci`, then every literal `test` command separately before
the broader suite. All 18 passed:

| Claim | Result |
| --- | --- |
| `sample-demo` | PASS |
| `local-only` | PASS |
| `discard-default` | PASS |
| `contract-report` | PASS |
| `forwarding-config` | PASS |
| `source-checkout` | PASS, including a fresh public GitHub checkout |
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

Each invocation selected exactly one tagged test. Raw logs are under
`/tmp/drain-check-qa/claim-*.log`; the result list is
`/tmp/drain-check-qa/claim-results.log`.

### Cold first read and one-click demo

**PASS.** A fresh 1440x900 browser context sees:

- what it does: **Inspect a log drain before forwarding**;
- for whom: platform teams checking volume, field types, and sensitive data;
- what to do first: **Try it with sample data**;
- what happens next: “Opens the bundled report. Writes no browser data.”

The action is fully visible at 390x844 (bottom edge 443.25 px). One click opens
`/?demo=1`, focuses “Review this drain sample,” and immediately shows 3 events,
0.005 events/second, 17 field paths, 3 findings across 2 fields, and 7/30-day
retention. The persistent demo banner includes **Reset demo** and **Start for
real**.

## Defects by severity

### Critical

None.

### High

None.

### Medium

None.

### Low

1. **`forwarding --platform` accepts newline/control content without escaping.**
   A shell argument such as `$'generic-http\nurl = "https://attacker.invalid"'`
   injects an uncommented line before the validated destination. This is local,
   operator-supplied input; the default and documented command are unaffected,
   and the URL itself remains validated and safely encoded. A future release
   should reject line breaks or comment each rendered line.

## Clean-checkout gates

I cloned the candidate with no shared working-tree artifacts into
`/tmp/drain-check-clean-9-Wwb3lF/repo`, detached at the candidate SHA, and ran:

| Gate | Result |
| --- | --- |
| `npm ci` | PASS; 96 packages, 0 vulnerabilities |
| `npm test` | PASS; 40/40 |
| `npm run typecheck` | PASS |
| `npm run build` | PASS; exact production output in `dist/site` |
| `npm audit --audit-level=high` | PASS; 0 vulnerabilities |
| `cargo test --all-targets --all-features --locked` | PASS; 25/25 |
| `cargo test --doc --locked` | PASS; 1/1 |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | PASS |
| `cargo doc --no-deps --locked` | PASS |
| `cargo package --locked --allow-dirty` | PASS; 10 files, 68.1 KiB unpacked / 18.5 KiB compressed |

No separate web lint script exists. TypeScript checking, Rustfmt, and Clippy are
the repository's available static checks. Logs and timings are in the clean
checkout root; `gates.tsv` records every zero exit.

## Packaged CLI and end-to-end behavior

I installed the packaged crate into a fresh Cargo root and ran it from an
unrelated temporary working directory.

- The installed 1,915,040-byte binary reports `drain-check 0.1.2`; `--help`
  lists `listen`, `inspect`, `demo`, and `forwarding`.
- `demo --json` reports 3 events, 17 paths, 3 findings, 558.1 KiB for 7 days,
  and 2.3 MiB for 30 days from any working directory.
- A mixed-type fixture preserved separate `$['http.method']` and
  `$.http.method` paths, separate `$['items[]']` and `$.items[]` paths, array
  element fields, Unicode, null/number/boolean types, and both configured
  sensitive-field findings.
- `--json` parsed identically to the written report. Empty input produced a
  valid zero-event/zero-retention report.
- Malformed NDJSON exited 1 and wrote no report. Zero sample duration and a
  non-HTTP forwarding URL exited 2 with actionable errors. A valid URL with
  spaces and quotes was percent-encoded in valid output.
- The packaged crate contains the source, embedded sample, README, changelog,
  MIT license, and lockfile. The doctested public library example passes.

### Receiver, recovery, concurrency, and allowance

Against the installed binary, 20 concurrent requests all returned 202. Request
21 in the same rolling second returned **429** with **`Retry-After: 1`**. After
1.1 seconds, another valid request returned 202.

The observed allowance is **20 accepted requests per rolling second per local
receiver**. The listener is bound to `127.0.0.1`.

The same running listener returned:

| Case | Status |
| --- | ---: |
| malformed JSON | 400 |
| declared body longer than sent body | 400 |
| GET | 405 |
| missing `Content-Length` | 400 |
| empty body | 400 |
| body over 2 MiB | 413 |
| valid body exactly 2 MiB | 202 |
| valid request after all errors | 202 |

Ctrl-C then exited zero and wrote a 23-event report: exactly the 20 concurrent
events plus the three later valid events. `bodies_saved` was false and unique
submitted values were absent from the report. A 32,768-byte HTTP header was
accepted; 32,769 bytes returned 431. Claim tests separately proved explicit
save behavior and that only accepted bodies are written.

## Live deployment, privacy, and identity

- `origin/main`, local HEAD, and the requested candidate all resolve to
  `fe76429af3a81f52fa103916afceeb71206df564`.
- Live `index.html`, designed 404 HTML, hashed JavaScript/CSS, artwork, icons,
  robots, and sitemap byte-match the clean candidate build. The live bundle and
  footer expose `v0.1.2+fe76429af3a8`.
- A complete cold landing/demo/reset/exit flow requested only four same-origin
  resources: HTML, hashed JS, hashed CSS, and the hero image. It created no
  cookies and no local/session storage. A seeded real-data sentinel survived
  demo reset, while only `demo:drain-check` was removed.
- Source and runtime inspection found no analytics, telemetry, remote fonts or
  scripts, authentication, billing, unlock calls, AI calls, or external API
  requests.
- Live HTML has `Cache-Control: public, must-revalidate, max-age=30` and returns
  304 for its ETag. Hashed JS/CSS have one-year immutable caching.
- Responses include HSTS, `X-Content-Type-Options: nosniff`,
  `Referrer-Policy: strict-origin-when-cross-origin`, and a same-origin CSP with
  `frame-ancestors 'none'`. No CSP errors occurred.
- `/`, `/?demo=1`, `/demo`, `/privacy`, `/terms`, `robots.txt`, and
  `sitemap.xml` return 200. `/missing` returns the designed 404. Every live link
  returns its intended status; the external public GitHub repository returns
  200.

## Browser, accessibility, and performance

- The factory `verify-url.sh` passes live `/` and `/?demo=1` with zero errors.
- `/`, both demo URLs, `/privacy`, `/terms`, and `/missing` were exercised at
  1440x900 and 390x844. Each has `lang=en`, one h1, one main landmark, a correct
  route title/canonical/description, ordered headings, and no horizontal
  overflow. The designed 404 emits only Chromium's expected main-resource 404
  console message.
- Axe reports zero serious/critical findings across all 12 route/viewport
  combinations. The experimental WCAG 2.5.3 label rule also reports zero.
- Every link/button is at least 44x44 CSS px. The skip link is first, focus is a
  visible 3 px amber outline, Enter opens the demo, Space resets it, and route
  changes/browser Back focus the new h1. No keyboard trap was found.
- At 200% root text size the 390 px demo has no horizontal overflow. Reduced
  motion removes the art animation and makes recording replay immediate.
- Normal 200 routes have zero console/page errors. No service worker or web app
  manifest is registered, and no offline/PWA claim is made.
- Production sizes: JS 11,309 bytes / 4,295 gzip; CSS 6,784 / 2,222 gzip;
  fonts 0; hero WebP 62,236 bytes. All supplied budgets pass.
- Fresh live mobile Lighthouse exited zero: Performance 100, Accessibility
  100, Best Practices 100, SEO 100; FCP 0.784 s, LCP 1.230 s, TBT 52 ms,
  CLS 0, total transfer 70,202 bytes.

The product is a deterministic local CLI with a static documentation/demo site.
Hosted persistence, health endpoints, Entra sign-in, billing, PWA update/offline
behavior, and AI gateway checks do not apply. AI would not improve this bounded,
deterministic contract analysis enough to justify sending log information to a
model.

No product code was modified during verification.
