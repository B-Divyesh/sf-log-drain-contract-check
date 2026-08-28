# Drain Check handoff

## Delivered

- Rust `drain-check` CLI with `listen`, `inspect`, `demo`, and `forwarding` commands.
- A localhost-only receiver (`127.0.0.1`) that accepts newline-delimited JSON over HTTP, aggregates a bounded sample, and discards bodies by default.
- JSON contract reports with event rate, average event size, field paths/types/presence, conservative secret and email detectors, 7/30-day retention estimates, and a forwarding note.
- `--save-sample` is explicit and records that decision in the report.
- Bundled realistic sample at `examples/drain.ndjson`; `cargo run -- demo --json` writes the same report to a temporary path.
- Vite static docs/demo site in `dist/site`, with `/`, `/demo`, `/privacy`, `/terms`, and a styled runtime 404 route.
- One-click browser demo, local CLI terminal recording, metadata, sitemap, robots, CSP/security headers, favicon/apple icon, and original pixel/demoscene hero art.

## Verification

Run from the repository root:

```sh
cargo test
npm test
npm run build:site
cargo package --allow-dirty --no-verify
```

All passed in this work order. The browser test suite covers all entries in `.factory/claims.json`, demo navigation, third-party request absence, and Axe on a 390 px mobile demo. The live receiver was also exercised with `curl`; it returned HTTP 202 for the bundled NDJSON and produced a report with 3 events.

Build output: `dist/site/index.html`.

Lighthouse-class checks: production JS is 7.04 KB raw / 3.02 KB gzip; CSS is 5.63 KB raw / 1.94 KB gzip; LCP hero is 62.2 KB WebP; Axe has zero violations on the mobile demo. A CLI Lighthouse run could not complete in this container because Chrome showed a local HTTP interstitial; the static budget and browser accessibility checks passed.

## Privacy

No telemetry, analytics, third-party runtime scripts, or remote fonts. The web demo is isolated and uses no persistent real-data storage. The CLI does not open a public listener by default and does not persist received bodies unless `--save-sample` is supplied.

## Known gaps / next steps

- The receiver accepts newline-delimited JSON. Other managed-drain envelopes may need a small adapter before their JSON records can be sampled.
- Conservative detectors are intentionally not a compliance scanner. Teams should add review rules for their own identifiers before forwarding production data.
- Run Lighthouse in a normal Chrome environment before a public performance certification; the production assets are already well below the stated budgets.
