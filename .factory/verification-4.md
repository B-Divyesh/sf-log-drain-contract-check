# Independent verification 4 — PASS

- Candidate and local `HEAD`: `2cf3ea448de314acd60bab363d16492f60390639`
- Live URL: <https://log-drain-contract-check.sociobot.in>
- Verified: 2026-08-29 UTC
- Environment: Node 22.23.2, npm 10.9.8, Rust/Cargo 1.98.0, Playwright 1.58.2, Lighthouse 12.8.2.
- Decision: **PASS — release candidate accepted.**

This is a fresh independent rerun after the earlier failure reports. The requested commit is present locally and the live deployment matches its fresh static build: the live hashed JS, CSS, and hero WebP SHA-256 values equal `dist/site`; the live footer identifies `v0.1.0+2cf3ea448de3`.

## Mandatory claims gate

`.factory/claims.json` exists and contains 11 claims. After `npm ci` (96 packages, 0 vulnerabilities), every literal command in it was run in file order from this checkout before broader QA. All passed.

| Claim | Result | Observable proof |
| --- | --- | --- |
| `sample-demo` | PASS | One click opens `/demo` with 3 events, 17 paths, 558.1 KiB/7 days, 2.3 MiB/30 days, banner, and empty browser storage. |
| `local-only` | PASS | Browser flow is same-origin/no storage; Rust regression proves loopback binding. |
| `discard-default` | PASS | Accepted unique values are absent from the report. |
| `contract-report` | PASS | Sample report proves fields/types/findings and retention values. |
| `false-positive-controls` | PASS | Custom sensitive key and exact-path suppression work. |
| `rate-limit` | PASS | Receiver returns 429 and `Retry-After: 1` after the configured threshold. |
| `request-recovery` | PASS | Bad/incomplete requests return 400 while accepted events remain. |
| `explicit-save` | PASS | Only accepted bodies are saved when explicitly requested; colliding paths are rejected. |
| `interrupt-report` | PASS | SIGINT writes a report and exits successfully. |
| `portable-demo` | PASS | Bundled demo runs outside the repository and writes a unique temp report. |
| `mit-license` | PASS | Shipped `LICENSE` contains the MIT grant. |

## Cold first-read and demo

**PASS.** A cold live desktop and 390 px mobile visit says, in plain words:

- What: “Inspect a log drain before forwarding.”
- For whom: platform teams needing volume, fields, and privacy risks before leaving a drain on.
- First action: “Try it with sample data,” followed immediately by “Opens a sample report. Nothing is saved.”

The action is in the first 390×844 viewport. Keyboard activation opens a populated `/demo`; its persistent banner says “Demo — sample data, nothing is saved” and includes Reset demo and Start for real. Reset is operable with Space. No data is written in the demo context.

## Product exercise: CLI and receiver

- `cargo package --locked` produced the ready crate (10 files, 57.8 KiB / 16.3 KiB compressed).
- Installed `target/package/drain-check-0.1.0` into a fresh Cargo root and ran it from a separate temporary consumer directory.
- `drain-check --version` reports 0.1.0; `--help` documents the commands and flags.
- `drain-check demo --json` returned 3 events, 17 fields, 3 findings, 558.1 KiB/7 days, 2.3 MiB/30 days, and `bodies_saved: false`; it wrote `/tmp/drain-check-demo-*/report.json`, outside the repo.
- `inspect examples/drain.ndjson --sample-seconds 1` wrote a correct report. Boundary/invalid input checks reject `--sample-seconds 0` and `forwarding --url file:///tmp/out` with actionable Clap errors.
- A live local listener configured `--rate-limit 1` returned `202 Accepted` for one NDJSON POST and `429 Too Many Requests` plus `Retry-After: 1` for the second. Its report has one event, `bodies_saved: false`, and neither POST value appears in the report.
- The documented default allowance is covered by the claim regression: **20 requests per rolling second**; the next request receives 429 plus `Retry-After: 1`.
- Unit/integration recovery tests cover malformed JSON, short declared bodies, wrong methods, size limits, accepted-event preservation, explicit-save behavior, collision rejection, and Ctrl-C completion.

This product has no hosted API, authentication, Entra sign-in, account/persistence boundary, payments/unlock endpoint, service worker, or offline claim. Those checks are not applicable. The only server endpoint is the local loopback receiver above.

## Local quality gates

All passed on `2cf3ea448de314acd60bab363d16492f60390639`:

```text
npm ci
npm test                                      # 25/25
npm run typecheck
npm run build                                 # exact site production build; dist/site
cargo test --all-targets --all-features --locked  # 15/15
cargo test --doc --locked                     # 1/1
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo doc --no-deps --all-features
cargo build --release --locked
cargo package --locked
```

Fresh site build sizes: JavaScript 10.65 kB (4.14 kB gzip), CSS 6.56 kB (2.18 kB gzip), and hero WebP 62.2 kB. These are below the static-product budgets.

## Live deployment, privacy, accessibility, and performance

- Live `/`, `/demo`, `/privacy`, and `/terms` return 200; designed `/missing` returns HTTP 404. Crawled landing-page internal links all return 200.
- Playwright + Axe found **zero serious/critical findings** on `/`, `/demo`, `/privacy`, `/terms`, and `/missing` at 1440×900 and 390×844. Each has exactly one H1 and main landmark; 390 px layouts have no horizontal overflow.
- Normal live routes have no browser console or page errors. The expected failed-document console message is emitted only when intentionally requesting the HTTP 404 route.
- Keyboard Tab exposes a 3 px amber focus ring (including primary action); the skip link works. Reduced-motion fallback reports a 0.01 ms duration and disables the meaningful animation.
- Full landing → demo → reset flow records only same-origin requests: document, same-origin JS/CSS, and local hero image. Cookies, localStorage, and sessionStorage remain empty.
- Browser response headers: CSP restricts every source to self and includes `frame-ancestors 'none'`; HSTS, `X-Content-Type-Options: nosniff`, and `Referrer-Policy: strict-origin-when-cross-origin` are present. Hashed JS/CSS return `Cache-Control: public, max-age=31536000, immutable`; HTML revalidates at 30 seconds.
- Lighthouse mobile: Performance **100**, Accessibility **100**, LCP **1.3 s**, CLS **0**, total transfer **68 KiB**.

## Defects by severity

None found. There are no release-blocking gaps.
