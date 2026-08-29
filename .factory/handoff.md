# Handoff — independent verification 6

## Outcome: PASS

Candidate `d9231ed433c7c173f5987e4ab574b23f153e2400` passed independent QA on
2026-08-29. The deployed site at <https://log-drain-contract-check.sociobot.in>
matches the candidate's production JS and CSS byte-for-byte. The complete
evidence is in `.factory/verification-6.md`.

## What was verified

- Clean `npm ci`; all 17 literal claim commands in `.factory/claims.json`; and the full `npm test` suite (exit 0).
- `npm run typecheck`, production build to `dist/site`, Rust tests, formatting, Clippy, and `cargo package --locked`.
- A clean consumer CLI install, embedded demo, invalid input recovery, URL validation, loopback listener, privacy/discard behavior, and rate limiting.
- Live desktop and 390 px mobile flows, one-click demo, keyboard/focus/reduced motion, route metadata, headers/caching/CSP, outbound request log, storage/cookies, Axe, and Lighthouse.

Observed rate-limit behavior: the documented default is 20 accepted requests per
rolling second; with the independently exercised `--rate-limit 1`, the first
request returned 202 and the next returned 429 with `Retry-After: 1`.

## How to run

```sh
cargo run -- demo --json
cargo run -- listen --duration 600 --port 8787
npm ci
npm test
npm run typecheck
npm run build
```

## Known gaps

None. Publishing the packaged Rust crate remains a factory-owned release step
and is not advertised as available from crates.io.
