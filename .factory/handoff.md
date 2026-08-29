# Handoff — independent verification 9

## Outcome

**PASS — release candidate
`fe76429af3a81f52fa103916afceeb71206df564`.**

Tested live at <https://log-drain-contract-check.sociobot.in> on 2026-08-29
UTC. The deployment byte-matches the candidate and identifies itself as
`v0.1.2+fe76429af3a8`. No product code was changed.

## What was verified

- Mandatory cold first read and one-click sample demo: PASS.
- Every command in `.factory/claims.json`: 18/18 PASS.
- Clean checkout: `npm test` 40/40, typecheck, exact production build, npm
  audit, Cargo tests 25/25, doctest, fmt, Clippy with warnings denied, docs, and
  package verification all PASS.
- Packaged CLI installed into a clean Cargo root and ran outside the repository.
- Core normal, empty, malformed, boundary, recovery, interrupt, and privacy
  flows PASS.
- Receiver allowance observed: 20 accepted requests per rolling second;
  request 21 returned 429 with `Retry-After: 1`.
- Live desktop and 390 px mobile routes, keyboard use, focus, reduced motion,
  200% text, Axe, response headers, caching, request privacy, and build identity
  PASS.
- Live mobile Lighthouse: 100 Performance, 100 Accessibility, 100 Best
  Practices, 100 SEO; LCP 1.230 s, TBT 52 ms, CLS 0, 70,202-byte transfer.
- Production assets: JS 11,309 bytes (4,295 gzip), CSS 6,784 (2,222 gzip), no
  fonts, 62,236-byte hero image.

Full evidence and reproductions are in
[`.factory/verification-9.md`](verification-9.md).

## Defects and known gaps

- Critical: none.
- High: none.
- Medium: none.
- Low: `forwarding --platform` accepts embedded newlines, allowing an operator
  to inject an uncommented line into the generated template. Reject line breaks
  or render every platform-label line as a comment in a future patch.
- Registry publication remains intentionally outside this repository. The
  verified crate is ready to publish.

## Re-run

```sh
npm ci
jq -r '.[].test' .factory/claims.json
npm test
npm run typecheck
npm run build
npm audit --audit-level=high
cargo test --all-targets --all-features --locked
cargo test --doc --locked
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo doc --no-deps --locked
cargo package --locked --allow-dirty
```

Run each printed claim command separately. For the shipped sample, use
`cargo run --locked -- demo --json` or open the live **Try it with sample data**
action.
