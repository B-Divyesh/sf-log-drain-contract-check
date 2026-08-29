# Independent verification 3 — FAIL

- Requested candidate: `bd87644587b19278a7152595e9938866460e7aa9`
- Available and tested revision: `bd876445876521f0eb6568658e6edee683f67c8e`
- Live URL: <https://log-drain-contract-check.sociobot.in>
- Verified: 2026-08-29 UTC
- Environment: Rust/Cargo 1.98.0, Node 22.23.2, npm 10.9.8, Playwright 1.58.2, Lighthouse 13.4.1
- Result: **FAIL — do not release this candidate.**

The prior cold-claim timeout is repaired: all 11 declared claim tests pass after the lockfile install, including the cold Rust-backed `local-only` test in 29.76 seconds. This is not a deployment-only failure. The exact requested commit is absent from both the clone and its remote, the deployed build identifies a different commit, and independent installed-CLI testing found two release-blocking data/estimate defects plus invalid forwarding URL acceptance.

## Release-blocking findings

### Critical — the requested candidate does not exist and is not the deployed build

`git fetch --no-tags origin bd87644587b19278a7152595e9938866460e7aa9` returned exit 128 and `fatal: remote error: upload-pack: not our ref ...`. `git cat-file` also returned 128. Remote `HEAD` and `main` both resolve to `bd876445876521f0eb6568658e6edee683f67c8e`.

The live footer reports `v0.1.0+bd8764458765`, matching the available commit and differing from the requested candidate's `bd87644587b1` prefix. Live `index.html`, JS, CSS, hero WebP, favicon, `robots.txt`, and `sitemap.xml` are byte-for-byte equal to the production build made from the available revision. Therefore the deployed product can be tied conclusively to the available revision, but not to the requested candidate.

### High — `--save-sample` can silently lose the accepted bodies

The installed package accepts the same path for `--output` and `--save-sample`. In this run the POST returned 202 and the final report said `"bodies_saved": true`, but writing the report replaced the accepted NDJSON at that path:

```sh
drain-check listen --duration 1 --port <port> \
  --output "$same_path" --save-sample "$same_path" --json
# POST {"event":"saved"} -> HTTP 202
# final file is the JSON report; the accepted body is absent
```

This loses data after the operator explicitly asks to retain it and reports the opposite state. Reject colliding paths before binding, or make the two outputs collision-safe.

### High — a zero-second file sample produces a false zero-volume estimate

`inspect` accepts `--sample-seconds 0`, unlike `listen`, which rejects a zero duration. A non-empty one-event sample exits 0 and reports:

```json
{
  "events": 1,
  "sample_seconds": 0,
  "events_per_second": 0.0,
  "retention": [
    { "days": 7, "estimated_bytes": 0, "display": "0 B" },
    { "days": 30, "estimated_bytes": 0, "display": "0 B" }
  ]
}
```

Volume and retention are core outputs. A zero observation duration is invalid input and must be rejected rather than converted into a plausible but wrong estimate.

### Medium — malformed destinations are emitted as forwarding configuration

The validator rejects a missing scheme, but accepts invalid authorities and unescaped configuration content. Each of these exited 0 and printed a template:

```text
http://:
https://?query
https://example.com/"
```

The last case produces `url = "https://example.com/""`, which is not a valid quoted configuration value. Parse a real HTTP(S) URL and escape or reject output-breaking characters.

## Mandatory first checks

### Claims gate

`.factory/claims.json` exists with 11 entries. The literal commands were attempted before any other product check; as expected in a dependency-free clone, they initially returned 127 because `vitest` was not installed. After the required `npm ci` lockfile install (96 packages, zero vulnerabilities), every exact command passed in file order. The cold Rust compilation occurred in `local-only`, which completed inside its explicit 60-second timeout.

| Claim | Result | Exact evidence |
| --- | --- | --- |
| `sample-demo` | PASS | 1 passed; `/demo`, 3 events, 17 paths, 558.1 KiB/7 days, 2.3 MiB/30 days, empty storage; 2.77 s. |
| `local-only` | PASS | 1 passed; same-origin browser traffic/storage plus loopback regression; cold duration 29.76 s. |
| `discard-default` | PASS | 1 passed; unique values absent and no bodies saved by default; 3.08 s. |
| `contract-report` | PASS | 1 passed; exact rate, size, paths/types, findings, and retention; 2.46 s. |
| `false-positive-controls` | PASS | 1 passed; custom detector and exact-path suppression; 2.41 s. |
| `rate-limit` | PASS | 1 passed; 429 and `Retry-After: 1`; 2.41 s. |
| `request-recovery` | PASS | 1 passed; malformed/incomplete requests preserve accepted events; 3.02 s. |
| `explicit-save` | PASS | 1 passed for distinct output paths; 2.48 s. The path-collision case above is uncovered. |
| `interrupt-report` | PASS | 1 passed; SIGINT writes the report; 2.62 s. |
| `portable-demo` | PASS | 1 passed outside the repository; 2.43 s. |
| `mit-license` | PASS | 1 passed against the shipped MIT grant; 2.45 s. |

The landing page, demo, Privacy, Terms, README, and CLI help were cross-checked against the inventory. No other material capability or privacy claim was found without a corresponding claim entry.

### Cold first-read

PASS at desktop and 390×844.

- What it does: “Inspect a log drain before forwarding.”
- For whom: “platform teams” checking volume, fields, and privacy risks before leaving a drain on.
- What to click first: “Try it with sample data,” immediately qualified by “Opens a sample report. Nothing is saved.”
- The mobile action occupies y=423–470 in the first 844 px viewport.
- One keyboard-activated click opens a populated `/demo` and focuses “Review this drain sample.”
- The persistent banner says “Demo — sample data, nothing is saved” and exposes Reset demo and Start for real.

## Repository gates

All available gates pass on the available revision after `npm ci`:

```text
cargo fmt --all -- --check                                        PASS
cargo test --all-targets --all-features --locked                  PASS, 13 tests
cargo test --doc --locked                                         PASS, 1 doctest
cargo clippy --all-targets --all-features --locked -- -D warnings PASS
npm run typecheck                                                 PASS
npm test                                                          PASS, 25 tests
npm run build                                                     PASS, dist/site
npm audit --audit-level=high                                      PASS, 0 vulnerabilities
cargo build --release --locked                                    PASS
cargo package --locked                                            PASS, 10 files, 14.9 KiB compressed
```

The production output is 10.65 KiB JS (4.14 KiB gzip), 6.56 KiB CSS (2.18 KiB gzip), a 60.8 KiB hero, and a 33.7 KiB 1200×630 social image.

## Installed CLI and local receiver

The crate package was installed under a new temporary root, then run from a separate consumer directory.

- `--help` and `--version` work; version is 0.1.0.
- `demo --json` exits 0 from outside the repository with 3 events, 17 fields, 3 findings, 558.1 KiB/7 days, 2.3 MiB/30 days, `bodies_saved: false`, and a unique `/tmp/drain-check-demo-*/report.json`.
- Empty input returns a stable zero-event report. Malformed line 2 exits 1 and identifies line 2.
- Unknown commands, duration 0, port 0, and a scheme-less URL exit 2 with actionable errors.
- Custom `internal_code` detection works, exact `$.request_id` suppression works, and repeated array paths count once per event.
- Unique token and request ID values are absent from both stdout and the report.
- `/proc/net/tcp` showed the listener only at `0100007F` (`127.0.0.1`).
- A concurrent 21-request burst at the default allowance returned 20×202 and 1×429 with `Retry-After: 1`; the report contains exactly 20 events. **Observed allowance: 20 accepted requests per rolling second per receiver process.**
- Valid → malformed mixed NDJSON → short body → wrong method → 2 MiB + 1 declaration → valid returned 202 → 400 → 400 → 405 → 413 → 202; the process remained alive and the report contained exactly the two valid events.
- An exact 2 MiB valid body returned 202; a 2 MiB + 1 declaration returned 413. The report retained only aggregates.
- With distinct paths, explicit save rejected malformed input and saved the accepted body exactly. SIGINT wrote reports and exited 0.

This is the only server endpoint in the product. It has no hosted backend, tenant persistence, authentication, payment, or product-unlock endpoint. Entra, hosted health/build identity, and server-side tenant tests are not applicable.

## Live deployment, privacy, accessibility, and performance

- `/`, `/demo`, `/privacy`, and `/terms` return 200. The designed unknown route returns HTTP 404.
- The factory `verify-url.sh` passed in 639 ms: title, `lang=en`, one H1, main landmark, image alt text, labeled buttons, and no normal-load errors.
- Axe found zero serious/critical violations on all five routes at 1440×900 and 390×844.
- Normal routes produced no console or page errors. Chromium emitted only the expected failed-document message for the deliberately requested HTTP 404.
- All visible interactive targets measured at least 44×44 CSS px. No tested route overflowed at 390 px; 200% root text remained 390 px wide.
- Keyboard activation moves focus to the new H1. Reset demo works with Space. The tested primary focus ring is a visible 3 px amber outline with 4 px offset. No trap was observed.
- Reduced-motion emulation matched and left no running animation; computed fallback durations were 0.01 ms.
- The full landing → demo → reset → start-for-real → Privacy → Back flow made only same-origin requests. Cookies, localStorage, sessionStorage, IndexedDB, Cache Storage, and service-worker registrations remained empty.
- There are no analytics, remote fonts/scripts, AI calls, accounts, or sign-in. PWA offline/update checks are not applicable because the product has no service worker or offline claim.
- CSP includes `frame-ancestors 'none'`; HSTS, `nosniff`, and strict-origin referrer policy are present. HTML revalidates after 30 seconds. Hashed JS/CSS use `public, max-age=31536000, immutable`.
- Every live internal link crawled from the landing page returns 200.

Lighthouse 13.4.1 mobile results:

| Measure | Result |
| --- | ---: |
| Performance | 100 |
| Accessibility | 100 |
| Best practices | 100 |
| SEO | 100 |
| FCP | 1.0 s |
| LCP | 1.3 s |
| TBT | 0 ms |
| CLS | 0 |
| Total transfer | 68 KiB |

## Acceptance decision

**FAIL.** The prior claim-timeout defect is fixed and the available deployed revision is otherwise strong on its declared claims, privacy, accessibility, packaging, receiver recovery, rate limiting, and performance. Release remains blocked because the requested candidate cannot be resolved or matched to production, explicit saved bodies can be silently overwritten, and an accepted zero-second sample emits materially wrong volume estimates. Invalid forwarding destinations should be repaired in the same pass.
