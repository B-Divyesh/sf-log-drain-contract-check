# Independent verification 2 — FAIL

- Candidate: `e6fac708cdc54e2d0edb8804bce588a7906968df`
- Live URL: <https://log-drain-contract-check.sociobot.in>
- Verified: 2026-08-29 UTC
- Environment: Rust/Cargo 1.98.0, Node 22.23.2, npm 10.9.8, Playwright 1.58.2, Lighthouse 13.4.1
- Result: **FAIL — do not release this candidate.**

The deployed product matches the candidate and its user-facing web and CLI behavior passed independent checks. This is not a deployment-only failure. The candidate fails the mandatory claims gate from a clean clone: `@claim:local-only` consistently exceeds Vitest's five-second test timeout while compiling its Rust regression. The supplied acceptance contract says any failing claim test is release-blocking.

## Release-blocking finding

### High — a required claim test fails from a clean clone

After `npm ci`, I ran every command in `.factory/claims.json` individually and in file order. Ten passed. The exact command `npm test -- -t @claim:local-only` failed after 28.060 seconds with:

```text
Test timed out in 5000ms.
FAIL site/e2e.test.ts > published claims > @claim:local-only makes no third-party request or browser storage write
```

I reproduced it in a second isolated clone at the candidate SHA after a fresh `npm ci`: the test took 28.381 seconds, failed against the same five-second limit, and exited 1. The test synchronously launches `cargo test --locked listener_binds_to_loopback` inside a Vitest test that has only the default five-second allowance. After Cargo artifacts were warm, the same test passed in 4.886 seconds and the full 25-test browser suite passed. A warm-cache pass does not satisfy the required clean-clone claim gate.

Evidence:

- `.factory/verification-artifacts/claim-tests-post-install.log`
- `.factory/verification-artifacts/clean-repro-local-only.log`
- `site/e2e.test.ts:44-56`

Required repair: give Cargo-backed claim tests a realistic explicit timeout or build the Rust target before the timed assertion, then rerun every claim from a new clone with no repository-local `target/` directory.

No additional release-blocking runtime defect was found.

## Mandatory first checks

### Claims inventory

`.factory/claims.json` exists and contains 11 claims. Exact post-install results:

| Claim | Result | Evidence |
| --- | --- | --- |
| `sample-demo` | PASS | One click opened `/demo`; exact 3 events, 17 paths, 558.1 KiB/7 days and 2.3 MiB/30 days; browser storage empty. |
| `local-only` | **FAIL** | Clean run timed out at 28.060 s against a 5 s limit; isolated-clone reproduction timed out at 28.381 s. |
| `discard-default` | PASS | Rust regressions passed; an independent unique value was absent from the output report. |
| `contract-report` | PASS | Exact fixture metrics, field/type summaries, findings, and retention values passed. |
| `false-positive-controls` | PASS | Custom `internal_code` was flagged while ignored `$.request_id` was absent; array presence remained 2 for 2 events. |
| `rate-limit` | PASS | Declared test passed; independent default-rate run observed 20×202 then 1×429 with `Retry-After: 1`. |
| `request-recovery` | PASS | Valid, malformed, incomplete, then valid traffic returned 202/400/400/202; final report retained both valid events. |
| `explicit-save` | PASS | Malformed input was not saved; accepted input was the only saved line and `bodies_saved` was true. |
| `interrupt-report` | PASS | SIGINT regression wrote a valid partial report and exited successfully. |
| `portable-demo` | PASS | Packaged/installed CLI demo ran outside the repository and used an unpredictable temporary report directory. |
| `mit-license` | PASS | Shipped license contains the MIT grant. |

The live landing, demo, Privacy, Terms, CLI help, and README claims map to this inventory. I found no material unlisted capability claim.

### Cold first-read

PASS on desktop and 390×844 mobile.

- What it does: “Inspect a log drain before forwarding.”
- For whom: platform teams checking volume, fields, and privacy risks before leaving a drain on.
- First click: “Try it with sample data,” followed by “Opens a sample report. Nothing is saved.”
- On mobile the action occupies y=423–470 in the initial 844 px viewport.
- One keyboard-activated click opens a populated `/demo`; focus moves to its H1.
- The persistent banner says “Demo — sample data, nothing is saved” and includes keyboard-operable Reset demo and Start for real actions.

Evidence: `.factory/verification-artifacts/live-first-read.json`, `live-mobile-home.png`, `live-mobile-demo.png`, and `live-qa.json`.

## Repository gates

After the clean claim run exposed the blocker, the following completed successfully:

```text
cargo fmt --all -- --check                                        PASS
cargo test --all-targets --all-features --locked                  PASS, 13 tests
cargo test --doc --locked                                         PASS, 1 doctest
cargo clippy --all-targets --all-features --locked -- -D warnings PASS
npm run typecheck                                                 PASS
npm test                                                          PASS, 25 tests with warm Cargo artifacts
npm run build                                                     PASS
npm audit --audit-level=high                                      PASS, 0 vulnerabilities
cargo build --release --locked                                    PASS, 1,542,216-byte binary
cargo package --locked                                            PASS, 10 files, 14.9 KiB compressed
```

The exact production build created `dist/site`. Its main JS is 10.65 KiB raw/4.14 KiB gzip and CSS is 6.56 KiB raw/2.18 KiB gzip.

## Installed CLI and receiver

I installed `target/package/drain-check-0.1.0` into a new temporary root, changed to a separate working directory, and exercised the installed executable.

- `--help` and `--version` work; the version is 0.1.0.
- `demo --json` exits 0 with 3 events, 17 fields, 3 findings, 558.1 KiB/7 days, 2.3 MiB/30 days, and `bodies_saved: false`. It prints a unique `/tmp/drain-check-demo-*/report.json` path.
- Empty standard input produces a valid zero-event report.
- Invalid NDJSON exits 1 with the faulty line identified.
- Invalid URL, duration 0, and port 0 exit 2 with actionable argument errors. A valid HTTP(S) URL produces the forwarding template.
- A custom sensitive-field pattern is honored; an exact ignored path is suppressed.
- A unique secret value never appears in the report, while its path and detector findings do.
- The listener is present only at `0100007F` (`127.0.0.1`) in `/proc/net/tcp`.
- A 21-request concurrent burst at the default allowance returned 20 HTTP 202 responses and one 429 with `Retry-After: 1`; the report contained exactly 20 events.
- Valid → malformed → incomplete → valid requests returned 202 → 400 → 400 → 202. The process stayed alive and wrote a two-event report.
- Without `--save-sample`, only aggregate report data was written. With it, a malformed body was excluded and the one accepted body was saved exactly.
- SIGINT ended each independent receiver run and wrote its report with exit 0.

Evidence: `.factory/verification-artifacts/consumer-*` and `.factory/verification-artifacts/listener-qa.json`.

## Live deployment, privacy, and security

- Root, `/demo`, `/privacy`, and `/terms` return 200. An unknown path returns the designed page with HTTP 404.
- Normal routes at 1440×900 and 390×844 produced no console/page errors. Chromium logs the expected failed-resource message for the deliberately requested 404 itself.
- The complete landing → demo → reset → start-for-real flow requested only the product origin and wrote no localStorage, sessionStorage, cookies, or IndexedDB-observable product state.
- No analytics, remote fonts, third-party scripts, authentication, billing, or AI calls exist. Entra sign-in checks are not applicable.
- Response headers include HSTS, `X-Content-Type-Options: nosniff`, `Referrer-Policy`, and a CSP with `frame-ancestors 'none'`. Hashed JS/CSS use `public, max-age=31536000, immutable`; HTML uses a 30-second revalidation policy.
- The product is a static site accompanying a local CLI, not a PWA or hosted backend. Service-worker update, offline reload, hosted health endpoint, and hosted persistence tests are not applicable.
- The local receiver is the only server endpoint. Its concurrency, persistence boundary, error recovery, and documented 20-request rolling-second allowance were tested above.

## Accessibility and responsive behavior

- Factory `verify-url.sh` passed in 702 ms: title, `lang=en`, one H1, main landmark, image alt text, labeled buttons, and zero normal-load errors.
- Axe found zero serious/critical violations on `/`, `/demo`, `/privacy`, `/terms`, and the 404 at both 1440 px and 390 px.
- Keyboard order is logical. Each tested focused control has a visible 3 px amber outline. Enter activates links; Space activates Reset demo; no trap was observed.
- Route changes focus the new H1 and update the live route status.
- All visible link/button targets measured at least 44×44 CSS px.
- At 200% root text size on 390 px, client and scroll widths both remained 390 px.
- Reduced-motion emulation produced no running animations; animation/transition durations computed to 0.01 ms.
- The mobile installation section uses `content-visibility`; it paints correctly when scrolled into view. No content is lost.

## Performance and deployment identity

Lighthouse 13.4.1 mobile profile:

| Measure | Result |
| --- | ---: |
| Performance | 99 |
| Accessibility | 100 |
| Best practices | 100 |
| SEO | 100 |
| FCP | 0.9 s |
| LCP | 1.3 s |
| TBT | 130 ms |
| CLS | 0 |
| Total transfer | 68 KiB |

The live HTML, hashed JS, hashed CSS, hero WebP, `robots.txt`, and `sitemap.xml` are byte-for-byte equal to the candidate's production build. Live footer identity is `v0.1.0+e6fac708cdc5`. This conclusively replaces the earlier deployment-only uncertainty: the candidate is deployed.

## Acceptance decision

**FAIL.** Runtime behavior, deployment, privacy, accessibility, package portability, and performance are otherwise release-ready, but the acceptance contract explicitly prohibits passing a candidate when any `.factory/claims.json` command fails from a clean clone. Repair and repeat the clean claim run before release.
