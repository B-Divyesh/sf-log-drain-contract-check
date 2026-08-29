# Polish round 4 — complete cumulative finding closure

Application repair commit: `cd28d94fe407243fa0e194cac7cbdc269ae75972`.
Deployment: `3757eef6-c34e-49e2-abba-126774284824`.

This round read every `.factory/review-*.md` and `.factory/polish-*.md`. The
table maps every finding ID to its present implementation and to reproducible
clean-clone and cold-live evidence.

| Finding ID | Change made | Evidence: test, screenshot, live check |
| --- | --- | --- |
| F-1-1 | The unavailable registry install remains removed. The linked public source checkout and Cargo command remain the only install path. | `@claim:source-checkout` passed from the clean clone; [home mobile](polish-artifacts-4/home-390.png); live source README was 200 in [live audit](polish-artifacts-4/live-audit.json). |
| F-1-2 | The report keeps only a forwarding recommendation; the separate real `forwarding --url` configuration remains explicit. | `@claim:forwarding-config` passed; [demo desktop](polish-artifacts-4/demo-query-1440.png); live `/?demo=1` exposes the headed configuration section. |
| F-1-3 | The report still renders all three detector findings and now calls the related locations field paths consistently. | `@claim:sample-demo` passed; [demo mobile](polish-artifacts-4/demo-query-390.png); live audit confirms 3 findings in 2 field paths. |
| F-1-4 | Broad vendor/search/telemetry slogans remain absent. Narrow loopback, body-disposal, same-origin, and storage behavior are executable claims. | `@claim:local-only` and `@claim:discard-default` passed; [privacy mobile](polish-artifacts-4/privacy-390.png); live audit records only four same-origin requests and empty browser storage. |
| F-1-5 | Direct section names remain: Local 10-minute sample and How it works. | Copy audit passed; [home desktop](polish-artifacts-4/home-1440.png); live cold audit rendered the direct labels. |
| F-1-6 | The old contract/intent slogans remain gone. The landing uses report, sensitive data, and forwarding configuration consistently. | `npm test` 40/40; [home mobile](polish-artifacts-4/home-390.png); live audit passed the landing and demo routes. |
| F-1-7 | Terms and 404 retain contextual h1s, metadata, legal links, focused navigation, and a real HTTP 404. | Route/Axe tests passed; [terms mobile](polish-artifacts-4/terms-390.png) and [404 mobile](polish-artifacts-4/missing-390.png); live audit recorded `/terms` 200 and `/missing` 404. |
| F-2-1 | All remaining concrete CLI promises retain a dedicated named claim; the untestable detector-limit wording remains absent. | `@claim:separate-output-paths`, `@claim:minimum-sample-duration`, `@claim:json-stdout`, and `@claim:complete-help` each passed in the clean clone; [home desktop](polish-artifacts-4/home-1440.png). |
| F-2-2 / F-4-1 | Restored **View local setup** in the banner, demo documentation, and tagged browser test. It returns to `/` and focuses its h1. | `@claim:local-only` passed; [demo mobile](polish-artifacts-4/demo-query-390.png); live audit checks the label, route, and h1 focus. |
| F-3-1 | `site-build-output` now proves the emitted root configuration contains routes, 404 handling, CSP, and immutable asset cache policy. | `@claim:site-build-output` passed from the clean clone; [home desktop](polish-artifacts-4/home-1440.png); live audit confirms routes and response headers. |
| F-3-2 | The unsupported package/release-status sentence remains deleted. | Claims-contract test passed; [home mobile](polish-artifacts-4/home-390.png); live source README check passed in the live audit. |
| F-4-2 | README deployment guidance now names Azure Static Web Apps and directs other hosts to recreate rewrites, 404, headers, and cache rules. | `@claim:site-build-output` parses the real generated configuration; [home desktop](polish-artifacts-4/home-1440.png); live source README 200 and exact guidance asserted in [live audit](polish-artifacts-4/live-audit.json). |
| F-4-3 | `--platform` rejects all control characters, so a newline cannot create an uncommented assignment. | `@claim:forwarding-config` runs `forwarding_rejects_control_character_platform_labels`; [demo configuration](polish-artifacts-4/demo-query-1440.png); live demo retains the validated configuration workflow. |
| F-4-4 | `local-only` now asserts empty cookies, local/session storage, IndexedDB, Cache Storage, service-worker registrations, and OPFS after landing → demo → reset → exit. | `@claim:local-only` passed from the clean clone; [privacy desktop](polish-artifacts-4/privacy-1440.png); live audit records every storage surface empty. |
| F-4-5 | Replaced the recorded `17 fields` with `17 field paths`; user-facing process prose now says receiver, not listener. | `@claim:sample-demo` passed; [home mobile](polish-artifacts-4/home-390.png) and [demo mobile](polish-artifacts-4/demo-query-390.png); live audit asserts both field-path metrics. |

## Cumulative verification

- Clean remote clone `/tmp/drain-check-round4-clean-mWtGuM/repo` at the repair
  commit: all 18 claim commands separately pass; `npm test` 40/40; typecheck,
  build, audit, 26 Rust tests, doc-test, Rustfmt, Clippy, docs, and package pass.
- Deploy command: `/opt/fleet/lib/deploy-static.sh log-drain-contract-check
  dist/site`.
- Cold live result: all primary/legal/demo routes 200, designed missing route
  404, desktop/mobile no overflow, one h1/main per route, zero serious/critical
  Axe results, no normal-route console errors, privacy and focus checks pass.
- Lighthouse mobile result: 100/100/100/100, LCP 1,252 ms, TBT 34 ms, CLS 0.

No finding from reviews 1–4 remains open.
