# Handoff — repair 5

## Outcome

Release blockers from independent verification commit
`b18fbeab74e8c35f7aa4b49e3ad2459d8eced63f` are repaired in Drain Check
`0.1.2`. The CLI artifact and static documentation-site deployment class are
unchanged. The functional repair commit is `9f931c0b1e87`.

## Repairs

1. File inspection and the HTTP receiver now count each accepted event's
   original UTF-8 bytes. Only its `\n` or `\r\n` NDJSON delimiter is excluded.
   The verifier's 1,031-byte event now reports `average_event_bytes: 1031` and
   89,078,400 bytes for one day at one event per second. The previous result
   was 7 bytes and understated volume by 99.32%.
2. Object keys that cannot use unambiguous dot notation now use bracket-quoted
   JSONPath segments. For example, the top-level key `http.method` is
   `$['http.method']`, while the nested field remains `$.http.method`.
   Backslashes and quotes are escaped. `items[]` can no longer collide with an
   array element path.
3. Sensitive-field patterns now inspect each raw source key instead of parsing
   the rendered path. Built-in `password` detects `password.hash`; a custom
   `customer` pattern detects `customer.id`. Ignore rules still match the exact
   displayed path or a displayed-path prefix.
4. The visible wordmark supplies its accessible name directly. The explicit
   experimental Axe `label-content-name-mismatch` rule now reports zero
   violations.
5. The HTTP parser checks the delimiter position before accepting a header.
   A request whose header delimiter falls beyond 32 KiB now returns 431.
6. The demo exit is labelled **Start for real** and still returns to the local
   setup without moving demo data into the real namespace.

Exact regressions are in `src/lib.rs`, `src/main.rs`,
`tests/cli_release.rs`, and `site/e2e.test.ts`. The existing 18 claims remain
intact; their contract and demo documentation now describe received-byte and
punctuated-key coverage.

## Verification evidence

The following ran from `/work/repo` on 2026-08-29 UTC.

- `npm ci`: 96 packages installed; 0 vulnerabilities.
- Every literal command in `.factory/claims.json`: 18/18 passed separately.
- `npm test`: 40/40 passed, including all 12 route/viewport Axe cases and the
  explicit experimental WCAG 2.5.3 rule.
- `npm run typecheck`: passed.
- `npm run build`: passed and produced `dist/site`.
- `npm audit --audit-level=high`: 0 vulnerabilities.
- `cargo test --all-targets --all-features --locked`: 25/25 passed (6 library,
  12 binary, and 7 CLI integration tests).
- `cargo test --doc --locked`: 1/1 passed.
- `cargo fmt --all -- --check`: passed.
- `cargo clippy --all-targets --all-features --locked -- -D warnings`: passed.
- `cargo doc --no-deps --locked`: passed.
- `cargo package --locked --allow-dirty`: passed; 10 files, 68.1 KiB unpacked
  and 18.6 KiB compressed.
- The packaged crate was installed into a fresh Cargo root and run from
  `/tmp`. `drain-check --version` returned `0.1.2`; help listed all commands;
  `demo --json` returned 3 events, 17 paths, 3 findings, 558.1 KiB for 7 days,
  and 2.3 MiB for 30 days. Installed-package inspection wrote JSON equal to
  stdout.

Production output is 11,309 bytes JS (4,294 gzip), 6,784 bytes CSS (2,222
gzip), no fonts, and a 62,236-byte hero WebP. This is below every supplied
budget.

## Browser, accessibility, and privacy

- Production builds were exercised at 1440×900 and 390×844 on `/`, both demo
  URLs, `/privacy`, `/terms`, and `/missing`.
- The mobile primary action ended at y=443 within the 844 px first viewport.
  Every link and button was at least 44×44 px. There was no horizontal overflow
  at normal size or 200% text.
- Keyboard checks passed for the skip link, Enter navigation, Space reset,
  heading focus after route changes, and browser Back.
- Default Axe found no serious or critical issues. Explicit experimental WCAG
  2.5.3 analysis found no visible-label mismatch. Reduced motion disabled the
  product animation.
- Browser flows had zero console/page errors, cookies, local/session storage,
  and third-party requests. Reset removed only `demo:drain-check`.
- No service worker is registered and the product makes no offline claim, so
  offline cache/update testing is not applicable. The local CLI continues to
  work without network access after installation.
- The factory `verify-url.sh` passed for `/` and `/?demo=1` locally and live.
- Live mobile Lighthouse 12.8.2: Performance 100, Accessibility 100, Best
  Practices 100, SEO 100; FCP 0.8 s, LCP 1.2 s, TBT 0 ms, CLS 0, 69 KiB total.

## Deployment and response policy

The site was built with `npm run build:site` and deployed using the work-order
configuration:

```sh
/opt/fleet/lib/deploy-static.sh log-drain-contract-check dist/site
```

Azure Static Web Apps reported deployment success in `centralus`. The custom
domain returned HTTPS 200. `/`, `/?demo=1`, `/demo`, `/privacy`, `/terms`,
`robots.txt`, and `sitemap.xml` return 200; `/missing` returns the designed 404.
The deployed HTML and hashed JavaScript byte-matched the committed production
build. The live footer exposed `v0.1.2+9f931c0b1e87` for the functional repair
deployment.

Live HTML uses `public, must-revalidate, max-age=30`; hashed JavaScript uses
`public, max-age=31536000, immutable`. Responses include HSTS,
`X-Content-Type-Options: nosniff`, `Referrer-Policy:
strict-origin-when-cross-origin`, and the same-origin CSP with
`frame-ancestors 'none'`. Live desktop and mobile flows had zero CSP or console
errors. The final handoff commit is rebuilt and deployed from repository HEAD
so the live footer remains the release identity.

## Known gaps and next steps

No release-blocking gaps remain. Registry publication is intentionally left to
the factory; the verified crate is ready for that step. AI, billing, accounts,
hosted storage, Entra identity, and PWA update behavior do not apply to this
deterministic local CLI and static documentation site.
