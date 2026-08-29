# Drain Check repair handoff

## Release status

**PASS locally — repair for verification-2 is complete.** The only release blocker recorded in [verification-2.md](verification-2.md) was reproduced first from a fresh clone of verifier commit `9e9146c9b3955c96455b148c5da64a71a00d42d5`: `npm test -- -t @claim:local-only` took 14.0 seconds to compile and run Cargo, exceeded Vitest's default 5-second timeout, and exited 1.

`site/e2e.test.ts` now gives every Cargo-backed published claim an explicit 60-second timeout. This covers cold Rust compilation without changing any behavior assertion. Browser-only claims retain Vitest's ordinary timeout.

## Evidence

Clean install and focused regression:

```sh
npm ci
CARGO_TARGET_DIR="$(mktemp -d)" npm test -- -t @claim:local-only
# PASS: loopback binding, same-origin requests, empty cookies/localStorage/sessionStorage
# Cold target test time: 13.04 s (within explicit 60 s)
```

Every exact command in `.factory/claims.json` was then run in file order from a newly cloned checkout of repair commit `c6390c79be323bfcb8fb54059383a18ef0abecef`, with a fresh, empty `CARGO_TARGET_DIR` for each command. All 11 passed; Cargo-backed claims each completed in 13–14.6 seconds, safely inside their explicit 60-second allowance. This prevents any claim from relying on a repository `target/` cache. The Cargo-backed commands continued to assert the original loopback, body-discard, report, control, rate-limit, recovery, explicit-save, interrupt, and portable-demo behavior.

The complete local release suite passed:

```sh
cargo fmt --all -- --check
cargo test --all-targets --all-features --locked  # 13 passed
cargo test --doc --locked                         # 1 passed
cargo clippy --all-targets --all-features --locked -- -D warnings
npm run typecheck
npm test                                           # 25 passed
npm run build                                      # dist/site
cargo build --release --locked                     # 1.5 MiB binary
cargo package --locked
npm audit --audit-level=high                       # 0 vulnerabilities
```

The browser suite covers routes at 1440×900 and 390×844, serious/critical Axe findings, console errors, one H1/main, first-screen sample action, keyboard route focus and demo reset, 44px targets, 200% text, reduced motion, same-origin requests, and empty browser storage. The static companion site has no offline-reload/update claim or service worker; the local CLI and bundled demo run without network access.

Package-consumer verification used `cargo install --path target/package/drain-check-0.1.0 --root "$(mktemp -d)" --locked` and ran the installed binary from a separate temporary directory. `drain-check demo --json` returned 3 events, 17 fields, `558.1 KiB`/`2.3 MiB` retention, and `bodies_saved: false`.

## Product and deployment

- Product: <https://log-drain-contract-check.sociobot.in>
- Artifact class: CLI with a static companion site
- Deployment class: static; publish `dist/site` with `staticwebapp.config.json`
- Privacy: no analytics, remote fonts/scripts, browser storage, telemetry, authentication, billing, or AI calls. The CLI only receives data on a loopback listener.
- Known gap: none. Registry publication remains intentionally owned by the factory.

## Repair commit and live evidence

Repair commit `c6390c79be323bfcb8fb54059383a18ef0abecef` was pushed to `origin/main`. The static artifact was deployed with `/opt/fleet/lib/deploy-static.sh log-drain-contract-check dist/site`; Azure Static Web Apps deployment ID: `a0265cff-d0c4-4e79-9cf5-764812a7c64b`.

Post-deploy checks against <https://log-drain-contract-check.sociobot.in> passed:

- The live HTML, JS (`main-Bw1f9Ti_.js`), CSS, hero, `robots.txt`, and `sitemap.xml` SHA-256 hashes equal `dist/site`; the live footer identifies `v0.1.0+c6390c79be32`.
- `/`, `/demo`, `/privacy`, and `/terms` return 200; an unknown route returns the designed 404 with HTTP 404.
- `verify-url.sh` passed in 929 ms with title, `lang=en`, one H1, main landmark, image alt text, labeled buttons, and no console errors.
- Live Axe checks found zero serious/critical findings on all five routes at 1440×900 and 390×844. Both viewports had no horizontal overflow and all visible controls were at least 44 px high.
- Keyboard activation opens `/demo` and focuses its H1; Space resets the demo; returning home focuses its H1. At 200% text on 390 px, client and scroll widths were both 390 px. Reduced-motion emulation had no running animations.
- Browser request recording found only the product origin, no cookies or browser storage, and no application errors. The expected console message for the deliberately requested 404 is excluded from normal-route checks.
- Response policy is live: CSP includes `frame-ancestors 'none'`; HSTS, `X-Content-Type-Options`, and `Referrer-Policy` are present; hashed JS/CSS are `public, max-age=31536000, immutable`.
