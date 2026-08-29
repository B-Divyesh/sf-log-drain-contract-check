# Handoff — independent verification 8

## Outcome: FAIL

Candidate `92f01451faec66b4dfe5ddab823b0b2e4ebaa26f` was independently
verified on 2026-08-29 against
<https://log-drain-contract-check.sociobot.in>. **Do not release it.** The live
deployment byte-for-byte matches the candidate, so this is not the previously
reported deployment-only condition.

## Release blockers

1. The CLI measures compact re-serialized JSON rather than received bytes. A
   valid 1,031-byte event reports 7 average bytes and underestimates one-day
   retention by 99.32%, outside the brief's 25% target.
2. Unescaped dotted/bracketed field names collide with nested/array paths, so
   the report merges distinct fields and types.
3. Dotted names also bypass sensitive-field matching. With
   `--sensitive-field customer`, the valid key `customer.id` produces no
   finding; built-in `password` likewise misses `password.hash`.
4. Explicit Axe/Lighthouse WCAG 2.5.3 analysis reports a serious accessible
   name/visible label mismatch on the wordmark.

Low-severity findings: the 32 KiB header guard accepts a 33 KiB header because
of read-chunk overshoot, and the demo exit says “View local setup” instead of
the prescribed “Start for real.”

Full evidence and remediation are in
[`verification-8.md`](verification-8.md).

## What passed

- Cold first read and one-click sample demo.
- All 18 `.factory/claims.json` commands after clean `npm ci`.
- Clean clone: 38/38 npm tests, TypeScript, exact production build, 19/19 Cargo
  tests, doctest, fmt, Clippy, package, docs, audit.
- Pack/install into a clean consumer; normal, empty, invalid, recovery,
  body-size, persistence, interrupt, JSON, and forwarding cases.
- Twenty concurrent requests. The documented allowance was observed as 20
  accepted requests per rolling second; request 21 returned 429 with
  `Retry-After: 1`.
- Live/candidate byte parity, same-origin/no-storage privacy, security headers,
  immutable hashed-asset caching, keyboard flow, 390 px layout, 200% text,
  reduced motion, and default Axe.
- Mobile Lighthouse: 100 Performance, 100 Accessibility, 100 Best Practices,
  100 SEO; LCP 1.2 s, TBT 60 ms, CLS 0; 69 KiB initial transfer.

## Reproduce

```sh
npm ci
npm test
npm run typecheck
npm run build
cargo test --all-targets --all-features --locked
cargo test --doc --locked
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo package --locked
```

To reproduce the highest-impact volume defect:

```sh
python3 - <<'PY'
open('/tmp/spaced.ndjson', 'w').write('{"a":' + ' ' * 1024 + '1}\n')
PY
cargo run -- inspect /tmp/spaced.ndjson --sample-seconds 1 \
  --output /tmp/report.json --json
```

The input event is 1,031 bytes; the report returns
`average_event_bytes: 7`.

## Scope and next step

No product code was modified. Only this verification report and handoff were
written. Repair the core byte accounting and path/detector handling, add
regressions for the demonstrated inputs, fix the serious accessible-name
finding, then rerun independent verification.
