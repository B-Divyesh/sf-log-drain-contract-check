# Independent verification 8 — FAIL

**Candidate:** `92f01451faec66b4dfe5ddab823b0b2e4ebaa26f`  
**Live URL:** <https://log-drain-contract-check.sociobot.in>  
**Verified:** 2026-08-29 UTC  
**Environment:** Node 22.23.2, npm 10.9.8, Rust/Cargo 1.98.0,
Playwright 1.58.2, Chromium 145.0.7632.6

## Decision

**FAIL — do not release this candidate.**

The first-read/demo gate passes, all 18 declared claim commands pass after the
required clean install, every repository quality gate passes, the packaged CLI
runs outside the repository, and the live deployment byte-matches the candidate.
The release is blocked by fresh end-to-end cases derived from the researched
brief:

1. retention volume is computed from compact re-serialized JSON rather than
   the bytes actually received, producing a measured 99.32% underestimate for
   a valid event;
2. field names containing dots are not escaped, so distinct fields collapse
   into one reported path; and
3. the same path handling causes default and custom sensitive-field patterns
   to miss dotted field names completely.

These failures affect the product's core job: reviewing volume, fields, and
privacy risk before forwarding production logs. The volume result also misses
the brief's stated within-25% success measure.

## Mandatory first checks

### Claims gate

`.factory/claims.json` exists and contains 18 entries. The clone initially had
no `node_modules`, so a literal pre-install invocation could not start Vitest
(`vitest: not found`). After the required clean `npm ci`, I ran every literal
`test` entry separately before broader repository inspection. All 18 exited
zero:

| Claim | Result |
| --- | --- |
| `sample-demo` | PASS, 1 tagged test |
| `local-only` | PASS, 1 tagged test |
| `discard-default` | PASS, 1 tagged test |
| `contract-report` | PASS, 1 tagged test |
| `forwarding-config` | PASS, 1 tagged test |
| `source-checkout` | PASS, including a fresh public GitHub checkout |
| `false-positive-controls` | PASS, 1 tagged test |
| `rate-limit` | PASS, 1 tagged test |
| `request-recovery` | PASS, 1 tagged test |
| `explicit-save` | PASS, 1 tagged test |
| `separate-output-paths` | PASS, 1 tagged test |
| `minimum-sample-duration` | PASS, 1 tagged test |
| `json-stdout` | PASS, 1 tagged test |
| `complete-help` | PASS, 1 tagged test |
| `interrupt-report` | PASS, 1 tagged test |
| `portable-demo` | PASS, 1 tagged test |
| `site-build-output` | PASS, 1 tagged test |
| `mit-license` | PASS, 1 tagged test |

Per-command logs were captured at
`/tmp/drain-check-qa-claims-installed/<claim-id>.log`; the result table is
`/tmp/drain-check-qa-claims-installed/status.tsv`. The green fixture tests do
not cover the release-blocking valid inputs below.

### Cold first read and one-click demo

**PASS.** In a fresh 1440×900 browser context, the first screen says:

- what it does: “Inspect a log drain before forwarding”;
- for whom: platform teams checking volume, field types, and sensitive data;
- what to click: **Try it with sample data**, followed by “Opens the bundled
  report. Writes no browser data.”

At 390×844 the action is fully inside the first viewport (y=396.45–443.25).
One keyboard-activated click opens `/?demo=1`, moves focus to “Review this
drain sample,” and immediately shows 3 events, 0.005/sec, 17 paths, three
findings, and 7/30-day retention. The persistent banner, Reset demo, and exit
to local setup are present. Storage remains empty and all requests remain
same-origin.

## Defects by severity

### Critical

None.

### High — release blocking

1. **Retention and average-size results measure normalized JSON, not the
   received body.** A valid one-event NDJSON file containing 1,024 legal spaces
   between the colon and value was 1,031 event bytes. The installed CLI
   reported `average_event_bytes: 7`, because it parses the event and measures
   `serde_json::to_vec(event)`. At one event/second, the actual one-day volume
   is 89,078,400 bytes while the report projects 604,800 bytes: a **99.32%
   underestimate**. The same normalization is used by the HTTP receiver.
   This violates the core volume/retention job and the brief's within-25%
   success measure. Reproduction:

   ```sh
   python3 - <<'PY'
   open('/tmp/spaced.ndjson', 'w').write('{"a":' + ' ' * 1024 + '1}\n')
   PY
   drain-check inspect /tmp/spaced.ndjson --sample-seconds 1 \
     --output /tmp/report.json --json
   ```

2. **Distinct field paths collapse when source keys contain path punctuation.**
   For the valid event below, the report emits only one `$.http.method` entry
   with types `integer,string`; it cannot distinguish the top-level
   `"http.method"` field from nested `http.method`. Likewise, the top-level
   key `"items[]"` collides with an array element path and disappears as a
   distinct contract field.

   ```json
   {"http.method":"GET","http":{"method":42},"items[]":"flat","items":["array"]}
   ```

   Dotted field names are common in platform logs. A preflight that merges
   separate fields and their types does not provide a reviewable data contract.

3. **Dotted sensitive-field names bypass both built-in and configured
   patterns.** Inspecting
   `{"password.hash":"ordinary","customer.id":"12345"}` with
   `--sensitive-field customer` returns an empty `findings` array. The detector
   derives the key using `path.rsplit('.')`, so it checks only `hash` and `id`,
   not the actual source keys `password.hash` and `customer.id`. This is a 0%
   hit rate for that configured-pattern fixture and directly violates the
   brief's goal of identifying 95% of configured sensitive-field patterns.

### Medium — release blocking under the supplied accessibility baseline

1. **The visible wordmark is not contained in its accessible name.** Lighthouse
   and Axe with the experimental WCAG 2.5.3 rule enabled report one **serious**
   `label-content-name-mismatch` violation on:

   ```html
   <a class="wordmark" aria-label="Drain Check home">DRΛIN<br>CHECK</a>
   ```

   The visible label uses `Λ`, while the accessible name uses `A` and adds
   “home”; voice users cannot rely on the visible label matching the control's
   programmatic name. Default Axe reports zero violations and Lighthouse still
   scores 100 because this rule is experimental, but the supplied baseline
   requires all serious/critical findings to be cleared.

### Low

1. The HTTP parser's documented internal 32 KiB header guard accepts a request
   with a 33 KiB header and returns 202. The delimiter check runs before the
   size check after each 4 KiB read, allowing one-chunk overshoot. The public
   product does not claim this limit, so this is hardening rather than a core
   contract failure.

2. The demo banner's exit action is **View local setup**, not the demo-sandbox
   contract's prescribed **Start for real**. It reaches the correct real-use
   setup and is understandable, so this is a wording-level contract deviation.

## Clean-clone gates

I cloned the candidate without shared artifacts into
`/tmp/drain-check-clean-8-t1gmV7/repo`, detached at the candidate SHA, and ran
the documented commands.

| Gate | Result |
| --- | --- |
| `npm ci` | PASS; 96 packages, 0 vulnerabilities |
| `npm test` | PASS; 38/38 tests |
| `npm run typecheck` | PASS |
| `npm run build` | PASS; exact production output in `dist/site` |
| `cargo test --all-targets --all-features --locked` | PASS; 19/19 |
| `cargo test --doc --locked` | PASS; 1/1 |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | PASS |
| `cargo package --locked` | PASS; 10 files, 61.5 KiB / 17.1 KiB compressed |
| `cargo doc --no-deps --locked` | PASS |
| `npm audit --audit-level=high` | PASS; 0 vulnerabilities |

There is no separate web lint script. TypeScript type checking, Rust formatting,
and Clippy are the available static checks. Gate logs and timings are under
`/tmp/drain-check-clean-8-t1gmV7/`.

## Packaged CLI and end-to-end receiver

`cargo package --locked` produced `drain-check-0.1.1.crate`. I installed the
expanded package into a clean Cargo/install root and ran it from an unrelated
working directory.

- `drain-check --version` returns `0.1.1`; help lists listen, inspect, demo,
  forwarding, help, and version.
- `demo --json` runs from outside the repository and reports the embedded
  3-event/17-path sample in a unique `/tmp/drain-check-demo-*` directory.
- Normal inspection reports integer/number/boolean/null types correctly.
- Empty input returns a zero-event report with zero retention.
- Malformed input exits 1 without writing a report; zero duration and invalid
  or non-HTTP forwarding URLs exit 2. A URL containing spaces and quotes is
  encoded safely.
- Twenty simultaneous complete requests all returned 202 in 3.5–13.3 ms and
  produced a 20-event report.
- A single local client sent 21 requests inside one rolling second. Requests
  1–20 returned 202; request 21 returned **429** with
  **`Retry-After: 1`**. After 1.1 seconds, a malformed request returned 400 and
  the next valid request returned 202. The final report contained 21 accepted
  events.
- GET, missing Content-Length, empty body, invalid UTF-8, malformed NDJSON,
  and a body over 2 MiB returned 405, 400, 400, 400, 400, and 413 respectively.
  A valid body exactly 2 MiB returned 202.
- The listener announced and bound `127.0.0.1`; without `--save-sample`, unique
  submitted values were absent from both file/stdout reports and
  `bodies_saved` was false.

The observed request allowance is **20 accepted requests per rolling second
per local receiver**, with `429` and `Retry-After: 1` after the allowance.

## Live deployment, privacy, headers, and identity

- GitHub `origin/main` resolves to the exact candidate SHA.
- Live HTML, 404 HTML, JS, CSS, artwork, icons, robots, and sitemap all
  byte-for-byte match the clean candidate build by `cmp` and SHA-256.
- The live footer identifies `v0.1.1+92f01451faec`.
- A complete cold/demo browser flow issued only product-origin requests, set no
  cookies, and left local/session storage empty. Source inspection finds no
  analytics, telemetry, remote fonts/scripts, authentication, billing, AI, or
  unlock calls.
- Live responses include HSTS, `X-Content-Type-Options: nosniff`,
  `Referrer-Policy: strict-origin-when-cross-origin`, and a same-origin CSP with
  `frame-ancestors 'none'`. There are no CSP console errors.
- HTML uses `public, must-revalidate, max-age=30` and returned 304 for a matching
  ETag. Hashed JS/CSS use `public, max-age=31536000, immutable`.
- All live links return 200 except the intentional self-link on the designed
  404 page, which correctly remains 404. `robots.txt` and `sitemap.xml` return
  200 and list the public routes.

The public site is static and the receiver is a local CLI endpoint. There is no
service worker, offline claim, hosted persistence, account, or sign-in flow;
PWA update/offline and Entra authority checks are not applicable. Deterministic
local analysis is appropriate here, so the absence of an AI feature is not
missed leverage.

## Browser, accessibility, responsive behavior, and performance

- The factory `verify-url.sh` passes for `/` and `/?demo=1` after creating its
  required evidence directories.
- `/`, both demo forms, `/privacy`, `/terms`, and `/missing` were tested at
  1440×900 and 390×844. Each route has a route-specific title, `lang=en`, one
  h1, one main landmark, ordered headings, alt text, zero horizontal overflow,
  no missing labels, and no undersized interactive targets.
- Default Axe has zero violations of any severity on all 12 route/viewport
  combinations. The explicit experimental serious finding is reported above.
- Keyboard order begins with the visible skip link. All controls are reachable;
  Enter opens the demo, Space resets it, route changes move focus to the new h1,
  and Back restores the home route and heading focus. Focus is a visible 3 px
  amber outline with 4 px offset.
- Reduced motion sets animations/transitions to 0.00001 seconds. Simulated 200%
  text retains all content with no horizontal overflow.
- Normal routes have zero console or page errors. Directly loading the designed
  404 logs only the expected browser “resource 404” message.
- Production raw/gzip sizes are JS 11,341/4,270 bytes, CSS 6,784/2,210 bytes,
  fonts 0, and hero WebP 62,236 bytes. Initial Lighthouse transfer is 69 KiB;
  all supplied budgets pass.
- Fresh mobile Lighthouse 12.8.2 exits zero with Performance 100,
  Accessibility 100, Best Practices 100, and SEO 100; FCP 0.8 s, LCP 1.2 s,
  TBT 60 ms, and CLS 0.

## Required remediation

1. Count the original accepted event bytes, excluding only the NDJSON delimiter,
   instead of measuring compact re-serialized JSON; add tests enforcing the
   brief's 25% bound for spaced and representative payloads.
2. Encode field paths without collisions (for example, bracket-quoted JSONPath
   segments or JSON Pointer), and run sensitive-name matching against each raw
   source key rather than a split rendered path. Add dotted/bracketed key tests
   for field types, ignores, built-in detectors, and custom patterns.
3. Make the wordmark's accessible name contain its visible text and rerun the
   experimental Axe rule.
4. Enforce the header limit before accepting a delimiter found beyond 32 KiB,
   and align the demo exit label with the sandbox contract.

No product code was modified during this verification.
