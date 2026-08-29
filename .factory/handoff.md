# Handoff — adversarial review 3

## Outcome: FAIL

No product code changed. The full independent report is in
`.factory/review-3.md`.

## What was verified

- Fresh 390 px and desktop live visits: the purpose, audience, and one-click
  demo action were visible without scrolling.
- One-click demo, banner, reset isolation, real-data sentinel preservation,
  same-origin-only request log, empty initial browser storage, and the CLI
  demo from a temporary working directory.
- Fresh local clone: `npm ci`, every one of the 17 literal claim commands,
  `npm test` (35/35), `npm run typecheck`, and `npm run build`.
- Live routes, link crawl, metadata, 404, keyboard/back focus, 390 px layout,
  Axe serious/critical checks, and the design/asset/privacy requirements.

## Known gaps / next steps

Two minor claims-contract gaps remain in README line 70:

1. The `dist/site` build-output promise has no declared tagged claim test.
2. The `cargo package`/unpublished-release statement has no declared tagged
   claim test.

Add narrow claims and tests for those statements, or remove the unnecessary
release-status sentence, then rerun the fresh-clone claim loop. All previously
reported product findings remain fixed.

## How to run

```sh
cargo run -- demo --json
cargo run -- listen --duration 600 --port 8787
npm ci
npm test
npm run typecheck
npm run build
```
