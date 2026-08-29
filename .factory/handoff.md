# Handoff — adversarial first-read review 4

## Outcome

**FAIL.** Review 4 is recorded in `.factory/review-4.md`. No product code was
changed.

The live first screen is clear and the one-click demo is populated and
isolated. All 18 declared claim commands pass from a fresh clone. The verdict
fails because the previously repaired F-2-2 demo-exit label regressed, and four
additional documentation, input-hardening, claim-coverage, and terminology
findings remain.

## Work performed

- Opened the live product cold in separate 390 × 844 and 1440 × 900 contexts.
- Audited every landing-page and README sentence with word counts.
- Exercised landing → demo → reset → exit → Back with request and storage
  logging.
- Ran the installed-style CLI demo from a new temporary working directory.
- Executed all 18 `.factory/claims.json` commands independently from a fresh
  clone.
- Ran the full clean-clone test, typecheck, build, Rust test, format, and
  Clippy gates.
- Rechecked reviews 1–3, polish reports 1–3, and every earlier finding against
  the live site and source.
- Crawled links and checked metadata, response headers, routing, 404 behavior,
  focus, both viewport sizes, and Axe on every route.
- Tested the README’s generic static-host instruction with a plain static
  server and reproduced the known `--platform` newline injection.

## Verification summary

- Declared claims: 18/18 literal commands pass.
- Full suite: `npm test` 40/40; typecheck and production build pass.
- Rust: 25/25 tests; Rustfmt and Clippy pass.
- Live routes: expected 200s; designed `/missing` returns 404.
- Axe: zero violations across six routes at mobile and desktop widths.
- Privacy: same-origin requests only; no cookies; demo reset preserves a
  seeded real-data key and removes only `demo:drain-check`.
- Build size: 11.31 kB JS (4.26 kB gzip), 6.78 kB CSS (2.21 kB gzip).

## Findings left for the repairer

1. **F-4-1 / F-2-2 — BLOCKING:** restore a truthful demo-exit label such as
   “View local setup.”
2. **F-4-2 — MAJOR:** remove or correctly scope the generic static-host
   deployment promise.
3. **F-4-3 — MINOR:** reject or safely comment newline/control content in
   `--platform`.
4. **F-4-4 — MINOR:** extend the no-storage claim test to IndexedDB, Cache
   Storage, service workers, and OPFS.
5. **F-4-5 — MINOR:** use “field paths” and “receiver” consistently.

See `.factory/review-4.md` for exact quotes, evidence, and concrete rewrites.
