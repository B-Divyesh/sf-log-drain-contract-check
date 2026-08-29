# Handoff — polish round 4

## Outcome

**PASS.** This repair closes every finding in review rounds 1–4. The repaired
application commit is `cd28d94fe407243fa0e194cac7cbdc269ae75972` (`0.1.3`),
deployed as Azure Static Web Apps deployment
`3757eef6-c34e-49e2-abba-126774284824`.

The live footer and cold audit report build `cd28d94fe407`. The live product is
at <https://log-drain-contract-check.sociobot.in>.

## What changed

- Rewrote the first screen as “Check a log drain before forwarding”; it remains
  visible at 390 px and explains the sample action in plain words.
- Restored the honest demo exit label: **View local setup**. `/?demo=1` remains
  a one-click isolated bundled report with its persistent banner and Reset demo.
- Hardened `forwarding --platform`: control characters, including newlines, are
  rejected before rendering configuration output.
- Extended the `local-only` claim test to cover cookies, Web Storage, IndexedDB,
  Cache Storage, service-worker registrations, and OPFS through the full demo
  flow.
- Scoped static-host instructions to Azure Static Web Apps and told other hosts
  to recreate the required host configuration.
- Standardized user-facing wording on **receiver** and **field paths**.
- Added `scripts/live-audit.mjs` for repeatable cold deployed-site checks.

## Exact verification evidence

Fresh remote clone: `/tmp/drain-check-round4-clean-mWtGuM/repo` at
`cd28d94fe407243fa0e194cac7cbdc269ae75972`.

- `npm ci` succeeded with 0 audit vulnerabilities.
- Every one of the 18 literal commands in `.factory/claims.json` ran separately
  from that clean clone and passed: `sample-demo`, `local-only`,
  `discard-default`, `contract-report`, `forwarding-config`, `source-checkout`,
  `false-positive-controls`, `rate-limit`, `request-recovery`, `explicit-save`,
  `separate-output-paths`, `minimum-sample-duration`, `json-stdout`,
  `complete-help`, `interrupt-report`, `portable-demo`, `site-build-output`, and
  `mit-license`.
- Full clean-clone suite passed: `npm test` **40/40**; `npm run typecheck`;
  `npm run build`; `npm audit --audit-level=high`; `cargo test --all-targets
  --all-features --locked` **26/26**; `cargo test --doc --locked` **1/1**;
  Rustfmt; Clippy with `-D warnings`; `cargo doc --no-deps --locked`; and
  `cargo package --locked --allow-dirty`.
- Production output: 11.33 kB JavaScript (4.25 kB gzip), 6.78 kB CSS (2.21 kB
  gzip), and 62.2 kB original WebP art.
- Live `verify-url.sh` passed for [home](polish-artifacts-4/verify-home/verify.json)
  and [direct demo](polish-artifacts-4/verify-demo/verify.json): HTTPS 200,
  route titles, `lang=en`, one h1, main, image alt text, labeled buttons, and no
  page errors.
- `node scripts/live-audit.mjs https://log-drain-contract-check.sociobot.in
  .factory/polish-artifacts-4 cd28d94fe407` passed after deploy. It checks all six routes at
  1440 and 390 px, real HTTP 404, metadata/canonicals, zero serious/critical
  Axe findings, zero normal-route console errors, no overflow, the query demo,
  reset isolation, all browser storage surfaces, same-origin requests, focus,
  response headers, source README deployment wording, and the exact footer
  build. See [live audit](polish-artifacts-4/live-audit.json).
- Mobile Lighthouse: Performance **100**, Accessibility **100**, Best Practices
  **100**, SEO **100**; LCP 1,252 ms, TBT 34 ms, CLS 0, transfer 70,209 bytes.
  See [report](polish-artifacts-4/lighthouse-mobile.json).

## Deployment and operation

```sh
npm ci
npm run build:site
/opt/fleet/lib/deploy-static.sh log-drain-contract-check dist/site
```

For development, use `cargo run -- demo --json` for the CLI sample or `npm run
dev` for the site. The ready-to-publish crate command is `cargo package
--locked`; do not publish from this worker.

## Known gaps

None. No offline or AI capability is claimed; this deterministic local CLI does
not need either one.
