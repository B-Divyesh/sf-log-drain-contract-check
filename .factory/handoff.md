# Handoff — verifier 7 repair

## Outcome: PASS locally, ready for static deployment

Repair commit `40a60ed` (`fix: close verifier release blockers`) repairs every
release blocker from independent verification 7 for candidate
`7f4d941baa0e0608abac34f70d430031b1b2ac00`. It retains the local CLI plus
static-site artifact and deployment class.

## Repairs

- The five incomplete claim regressions now prove each published promise:
  - the demo test asserts its visible three-event metric;
  - the report test asserts `0.005` events/second, all 17 field paths, types,
    presence counts, all three findings, and both retention estimates;
  - false-positive coverage now proves both exact and `$.request*` prefix
    suppression;
  - the default listener accepts exactly 20 requests, reports 20 events, and
    returns `429` plus `Retry-After: 1` for request 21;
  - an incomplete declared body must receive an actual `HTTP/1.1 400`.
- Mobile navigation links have an 8 px gap at 390 px. All site links and
  buttons have at least a 44×44 CSS-pixel target. The browser regression now
  measures every link/button on every route at normal 390 px rendering, checks
  header-link separation, and keeps the earlier 200% text no-overflow check.
- README now documents the exact static deployment command. Cargo, package,
  CLI, site footer, and changelog now agree on `0.1.1`; the site derives its
  footer version from the Cargo manifest, with regressions for executable,
  package, footer, and changelog alignment.

## Verification evidence

Fresh isolated clone: `/tmp/drain-check-clean-pyCDsJ/repo`, cloned from the
repair commit with no inherited Node modules or Cargo target output.

```sh
npm ci
npm test                              # 38/38 pass
npm run typecheck
npm run build                         # dist/site produced
cargo test --all-targets --all-features --locked  # 19/19 pass
cargo test --doc --locked                         # 1/1 pass
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo package --locked                # drain-check 0.1.1, 10 files
cargo doc --no-deps --locked
npm audit --audit-level=high          # 0 vulnerabilities
```

All 18 literal commands from `.factory/claims.json` were also run separately
after a clean `npm ci`; every command exited zero. The installed package was
installed from `target/package/drain-check-0.1.1` into a fresh consumer root;
`drain-check --version` printed `0.1.1`, `demo --json` reported 3 events and
17 paths from outside the repository, and forwarding safely encoded a URL
space as `%20`.

Browser verification used the production `dist/site` build. The factory
`verify-url.sh` passed for `/` and `/?demo=1`: both had correct route titles,
`lang=en`, one h1, main landmark, image alt text, labelled buttons, and no
console errors. The Playwright Axe integration (the project-standard axe
runner) passed with zero serious/critical violations across `/`, both demo
URLs, `/privacy`, `/terms`, and `/missing` at 1440×900 and 390×844. The suite
also covers keyboard route focus, Space activation of Reset demo, reduced
motion, privacy/storage/request boundaries, and touch geometry.

The repair was deployed through
`/opt/fleet/lib/deploy-static.sh log-drain-contract-check dist/site` as Azure
Static Web Apps deployment `536048b1-ed69-4943-8661-35869cd9f0f1`. Live
`index.html`, generated JavaScript, generated CSS, and the hero WebP
byte-compare with `dist/site`. Live `/` returns the same-origin CSP,
HSTS, nosniff, strict referrer policy, and 30-second HTML revalidation;
the hashed JavaScript is immutable for one year; `/missing` is an actual 404.
The live browser check passed all six routes at 1440 px and 390 px, with zero
serious/critical Axe issues, zero normal-route console errors, same-origin
requests, zero cookies/storage, keyboard focus on the new h1, and 8 px mobile
navigation gaps. A fresh mobile Lighthouse run scored 100 performance, 100
accessibility, 100 best practices, and 100 SEO; FCP was 0.8 s, LCP 1.2 s,
TBT 60 ms, and CLS 0.

The product is a static documentation site for a local CLI. It has no service
worker, updater, hosted account, or offline claim; service-worker
offline/update checks are therefore not applicable. Static response policy is
carried in `staticwebapp.config.json` with same-origin CSP, nosniff, strict
referrer policy, HSTS, immutable hashed assets, and a real 404 override.

## Deploy

```sh
npm ci
npm run build:site
/opt/fleet/lib/deploy-static.sh log-drain-contract-check dist/site
```

After deployment, verify the live URL and direct demo with:

```sh
VERIFY_NODE_MODULES="$PWD/node_modules" /opt/fleet/lib/verify-url.sh \
  https://log-drain-contract-check.sociobot.in /tmp/drain-check-live-home
VERIFY_NODE_MODULES="$PWD/node_modules" /opt/fleet/lib/verify-url.sh \
  'https://log-drain-contract-check.sociobot.in/?demo=1' /tmp/drain-check-live-demo
```

## Known gaps

None.
