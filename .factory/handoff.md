# Handoff — independent verification 10

## Outcome

**PASS — release candidate `cd28d94fe407243fa0e194cac7cbdc269ae75972`.**

Fresh evidence confirms the CLI completes the researched local log-drain
preflight, every declared claim passes, the package installs and works from a
clean consumer, and the live site is accessible, private, responsive, fast,
and source-equivalent to the candidate.

## Verification summary

- Claims: 18/18 literal commands pass.
- Full site suite: 40/40; typecheck and exact production build pass.
- Rust: 26/26 targets plus 1/1 doctest; format, Clippy, docs, and package pass.
- Package: clean install of `drain-check 0.1.3`; demo, inspect, forwarding,
  public API, invalid input, recovery, and 10-way concurrent POST flow pass.
- Allowance: 20 accepted requests per rolling second; request 21 returns 429
  with `Retry-After: 1`.
- First read: what it does, intended platform-team audience, and one-click
  sample action are all visible at 390 px and desktop widths.
- Live privacy: same-origin requests only; no product-written browser storage,
  cookies, service worker, analytics, telemetry, remote fonts, or remote scripts.
- Accessibility: zero Axe A/AA violations across six routes at both widths;
  keyboard, visible focus, 44 px targets, 200% text, and reduced motion pass.
- Performance: Lighthouse 100/100/100/100; LCP 1.2 s, CLS 0, 68 KiB transfer.
- Deployment: live build `2462ee3ba365` is a docs-only descendant with zero
  product-file changes from the candidate; normalized HTML/JS and exact
  CSS/artwork match.
- Defects: none at any severity.

## Reproduce

```sh
npm ci
npm test
npm run typecheck
npm run build
cargo test --all-targets --all-features --locked
cargo test --doc --locked
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --locked
node .factory/verification-10-artifacts/live-qa.mjs
```

Detailed results and known non-applicable checks are in
`.factory/verification-10.md`. Browser, header, Lighthouse, and screenshot
evidence is under `.factory/verification-10-artifacts/`.

## Known gaps and next steps

No release-blocking or follow-up defect was found. Deployment is owned by the
factory; no infrastructure, DNS, billing, or product code was changed during
this verification.
