# Polish round 2 — cumulative finding closure

Repair implementation and deployed source commit: `4d6b1d3e6b2ac238fb6f6985876979f6420563bf`.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Replaced the unpublished registry-install instruction with the public GitHub checkout and Cargo path. | Clean-clone `@claim:source-checkout` passed against GitHub. The live setup is on [home](https://log-drain-contract-check.sociobot.in/#install-title); see [home mobile](polish-artifacts-2/home/screenshot-mobile.png). |
| F-1-2 | Kept forwarding separate from the report and shows the real `forwarding --url` command and generated configuration. | Clean-clone `@claim:forwarding-config` passed. Live demo check records the exact command, POST method, and URL in [live-round2.json](polish-artifacts-2/live-round2.json). |
| F-1-3 | Keeps all three detector findings visible and labels the metric as three findings across two fields. | Clean-clone `@claim:sample-demo` passed. The cold live demo has three detector rows in [live-round2.json](polish-artifacts-2/live-round2.json) and [demo desktop](polish-artifacts-2/demo/screenshot-desktop.png). |
| F-1-4 | Removed broad untestable privacy/product wording and retained only narrow, executable claims about loopback, discarded bodies, browser storage, and same-origin requests. | Clean-clone `@claim:local-only` and `@claim:discard-default` passed. The live demo request log is same-origin only and reset preserves `real:drain-check` in [live-round2.json](polish-artifacts-2/live-round2.json). |
| F-1-5 | Replaced mood-only labels with “LOCAL 10-MINUTE SAMPLE” and “HOW IT WORKS.” | The updated copy audit has no banned language or long sentences. The cold live [home screenshot](polish-artifacts-2/home/screenshot-desktop.png) shows the direct labels. |
| F-1-6 | Uses “report,” “sensitive data,” and “forwarding configuration” consistently; the old contract/intent wording is absent. | `.factory/copy-audit.md` records the terminology table. The cold live [home screenshot](polish-artifacts-2/home/screenshot-desktop.png) and [demo check](polish-artifacts-2/live-round2.json) confirm the rendered wording. |
| F-1-7 | Retained plain Terms and 404 headings with route-specific titles, metadata, focus handling, and working legal links. | Clean-clone route/Axe suite passed. Live `/terms` is 200 and `/missing` is a designed HTTP 404; see [404 headers](polish-artifacts-2/404-headers.txt) and [live route check](polish-artifacts-2/live-round2.json). |
| F-2-1 | Removed the unsupported Rust-version sentence and the untestable detector-limit sentence. Added four named contracts and real CLI integration tests for path collision rejection, minimum inspection duration, JSON stdout, and complete help. | Every one of 17 claim commands passed separately from fresh clone `/tmp/drain-check-clean-Svo2RO/repo`; `@claim:separate-output-paths`, `@claim:minimum-sample-duration`, `@claim:json-stdout`, and `@claim:complete-help` each invoke their own observable test. |
| F-2-2 | Renamed the demo exit control from “Start for real” to “View local setup.” It still leaves the isolated demo and routes home. | Cold live demo check finds exactly one control with that name, lands on `/`, and moves focus to the home H1; see [live-round2.json](polish-artifacts-2/live-round2.json) and [demo mobile](polish-artifacts-2/demo/screenshot-mobile.png). |

## Cumulative acceptance evidence

- Fresh clone: `https://github.com/B-Divyesh/sf-log-drain-contract-check.git` at `4d6b1d3`; `npm ci`, all 17 literal commands from `.factory/claims.json`, `npm test` (35/35), typecheck, production build, 14 Rust tests, format, Clippy with warnings denied, and clean `cargo package --locked` all passed.
- Production build: 11.33 kB JavaScript (4.26 kB gzip), 6.71 kB CSS (2.21 kB gzip), and a 62.2 kB hero WebP.
- Deployed through `/opt/fleet/lib/deploy-static.sh log-drain-contract-check dist/site`; the live footer reports build `4d6b1d3e6b2a`.
- Cold live checks: `/`, `/?demo=1`, `/demo`, `/privacy`, and `/terms` return 200; `/missing` returns 404. At both 1440×900 and 390×844 every route has one H1 and main landmark, no horizontal overflow, and zero Axe serious/critical violations. Normal routes logged no console/page errors. Chrome reports the expected failed-network console entry only when directly opening the intentionally HTTP-404 route.
- The factory verifier passed for [home](polish-artifacts-2/home/verify.json) and [demo](polish-artifacts-2/demo/verify.json). Live CSP, HSTS, nosniff, and referrer-policy headers are recorded in [headers.txt](polish-artifacts-2/headers.txt).

No finding from review 1 or review 2 remains open.
