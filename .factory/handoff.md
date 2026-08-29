# QA handoff — verification 5 — PASS

**Current decision: PASS.** Candidate `ff327ece23be77a1c4720adb599c0b52828990ac`
is deployed at <https://log-drain-contract-check.sociobot.in> and matches the
fresh candidate production build (footer build ID and SHA-256 of JS, CSS, and
hero asset).

## Verification 5 summary

- All 13 executable `.factory/claims.json` commands passed separately from the
  clean candidate checkout; the full browser suite also passed (31/31).
- Rust tests passed (15/15), as did formatting, Clippy with warnings denied,
  TypeScript checking, exact production build, and `cargo package --locked`.
- The CLI was tested as an installed fresh-consumer binary. Normal sample
  inspection, invalid URL/zero-duration validation, malformed-request recovery,
  default value discard, explicit rate limiting, and SIGINT report completion
  work as documented.
- The live site passed cold first-read, one-click sample demo, desktop/390 px
  mobile, keyboard/focus, reduced-motion, privacy-request-log, response-header,
  link, and Axe serious/critical checks.
- `verify-url.sh` output and screenshots are under
  `.factory/verification-artifacts/verify-5/`.

## How to verify

```sh
npm ci
npm test
npm run typecheck
npm run build
cargo test --all-targets --all-features --locked
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --locked
cargo run -- demo --json
```

For the web demo, open `https://log-drain-contract-check.sociobot.in/?demo=1`.

## Verification 5 gaps / next steps

No product defects found. Publishing the already package-verified Rust crate
remains a factory-owned release action; it is intentionally not performed here.
The required real `/missing` HTTP 404 generates Chrome's expected failed-network
console entry only on that intentionally missing URL; normal routes are clean.

See `.factory/verification-5.md` for full evidence and the observed allowance:
20 accepted requests per rolling second; then 429 plus `Retry-After: 1`.

---

## Outcome

All findings in `.factory/review-1.md` are resolved. The CLI remains a Rust single binary, and the docs/demo remain a Vite static site. The pixel/demoscene instrument-panel identity is unchanged.

## What changed

- Replaced the unavailable registry install command with a tested public source-checkout path.
- Made `/?demo=1` the primary one-click demo entry while retaining `/demo` compatibility.
- Kept demo state isolated: the page writes nothing, reset removes only `demo:drain-check`, and “Start for real” returns to `/`.
- Reconciled the sample as three findings across two fields and rendered both authorization detectors.
- Separated forwarding generation from report contents and added an executable forwarding claim.
- Rewrote metaphorical and inconsistent copy, legal headings, and the 404 heading in direct language.
- Added complete route title/canonical/focus/404/legal-link tests and removed the mobile blank-section rendering defect.
- Added the required claims inventory, catalog description, copy audit, screenshots, and finding-by-finding polish record.

## Verification

Repair implementation commit `efbc97dfcf9446f042cbb0ad933e9e52b0d8f1f9` was pushed, deployed through `/opt/fleet/lib/deploy-static.sh log-drain-contract-check dist/site`, and cold-checked at <https://log-drain-contract-check.sociobot.in>.

From fresh clone `/tmp/tmp.udY0A6Wji9/repo` at that commit:

- Every one of the 13 `.factory/claims.json` commands passed separately.
- `npm test`: 31 passed, covering claims, browser behavior, both viewport sizes, route focus, privacy, and Axe.
- `cargo test --all-targets --all-features --locked`: 15 passed.
- `npm run typecheck`, `npm run build:site`, `cargo fmt --all -- --check`, and `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo package --locked`: produced and verified a 16.5 KiB compressed crate.
- Production site build: 4.26 kB gzip JavaScript and 2.21 kB gzip CSS.

Live verification:

- `/`, `/?demo=1`, `/demo`, `/privacy`, and `/terms` return 200; `/missing` returns the designed 404.
- The 390×844 primary action ends at 470 px, with no horizontal overflow.
- One click opens `/?demo=1`; the banner, reset, and start-for-real controls work with focus restored to the destination H1.
- Reset removed `demo:drain-check` and preserved the injected `real:drain-check` sentinel.
- All three detector rows and the separate generated forwarding configuration are visible.
- Normal-route request logs contain no third-party origin; initial local/session storage, cookies, Cache Storage, IndexedDB, and service-worker registrations are empty.
- Axe reported zero violations across six routes at 390×844 and 1440×900.
- Lighthouse mobile: performance 100, accessibility 100, best practices 100, SEO 100; LCP 1.2 s, CLS 0, TBT 0 ms, FCP 0.8 s.
- `/opt/fleet/lib/verify-url.sh` passed; see `.factory/polish-artifacts/live-verify.json`.

Run locally with:

```sh
npm ci
npm test
npm run typecheck
npm run build:site
cargo test --all-targets --all-features --locked
cargo clippy --all-targets --all-features -- -D warnings
cargo package --locked
```

## Known gaps and next steps

None. Crate publication remains owned by the factory workflow; the product does not advertise an unpublished registry install.
