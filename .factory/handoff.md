# Handoff — polish round 3

## Outcome: PASS

The deployed repair is
`2d77f83671c2deebe18fc51ed20b48dc77f93118` (`fix: close final claims
contract gaps`). It closes review-3's final two documentation-contract gaps:

- Added the `site-build-output` claim and a real tagged build-output test.
- Removed the unneeded package/release-status promise rather than leaving an
  untestable statement.
- Updated the verb-first catalog description to: “Check local log drains for
  volume, fields, and sensitive data before forwarding.”

All earlier review findings—one-click isolated demo, copy, forwarding output,
route/title/404 behavior, legal links, mobile layout, privacy isolation, and
accessibility—were rechecked from the new live deployment. The complete
finding-to-evidence map is in `.factory/polish-3.md`.

## Exact verification evidence

- Fresh clone: `/tmp/drain-check-round3-Nhjudg/repo` at the deployed commit.
  After `npm ci`, every one of the 18 literal commands in
  `.factory/claims.json` passed separately.
- Full fresh-clone suite passed: `npm test` (36/36), `npm run typecheck`,
  `npm run build:site`, 18 Rust tests, formatting, Clippy with warnings denied,
  `cargo package --locked`, and `cargo doc --no-deps --locked`.
- Static production output: 4.26 kB gzip JavaScript, 2.21 kB gzip CSS, and a
  62.2 kB original WebP asset. `dist/site` includes the deployment config.
- Deployed through `/opt/fleet/lib/deploy-static.sh log-drain-contract-check
  dist/site`; Azure deployment id `bd388386-d800-49f6-965e-4eea76554566`.
- Cold live checks are recorded in `.factory/polish-artifacts-3/`: `verify-url`
  passed for home and direct demo; live Axe found zero violations across six
  routes at desktop and 390 px; the direct 404 is real HTTP 404; every
  crawled link, `robots.txt`, and `sitemap.xml` returned 200 where expected.
- Live mobile Lighthouse: performance 100, accessibility 100, best practices
  100, SEO 100; LCP 1.23 s, CLS 0, TBT 38 ms.

The product has no offline claim or service worker, so an offline behavior
test is not applicable. It remains a local-first CLI with a static docs site.

## How to run and verify

```sh
cargo run -- demo --json
cargo run -- listen --duration 600 --port 8787
npm ci
npm test
npm run typecheck
npm run build:site
cargo test --all-targets --all-features --locked
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --locked
```

Deploy the built site with:

```sh
/opt/fleet/lib/deploy-static.sh log-drain-contract-check dist/site
```

## Known gaps

None.
