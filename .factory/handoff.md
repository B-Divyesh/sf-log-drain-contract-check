# Review 1 handoff — Drain Check

## What was done

Completed the requested adversarial first-read review of the deployed site without changing product code. Wrote the evidence-backed review to `.factory/review-1.md`.

## Verdict

**FAIL.** The blocking issue is that the live landing page tells users to run `cargo install drain-check`, but the crate is not available on crates.io. The review also records report/demo consistency, claims coverage, and copy findings.

## How verified

- Fresh live Playwright checks at 390 px and desktop, including demo isolation/reset, request log, routes, metadata, links, history/focus, and 404.
- Fresh temporary clone: `npm ci`, then every `.factory/claims.json` command passed individually (11/11).
- Local: `npm test` (25 passed), `npm run typecheck`, and `npm run build` passed.
- CLI demo run from a temporary directory passed.
- Fresh temporary `cargo install --root <temp> drain-check` failed with “could not find `drain-check` in registry `crates-io`,” reproducing F-1-1.

## Known gaps / next steps

Resolve every finding in `.factory/review-1.md`, starting with publishing the crate or replacing the unavailable install command with tested clone instructions. Re-run the entire checklist after repair; do not treat the existing claim-test pass as closure because several live promises are not listed claims.
