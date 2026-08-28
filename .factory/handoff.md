# Drain Check verification handoff

## Verdict

**FAIL — do not release candidate `fc83213d0b4bef15d7d049310d3b6245903bf418`.**

Tested live at <https://log-drain-contract-check.sociobot.in> on 2026-08-28 UTC. The deployment is healthy and its emitted HTML/assets match the candidate byte-for-byte, so this is not a deployment-only failure.

Full evidence and reproduction details are in [verification.md](verification.md).

## Release blockers

- An installed package cannot run `drain-check demo`; it depends on repository-relative `examples/drain.ndjson`.
- The web demo understates both retention estimates by about 100×.
- Invalid NDJSON is silently accepted as HTTP 202; an incomplete request terminates the receiver and loses the report.
- Raw bodies and parsed events remain in memory for the full window despite the discard-after-aggregation claim.
- No rate limit was observed through 100 rapid requests (all 202; no 429/`Retry-After`).
- The brief requires false-positive handling, but the CLI offers none.
- The mobile landing page has two serious Axe findings.
- `cargo fmt --check`, strict Clippy, and strict TypeScript checks fail.
- Claims coverage is incomplete, and the discard/no-save claim tests assert copy rather than behavior.

## What passed

- All three commands listed in `.factory/claims.json` pass after `npm ci`.
- The cold first screen plainly explains what the product does, who it serves, and what to click; its one-click sample demo opens successfully.
- `cargo test`, Rust release build, `npm test`, exact Vite production build, `cargo package`, and clean package installation pass.
- Normal inspect/receiver flows, JSON output, explicit sample saving, missing-file errors, and unknown-option exit behavior work.
- Live routes have correct titles/landmarks and no console errors or third-party requests.
- Lighthouse mobile: performance 99, accessibility 100, best practices 100, SEO 100; LCP 1.3 s, CLS 0.
- Initial JS/CSS/image transfer is well inside budget; npm audit reports no vulnerabilities.

## Verification commands

```sh
npm ci
npm test -- -t @claim:local-only
npm test -- -t @claim:discard-default
npm test -- -t @claim:sample-demo
npm test
npm run build
cargo test --all-targets --all-features --locked
cargo test --doc --locked
cargo build --release --locked
cargo package --locked
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

The product code was not modified during verification. Only this handoff and the independent verification report were added or updated.
