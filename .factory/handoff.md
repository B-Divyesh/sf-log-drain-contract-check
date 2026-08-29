# Handoff — independent verification 7

## Outcome: FAIL

Candidate `7f4d941baa0e0608abac34f70d430031b1b2ac00` was independently
verified on 2026-08-29 against
<https://log-drain-contract-check.sociobot.in>. **Do not release this
candidate.** Full evidence and severity detail are in
[`verification-7.md`](verification-7.md).

## Release blockers

- Several sole tagged claim tests do not prove their complete promises. Most
  importantly, the incomplete-request regression accepts an empty response
  even though the claim requires HTTP 400. The default 20-request limit,
  report event-rate/types, prefix suppression, and the demo's three-event
  metric are also not fully asserted by their named claim tests.
- At 390 px, the Demo and How it works navigation targets have 0 px spacing;
  the GitHub source link is 43.8 px high. The supplied baseline requires 8 px
  target spacing and 44×44 px targets.
- README lacks deployment instructions, and Cargo/site report version 0.1.0
  while CHANGELOG's newest released section is 0.1.1.

## What passed

- Mandatory cold read and one-click sample demo.
- Every one of 18 literal claims commands, both initially and after `npm ci`
  in a clean clone at the candidate SHA.
- `npm test` (36/36), typecheck, exact production build, 18 Rust tests,
  doctest, formatting, Clippy with warnings denied, package verification,
  docs, and npm audit.
- Installed-package CLI use from a clean consumer: demo, inspect, forwarding,
  invalid inputs, request recovery, explicit-save boundaries, interrupt, and
  output-path protection.
- Default receiver allowance observed end to end: 20 accepted requests per
  rolling second, then HTTP 429 with `Retry-After: 1`.
- Live/candidate identity: all public build artifacts match byte-for-byte and
  the footer reports `7f4d941baa0e`.
- Same-origin-only browser traffic, empty product storage/cookies, security
  headers, 304 revalidation, immutable hashed assets, all links, and real 404.
- Desktop/390 px route checks: zero Axe violations, no normal console errors,
  correct focus and reduced motion, no overflow. Lighthouse: 99/100/100/100;
  LCP 1.3 s, TBT 140 ms, CLS 0. Initial transfer: 70,350 bytes.

## How to reproduce

```sh
npm ci
jq -r '.[] | .test' .factory/claims.json
npm test
npm run typecheck
npm run build
cargo test --all-targets --all-features --locked
cargo test --doc --locked
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo package --locked
```

Per-command logs, browser screenshots, route/Axe output, headers, Lighthouse,
artifact hashes, and installed-CLI results are under
[`verification-evidence/`](verification-evidence/).

No product code was changed. Next work should repair the claim tests, touch
geometry, README deployment guidance, and version mismatch, then rerun all 18
claim commands from a new clean clone before reconsidering release.
