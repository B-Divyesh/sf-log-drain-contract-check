# Polish round 1 — finding closure

Reviewed sources: `.factory/review-1.md` and the complete set of earlier `.factory/review-*.md` / `.factory/polish-*.md`. No earlier polish report exists. Repair implementation commit: `efbc97dfcf9446f042cbb0ad933e9e52b0d8f1f9`.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Removed the unavailable `cargo install drain-check` instruction. The first page and README now give the public GitHub clone, `cd`, and `cargo run` path. Added the `source-checkout` claim. | `@claim:source-checkout` cloned the public repository into a fresh temporary directory and ran `cargo run --locked -- --help`; passed from clean clone `efbc97d`. Live copy is visible at <https://log-drain-contract-check.sociobot.in/#install-title> and in [landing-mobile.png](polish-artifacts/landing-mobile.png). |
| F-1-2 | Changed the report description to a forwarding recommendation. Added a separate “Generate a forwarding configuration” section with the real CLI command and exact output. Added URL validation/rendering claim coverage. | `@claim:forwarding-config` passed, including `forwarding_requires_a_real_http_url_and_encodes_it_safely`. Live demo output is visible at <https://log-drain-contract-check.sociobot.in/?demo=1> and in [demo-mobile.png](polish-artifacts/demo-mobile.png). |
| F-1-3 | Grouped the demo by field without dropping detector results. The metric now says “3 findings across 2 fields,” and CLI output uses the same wording. | `@claim:sample-demo` asserts the metric plus `secret-shaped value`, `sensitive field name`, and `email-shaped value`; passed. Cold live inspection found exactly three rendered detector rows. See [demo-desktop.png](polish-artifacts/demo-desktop.png). |
| F-1-4 | Removed broad “does not search logs,” “not a log destination,” and “no telemetry” wording. Replaced it with narrow statements about loopback binding, body disposal, same-origin website requests, and browser storage. Added claims for source checkout and forwarding output; all site and README claims now map to 13 executable entries. | Every command in `.factory/claims.json` passed separately from clean clone `efbc97d`. `@claim:local-only` and `@claim:discard-default` cover the replacement privacy statements. Live request logging found zero third-party requests and zero initial storage entries. |
| F-1-5 | Replaced “LOCAL PRE-FLIGHT / 10-MINUTE WINDOW” with “LOCAL 10-MINUTE SAMPLE” and “THE SHORT PATH” with “HOW IT WORKS.” | `.factory/copy-audit.md` has no banned wording or sentence over 22 words. Cold live text search found none of the removed labels. See [landing-desktop.png](polish-artifacts/landing-desktop.png). |
| F-1-6 | Replaced “Read the contract” with “Review the report,” “likely secrets” with “likely sensitive data,” and “Forward with intent” with “Generate a forwarding configuration.” | Copy audit terminology table uses one term per concept. Cold live text search found none of the old phrases. See [landing-mobile.png](polish-artifacts/landing-mobile.png). |
| F-1-7 | Changed the Terms H1 to “Terms for using Drain Check” and the unknown-route H1 to “Page not found.” Route-specific title, canonical metadata, focus restoration, legal links, and the real 404 response are covered together. | Browser test `sets route titles, metadata, legal links, and the designed 404 contract` passed. Live `/terms` returned 200 and `/missing` returned 404 with their expected titles/H1s. See [terms-mobile.png](polish-artifacts/terms-mobile.png) and [404-mobile.png](polish-artifacts/404-mobile.png). |

## Cross-cutting acceptance evidence

- Clean remote clone: `/tmp/tmp.udY0A6Wji9/repo`, commit `efbc97dfcf9446f042cbb0ad933e9e52b0d8f1f9`.
- All 13 claim commands passed separately from that clean clone.
- Full clean-clone suite passed: 31 Vitest/Playwright tests, 15 Rust tests, typecheck, build, formatting, Clippy, and `cargo package --locked`.
- Axe on `/`, `/?demo=1`, `/demo`, `/privacy`, `/terms`, and `/missing` at 390×844 and 1440×900: zero violations.
- Live Lighthouse mobile: performance 100, accessibility 100, best practices 100, SEO 100; LCP 1.2 s, CLS 0, TBT 0 ms.
- Production build: 4.26 kB gzip JavaScript and 2.21 kB gzip CSS.
- Factory URL verifier passed with title, `lang=en`, one H1, main landmark, image alt text, labeled buttons, and no load errors; raw summary: [live-verify.json](polish-artifacts/live-verify.json).
- The product has no offline claim or service worker. Live checks found no cache, IndexedDB database, or service-worker registration.

All seven findings are closed. No known defect or deferred minor item remains.
