# Drain Check independent QA handoff

## Release status

**FAIL — do not release candidate `e6fac708cdc54e2d0edb8804bce588a7906968df`.**

Independent verification on 2026-08-29 confirms that <https://log-drain-contract-check.sociobot.in> matches the candidate, so this is not a deployment-only failure. The mandatory clean-clone claims gate fails: after `npm ci`, `npm test -- -t @claim:local-only` exceeds Vitest's five-second timeout and exits 1. A second isolated clone reproduced the failure. Ten other claim commands pass.

Full evidence and all other QA results are in [verification-2.md](verification-2.md).

## Blocking repair

Give Cargo-backed claim tests a realistic explicit timeout or compile the Rust target before the timed assertion. Then run every `.factory/claims.json` command from a new clone with no repository-local `target/` directory. A warm-cache pass is insufficient.

## What passed

- Cold first-read and one-click sample demo.
- Rust format, 13 tests, doctest, strict Clippy, TypeScript, production build, audit, release build, and Cargo package.
- Full 25-test browser suite after Cargo artifacts were warm.
- Installed CLI demo from an unrelated directory, invalid-input handling, false-positive controls, body-discard boundary, and SIGINT reporting.
- Independent receiver concurrency/recovery: default allowance observed as 20 accepted requests per rolling second, followed by 429 with `Retry-After: 1`.
- Live privacy request log, storage checks, security headers, immutable hashed-asset caching, and candidate/live byte equality.
- Desktop and 390 px mobile, keyboard-only flow, visible focus, 200% text, reduced motion, and zero serious/critical Axe findings.
- Lighthouse mobile: 99 performance, 100 accessibility, 100 best practices, 100 SEO; LCP 1.3 s, CLS 0, 68 KiB transfer.

## Verification commands

```sh
npm ci
jq -r '.[] | [.id,.test] | @tsv' .factory/claims.json
# Run every listed command individually before warming Cargo artifacts.

cargo fmt --all -- --check
cargo test --all-targets --all-features --locked
cargo test --doc --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
npm run typecheck
npm test
npm run build
cargo build --release --locked
cargo package --locked
```

No product code was changed during verification. Verification documentation and evidence are the only repository changes.
