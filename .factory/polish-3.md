# Polish round 3 — complete finding closure

Repair and deployed application commit:
`2d77f83671c2deebe18fc51ed20b48dc77f93118`.

This round read every `review-*.md` and `polish-*.md`. The two outstanding
review-3 contract gaps are repaired here; every earlier fix was rechecked from
the deployed build rather than assumed from its earlier report.

| Finding | Change made | Evidence: test, screenshot, live check |
| --- | --- | --- |
| F-1-1 | Replaced the unpublished registry install with the linked public GitHub source checkout and Cargo commands. | `@claim:source-checkout` passed from the fresh clone; [home screenshot](polish-artifacts-3/home-1440.png); the live home link and GitHub source link returned 200 in [live-link-crawl.json](polish-artifacts-3/live-link-crawl.json). |
| F-1-2 | Kept the report's forwarding recommendation separate and shows the real `forwarding --url` command and its generated configuration. | `@claim:forwarding-config` passed; [demo screenshot](polish-artifacts-3/demo-query-390.png); live `/?demo=1` exposes the headed configuration section. |
| F-1-3 | Renders all three detector findings and labels the metric as “3 findings across 2 fields.” | `@claim:sample-demo` passed; [demo screenshot](polish-artifacts-3/demo-query-1440.png); live direct demo rendered the bundled report and passed the cold audit. |
| F-1-4 | Removed broad vendor/search/telemetry wording. The remaining loopback, disposal, request, and storage statements have narrow executable coverage. | `@claim:local-only` and `@claim:discard-default` passed; [privacy screenshot](polish-artifacts-3/privacy-390.png); live demo began with empty storage, reset only `demo:drain-check`, wrote no cookie, and made four same-origin requests in [live-audit.json](polish-artifacts-3/live-audit.json). |
| F-1-5 | Uses direct section names: “Local 10-minute sample” and “How it works.” | Copy audit passed with no banned wording; [home screenshot](polish-artifacts-3/home-390.png); the live cold audit confirmed the removed labels are absent. |
| F-1-6 | Uses `report`, `sensitive data`, and `forwarding configuration` consistently; the undefined contract/intent language is gone. | `npm test` passed; [home screenshot](polish-artifacts-3/home-1440.png); live copy audit checked the old phrases are absent and required wording is present. |
| F-1-7 | Retains page-specific Terms and 404 headings, route titles, canonical metadata, focused navigation, and real HTTP 404 delivery. | Route/Axe browser suite passed; [terms screenshot](polish-artifacts-3/terms-390.png) and [404 screenshot](polish-artifacts-3/missing-390.png); live `/terms` returned 200 and `/missing` returned 404 with one H1 and no Axe findings. |
| F-2-1 | Kept one named claim test each for output collisions, minimum duration, JSON stdout, and complete help; removed the untestable detector-limit wording. | `@claim:separate-output-paths`, `@claim:minimum-sample-duration`, `@claim:json-stdout`, and `@claim:complete-help` passed from the fresh clone; [home screenshot](polish-artifacts-3/home-1440.png); the deployed source README was checked through the live GitHub source route. |
| F-2-2 | The demo-exit action is “View local setup,” which names its destination. | `@claim:local-only` passed; [demo screenshot](polish-artifacts-3/demo-query-390.png); live click routed to `/` and focused the landing H1 in [live-audit.json](polish-artifacts-3/live-audit.json). |
| F-3-1 | Added the `site-build-output` claim and a single tagged test that runs `npm run build:site` and checks `dist/site/index.html` plus `dist/site/staticwebapp.config.json`. | `@claim:site-build-output` passed from the fresh clone; [home screenshot](polish-artifacts-3/home-1440.png); the live GitHub README contains only the now-tested `dist/site` statement in [live-readme.md](polish-artifacts-3/live-readme.md). |
| F-3-2 | Deleted the unnecessary `cargo package` / unpublished-release-status sentence, so no release-status promise remains. | Claims-contract test and README unlisted-claim audit passed; [home screenshot](polish-artifacts-3/home-1440.png); the deployed source README has no `cargo package prepares` or `repository does not publish` wording in [live-readme.md](polish-artifacts-3/live-readme.md). |

## Fresh-clone verification

- Fresh remote clone: `/tmp/drain-check-round3-Nhjudg/repo` at
  `2d77f83671c2deebe18fc51ed20b48dc77f93118`.
- `npm ci`, then all 18 literal commands in `.factory/claims.json`, each run
  separately: pass.
- Full fresh-clone suite: `npm test` (36/36), `npm run typecheck`,
  `npm run build:site`, `cargo test --all-targets --all-features --locked`
  (18/18), `cargo fmt --all -- --check`,
  `cargo clippy --all-targets --all-features -- -D warnings`,
  `cargo package --locked`, and `cargo doc --no-deps --locked`: pass.
- Production build: 11.33 kB JavaScript (4.26 kB gzip), 6.71 kB CSS
  (2.21 kB gzip), and 62.2 kB original WebP art.

## Deployment and cold live verification

- Deployed with `/opt/fleet/lib/deploy-static.sh log-drain-contract-check
  dist/site`; Azure deployment `bd388386-d800-49f6-965e-4eea76554566` served
  the footer build `2d77f83671c2`.
- `/`, `/?demo=1`, `/demo`, `/privacy`, and `/terms` returned 200. `/missing`
  returned a designed HTTP 404. Each route at 1440 px and 390 px had one H1,
  one main landmark, route-specific title/canonical/description, no overflow,
  and zero Axe violations. The normal routes had no console/page errors; the
  browser's expected failed-resource message is recorded only for the direct
  HTTP-404 visit.
- The live demo banner, reset isolation, same-origin request log, cookies,
  mobile first action, keyboard route focus, browser Back focus, and all live
  links/robots/sitemap are recorded in [live-audit.json](polish-artifacts-3/live-audit.json)
  and [live-link-crawl.json](polish-artifacts-3/live-link-crawl.json).
- `verify-url.sh` passed for the live [home](polish-artifacts-3/verify-home/verify.json)
  and [demo](polish-artifacts-3/verify-demo/verify.json). Live mobile
  Lighthouse: performance 100, accessibility 100, best practices 100, SEO
  100; LCP 1.23 s, CLS 0, TBT 38 ms in
  [lighthouse-summary.json](polish-artifacts-3/lighthouse-summary.json).

No finding of any severity remains open.
