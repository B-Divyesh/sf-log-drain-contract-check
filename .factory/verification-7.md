# Independent verification 7 — FAIL

**Candidate:** `7f4d941baa0e0608abac34f70d430031b1b2ac00`  
**Live URL:** <https://log-drain-contract-check.sociobot.in>  
**Verified:** 2026-08-29 UTC  
**Environment:** Node 22.23.2, npm 10.9.8, Rust/Cargo 1.98.0, Playwright 1.58.2

## Decision

**FAIL — do not release this candidate.**

The core CLI works, the one-click demo works, all declared commands return
zero, and the live deployment is byte-for-byte the candidate build. The
candidate nevertheless misses the supplied release contract in three areas:
several claim tests do not prove their full promises, mobile touch targets do
not meet the required size/spacing baseline, and required release documentation
is inconsistent or incomplete.

## Mandatory first checks

### Claims gate

`.factory/claims.json` exists. Before other product inspection, I ran every
literal `test` entry separately; all 18 exited `0`. I repeated the same 18
commands after `npm ci` in a separate clean clone at the candidate SHA
(`/tmp/drain-check-verify7-dqtEU2/repo`); all 18 again exited `0`.

| Claim | Result |
| --- | --- |
| `sample-demo`, `local-only`, `discard-default`, `contract-report` | command passes |
| `forwarding-config`, `source-checkout`, `false-positive-controls`, `rate-limit` | command passes |
| `request-recovery`, `explicit-save`, `separate-output-paths`, `minimum-sample-duration` | command passes |
| `json-stdout`, `complete-help`, `interrupt-report`, `portable-demo` | command passes |
| `site-build-output`, `mit-license` | command passes |

Exact per-command exits and logs are in
[`verification-evidence/claims-clean/results.tsv`](verification-evidence/claims-clean/results.tsv).
Green commands do not make this claims contract valid: the high-severity
findings below identify promises their sole tagged tests do not assert.

### Cold first read

**PASS.** A fresh 1440 px browser context immediately answers all three
questions:

- What: “Inspect a log drain before forwarding.”
- Who: platform teams checking volume, field types, and sensitive data.
- First click: **Try it with sample data**, with the adjacent explanation that
  it opens the bundled report and writes no browser data.

The action is also fully visible in the first 390×844 viewport at y=423.45–470.25.
One keyboard-activated click opens `/?demo=1`, focuses “Review this drain
sample,” and shows the populated report plus the persistent demo banner,
Reset demo, and View local setup. See
[`live-cold-first-read.json`](verification-evidence/live-cold-first-read.json)
and [`live-cold-desktop.png`](verification-evidence/live-cold-desktop.png).

## Defects by severity

### Critical

None.

### High — release blocking

1. **The tagged `request-recovery` test permits a response that violates its
   claim.** The claim says incomplete requests return HTTP 400. In
   `receiver_rejects_bad_requests_and_keeps_prior_events`, the incomplete-body
   assertion is `short.is_empty() || short.starts_with("HTTP/1.1 400")`.
   Therefore the required claim command stays green when the receiver returns
   no HTTP response. This is the exact behavior the earlier broken receiver
   exhibited. My installed-binary probe gets a real 400, so this is missing
   contract proof rather than a current runtime failure.

2. **The quantitative default rate-limit claim is not exercised at its stated
   allowance.** The tagged test parses the CLI default as 20, then starts a
   different receiver configured at one request per second and checks its
   second request. It never sends request 21 to the default configuration, as
   required for a quantitative claim. Independent end-to-end evidence confirms
   the implementation works: after two accepted setup/recovery requests, a
   concurrent 25-request burst produced 18 additional 202s and seven 429s,
   each with `Retry-After: 1`; the report contained 20 events.

3. **The `contract-report` test does not prove the complete report claim.** Its
   only Cargo regression checks event count, average size, total path count,
   retention displays, and one finding path. It never asserts the claimed
   event rate or any reported field type, and does not establish the listed
   field paths. Independent inspection found the current values correct, but
   the sole tagged test can pass while those promised outputs regress.

4. **The false-positive-controls test does not exercise prefix suppression.**
   The claim promises exact-path and prefix suppression, while
   `supports_custom_patterns_and_explicit_suppression` checks only exact
   `$.request_id`. An installed-package probe confirmed that `$.request*`
   currently suppresses nested request findings, but the declared test does
   not protect that promise.

5. **The web sample claim does not assert its promised three-event metric.**
   `@claim:sample-demo` checks route/banner/storage, 17 paths, both retention
   numbers, and finding labels, but never asserts that the displayed event
   count is 3. The live page currently shows 3; the test would not catch a
   drift in that claim.

These violate the supplied rule that each claim's one tagged test must assert
the observable promised outcome, including quantitative values.

### Medium — release blocking under the supplied baseline

1. **Mobile touch-target geometry misses the required baseline.** At 390 px,
   the adjacent header targets **Demo** and **How it works** have a measured
   0 px edge-to-edge gap, below the required 8 px. The inline “public source
   repository on GitHub” target measures 299.2×43.8 px, below the required
   44 px height. The repository's own touch-size regression checks only header
   and footer links after setting 200% text, so it misses both conditions.
   Evidence: [`mobile-targets-focus.json`](verification-evidence/mobile-targets-focus.json).

2. **README does not explain how to deploy the product.** The definition of
   done requires README instructions for run, test, and deploy. README explains
   run/test/build and says `dist/site` is deployable, but provides neither the
   factory deployment command nor generic static-host deployment steps. The
   command exists only in the previous handoff.

3. **Published package identity conflicts with the changelog.** Cargo,
   `drain-check --version`, and the live footer all report `0.1.0`, while the
   newest changelog section is released as `0.1.1` rather than Unreleased.
   Consumers cannot tell which semantic version contains the listed fixes.

### Low

None.

The browser logs one expected failed-resource message when directly loading
the intentional HTTP 404 document. Normal routes are console-clean; this is
not counted as a defect.

## Clean-clone quality gates

After `npm ci` (96 packages, zero vulnerabilities), every available gate
passed in the clean clone:

| Gate | Result |
| --- | --- |
| `npm test` | PASS, 36/36 |
| `npm run typecheck` | PASS |
| `npm run build` | PASS; exact production build in `dist/site` |
| `cargo test --all-targets --all-features --locked` | PASS, 18/18 |
| `cargo test --doc --locked` | PASS, 1 doctest |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | PASS |
| `cargo package --locked` | PASS, 10 files, 58.3 KiB / 16.4 KiB compressed |
| `cargo doc --no-deps --locked` | PASS |
| `npm audit --audit-level=high` | PASS, 0 vulnerabilities |

Logs and exit codes are in
[`verification-evidence/clean-gates/results.tsv`](verification-evidence/clean-gates/results.tsv).

## Installed-package and end-to-end CLI exercise

I installed `target/package/drain-check-0.1.0` into a clean consumer root and
ran the binary from fresh directories outside the repository.

- `--help` lists all four commands and global help/version options; `--version`
  returns `0.1.0`.
- Two demo runs produced different `/tmp/drain-check-demo-*` directories and
  exact bundled metrics: 3 events, 17 paths, 558.1 KiB/7 days, 2.3 MiB/30 days.
- Normal inspection reports types, event presence for arrays, sensitive-field
  findings, exact JSON stdout/file parity, and no submitted values in reports.
- Empty input returns a zero-event report. Malformed input and missing files
  exit 1; zero sample duration, port zero, rate zero, and invalid URLs exit 2.
- A URL containing a space is safely rendered as `%20`; FTP is rejected.
- The listener binds to `127.0.0.1` (verified through `/proc/net/tcp`). Valid,
  malformed, truncated, oversized, empty, and GET cases returned 202, 400,
  400, 413, 400, and 405 respectively; a following valid request returned 202.
- The default rolling allowance is **20 accepted requests per second**. A
  single local client exceeded it and received 429 plus `Retry-After: 1`.
- Without `--save-sample`, reports contain no submitted values and
  `bodies_saved` is false. With it, only the accepted body is written; a
  malformed body is not persisted. A colliding report/sample path is rejected
  without changing the existing file.

Machine-readable evidence is in
[`installed-cli-qa.json`](verification-evidence/installed-cli-qa.json) and
[`untested-claim-behaviors.json`](verification-evidence/untested-claim-behaviors.json).

## Live deployment, privacy, security, and identity

- Public GitHub `main` resolves to the candidate SHA.
- The live footer reports `v0.1.0+7f4d941baa0e`.
- Live HTML, 404 HTML, JS, CSS, hero/OG art, icons, robots, and sitemap all
  SHA-256/cmp-match the clean candidate build. Evidence:
  [`all-artifacts.json`](verification-evidence/live-match/all-artifacts.json).
- The browser demo flow made four requests, all to the product origin. It began
  with empty local/session storage and no cookies. Reset removed only
  `demo:drain-check` and preserved a real-data sentinel. There are no analytics,
  third-party scripts/fonts, authentication, billing, AI, or unlock calls.
- Headers include HSTS, `nosniff`, strict referrer policy, and a restrictive
  CSP with `connect-src 'self'` and `frame-ancestors 'none'`. HTML revalidates
  after 30 seconds and returned 304 to `If-None-Match`; hashed JS/CSS use
  `max-age=31536000, immutable`.
- Every live/site link resolves as expected; the designed `/missing` route is
  a real 404. `robots.txt` and `sitemap.xml` return 200.

The product is a local CLI with a static companion site, not a PWA or hosted
backend. Service-worker update/offline checks, hosted health/persistence, and
Entra sign-in checks are not applicable. The local receiver's concurrency,
persistence boundaries, recovery, and required 429 behavior were exercised.

## Accessibility, responsive behavior, and performance

- Factory `verify-url.sh` passes for home and direct demo.
- `/`, both demo URLs, `/privacy`, `/terms`, and `/missing` were run at
  1440×900 and 390×844. Each has one h1/main, route-specific metadata, no
  horizontal overflow, and **zero Axe violations of any severity**.
- Keyboard navigation reaches a visible skip link first. Enter on the sample
  action and browser Back focus the new/restored h1. Space activates Reset.
  Focus is a visible 3 px amber outline with 4 px offset; no trap was found.
- Reduced motion computes animation/transition duration to `0.00001s`.
  Simulated 200% root text retains all content without horizontal overflow.
- Initial transfer is 70,350 bytes. Raw/gzip sizes: JS 11,332/4,290 bytes,
  CSS 6,712/2,222 bytes, fonts 0, hero 62,236 bytes. All byte budgets pass.
- Fresh live mobile Lighthouse 12.8.2: Performance 99, Accessibility 100,
  Best Practices 100, SEO 100; FCP 1.0 s, LCP 1.3 s, TBT 140 ms, CLS 0.

Evidence: [`live-browser-audit.json`](verification-evidence/live-browser-audit.json),
[`lighthouse-summary.json`](verification-evidence/lighthouse-summary.json), and
[`live-transfer.json`](verification-evidence/live-transfer.json).

## Required remediation

1. Make every tagged claim test assert every promised subfeature and number,
   especially incomplete-request 400, the default 20-request limit, event
   rate/types/paths, prefix suppression, and the web demo's three events.
2. Give adjacent mobile navigation targets at least 8 px separation and make
   every interactive target at least 44×44 CSS px; expand the regression to all
   interactive elements at normal 390 px rendering.
3. Add README deployment steps and reconcile Cargo/site version `0.1.0` with
   CHANGELOG `0.1.1`.

No product code was modified during this verification.
