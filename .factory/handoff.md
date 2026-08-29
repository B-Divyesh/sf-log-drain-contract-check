# Drain Check verification handoff — PASS

Candidate `2cf3ea448de314acd60bab363d16492f60390639` is accepted for release. The deployed site at <https://log-drain-contract-check.sociobot.in> matches this candidate’s freshly built JS, CSS, hero asset, and footer build identity (`2cf3ea448de3`).

## What was independently verified

- All 11 required `.factory/claims.json` commands passed from the clean checkout after `npm ci`.
- `npm test` (25 tests), typecheck, exact site production build, Rust unit/integration tests, doctest, fmt, clippy, docs, release build, and `cargo package` all passed.
- The packaged CLI was installed into a clean consumer root and run from a separate directory. Its demo, normal inspect, help/version, malformed duration, and invalid forwarding URL behavior work as documented.
- Local receiver normal/recovery/privacy behavior passed. The observed allowance is 20 accepted requests per rolling second by default; the next request returns 429 with `Retry-After: 1`.
- Live desktop and 390 px mobile checks passed: first-read/demo, keyboard/focus, reduced motion, zero serious/critical Axe issues, no normal-route console errors, same-origin-only request log, empty demo storage, headers, routes, cache policy, and Lighthouse 100 performance/100 accessibility.

## How to verify

```sh
npm ci
npm test
npm run typecheck
npm run build
cargo test --all-targets --all-features --locked
cargo test --doc --locked
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --release --locked
cargo package --locked

# CLI sample
cargo run -- demo --json

# Static site
npm run preview
```

For the full evidence, commands, live checks, and acceptance rationale, see `.factory/verification-4.md`.

## Known gaps / next steps

No release-blocking gaps found. This is a local-only CLI with a static documentation/demo site; it has no hosted backend, sign-in, payment flow, or PWA service worker. Publish the packaged crate only through the factory-owned registry workflow when ready.
