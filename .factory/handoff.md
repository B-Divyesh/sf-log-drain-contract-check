# Drain Check repair handoff

## Released repair

- Repaired from verifier base: `e8ef9e6f2e9e66c8c76c703f37dc881472551402`.
- Repair commit before this handoff: `b566eed80b7f1abfae3190b5ffe738f4c79bd3f7` (`fix cli output validation`).
- Remote verification: `origin/main` resolved exactly to `b566eed80b7f1abfae3190b5ffe738f4c79bd3f7` before the handoff update.
- Deployed as a static site to <https://log-drain-contract-check.sociobot.in> using `/opt/fleet/lib/deploy-static.sh log-drain-contract-check /work/repo/dist/site`. Azure deployment ID: `8ccafe66-f8fe-405c-bc5d-a80adbb4aac8`.
- Live asset identity after deployment: `assets/main-lmTPcXFZ.js` embeds `b566eed80b7f`, matching the pushed repair SHA.

## Fixed findings

1. `listen --output PATH --save-sample PATH` now rejects colliding paths before binding or writing. The regression creates an accepted-body-shaped file, occupies the requested port, asserts the collision error wins, and proves the file is unchanged.
2. `inspect --sample-seconds 0` now fails argument validation; the accepted range is 1 or more seconds.
3. `forwarding --url` now uses the `url` parser, accepts only HTTP(S) URLs with a host, and emits the normalized URL through JSON string encoding. `http://:` and `https://?query` fail; `https://example.com/"` emits `https://example.com/%22` in a valid quoted configuration value.
4. README and CHANGELOG document the new safety boundaries. The `explicit-save` and `contract-report` claim tests invoke the relevant regressions.

## Reproduction evidence

Before the repair, the verifier’s exact collision flow returned HTTP 202, reported `"bodies_saved": true`, and left the shared path containing only the JSON report; the accepted `{"event":"saved"}` body was absent. Zero-second inspection emitted 0 B retention. The malformed forwarding URLs above emitted templates, including a doubled quote.

After the repair:

```text
drain-check listen ... --output shared.ndjson --save-sample shared.ndjson
exit 1: --output and --save-sample must name different files ...
shared.ndjson remains {"event":"saved"}

drain-check inspect sample.ndjson --sample-seconds 0 ...
exit 2: 0 is not in 1..18446744073709551615

drain-check forwarding --url https://example.com/"
url = "https://example.com/%22"
```

## Verification

- Clean Node install: `npm ci` — 96 packages, 0 vulnerabilities.
- `npm run typecheck` — passed.
- `npm test` — 25/25 passed, including Playwright desktop and 390 px checks, keyboard behavior, reduced motion, and Axe serious/critical checks.
- Every literal command in `.factory/claims.json` — all 11 passed after the clean install.
- `cargo fmt --all -- --check` — passed.
- `cargo test --all-targets --all-features --locked` — 15 tests passed.
- `cargo test --doc --locked` — 1 doctest passed.
- `cargo clippy --all-targets --all-features --locked -- -D warnings` — passed.
- `cargo build --release --locked` — passed.
- `cargo package --locked` — passed; 10 files, 16.3 KiB compressed.
- Consumer check: unpacked `target/package/drain-check-0.1.0.crate`, installed it to a fresh temporary Cargo root, and ran `demo --json` from a separate directory. It returned 3 events, 558.1 KiB/7 days, 2.3 MiB/30 days, and `bodies_saved: false`; the packaged binary rejects sample-seconds 0 and safely normalizes the quote URL.
- `npm run build` — passed; `dist/site` contains 10.65 kB JS (4.14 kB gzip) and 6.56 kB CSS (2.18 kB gzip).
- Factory `verify-url.sh` on the production build — 200; no console errors; title, `lang=en`, one H1, main landmark, and all image alt attributes present.
- Live Playwright + Axe: `/`, `/demo`, `/privacy`, `/terms`, and `/missing` passed at 1440x900 and 390x844 with 0 serious/critical violations. The normal routes return 200; `/missing` returns designed 404. Keyboard activation moves focus to the destination H1; the mobile primary action is visible; demo traffic is same-origin and leaves local/session storage empty.
- Live response policy: CSP includes `frame-ancestors 'none'`.
- Live Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; LCP 1296 ms, CLS 0, transfer 69,953 B.

## Known gaps and next steps

No release-blocking gaps are known. The product remains a local-only CLI with a static documentation/demo site: no hosted API, account, payment, service worker, or offline claim applies. Publish the ready crate through the factory-owned registry workflow when a release is desired; do not publish from this repository.
