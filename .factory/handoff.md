# Drain Check verification handoff

## Release status

**FAIL — do not release the requested candidate.**

- Requested candidate: `bd87644587b19278a7152595e9938866460e7aa9`
- Available/tested revision: `bd876445876521f0eb6568658e6edee683f67c8e`
- Live URL: <https://log-drain-contract-check.sociobot.in>
- Full report: [verification-3.md](verification-3.md)

The requested object is absent locally and `git fetch` reports `not our ref`. Remote `main` and the live footer instead identify the available revision. The live HTML and core assets match the production build from that available revision byte-for-byte.

## Blocking defects

1. The requested candidate cannot be fetched, checked out, or matched to production.
2. When `--output` and `--save-sample` name the same file, the final report overwrites the accepted sample while reporting `bodies_saved: true`.
3. `inspect --sample-seconds 0` accepts a non-empty sample and emits zero event rate and `0 B` retention estimates.
4. Forwarding URL validation accepts malformed values such as `http://:`, `https://?query`, and quote-containing URLs, then emits broken configuration.

## What passed

- After `npm ci`, every exact test in `.factory/claims.json` passed; the cold Rust-backed `local-only` claim completed in 29.76 seconds.
- `npm test` passed 25/25; TypeScript, Rust formatting, 13 Rust tests, one doctest, strict Clippy, audit, release build, site build, and crate packaging all passed.
- A clean consumer install ran the embedded demo outside the repository with the documented exact metrics.
- Independent receiver tests confirmed loopback-only binding; recovery from 400/405/413 inputs; exact 2 MiB acceptance; default body discard; explicit save with distinct paths; and SIGINT reporting.
- The observed receiver allowance is 20 accepted requests per rolling second. Request 21 returned 429 with `Retry-After: 1`.
- Live desktop and 390 px checks found zero serious/critical Axe issues, no normal-route console/page errors, 44 px targets, visible keyboard focus, correct route focus, no 200% text overflow, and reduced-motion compliance.
- The complete demo flow made only same-origin requests and left cookies, Web Storage, IndexedDB, Cache Storage, and service workers empty.
- Lighthouse mobile scored 100 in performance, accessibility, best practices, and SEO; LCP was 1.3 s, TBT 0 ms, CLS 0, and total transfer 68 KiB.

## Reproduce and next steps

Run the clean suite with:

```sh
npm ci
jq -r '.[].test' .factory/claims.json  # run each printed command
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

Repair path collisions before binding, require `sample-seconds >= 1`, and use a real HTTP(S) URL parser with safe output encoding. Push a real candidate SHA, deploy that exact build, then repeat all claim, package-consumer, receiver-boundary, and live identity checks. No product code was changed during this verification.
