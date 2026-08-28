# Independent verification — FAIL

- Candidate: `fc83213d0b4bef15d7d049310d3b6245903bf418`
- Live URL: <https://log-drain-contract-check.sociobot.in>
- Verified: 2026-08-28 UTC
- Environment: Rust/Cargo 1.98.0, Node 22.23.2, npm 10.9.8, Playwright 1.58.2
- Result: **FAIL — do not release this candidate.**

The live deployment is reachable and byte-for-byte matches the candidate's built HTML, JS, CSS, hero art, favicon, robots file, and sitemap. This is not a deployment-only failure. The candidate fails product correctness, CLI demo, receiver recovery/privacy, rate-limit, claims, accessibility, and quality-gate requirements.

## Mandatory first checks

### Claims gate

I started from clean `main` at the requested SHA. The initial pre-install invocations could not find `vitest`; after `npm ci` from the committed lockfile (95 packages, 0 vulnerabilities), every command in `.factory/claims.json` passed:

| Claim | Exact command | Result |
| --- | --- | --- |
| `local-only` | `npm test -- -t @claim:local-only` | PASS, 1 test, 556 ms |
| `discard-default` | `npm test -- -t @claim:discard-default` | PASS, 1 test, 508 ms |
| `sample-demo` | `npm test -- -t @claim:sample-demo` | PASS, 1 test, 543 ms |

The declared tests passing does not make the claims contract valid. `discard-default` only asserts that the sentence is rendered; it never exercises the receiver. `sample-demo` checks the route, heading, and banner but neither persistence nor the displayed report values. This violates the requirement that claim tests prove the observable outcome.

The inventory is also incomplete. Unlisted live/README claims include localhost binding, no telemetry, field/type and secret detection, exact sample metrics and retention numbers, no log-body transmission, free/open-source status, and the receiver not storing logs. Quantitative demo values have no quantitative claim test.

### Cold first-read

PASS on desktop and 390×844 mobile.

- What it does: inspects a log drain before it is forwarded.
- For whom: platform teams checking volume, fields, and privacy risks.
- First click: **Try it with sample data**, with “Opens a sample report. Nothing is saved.” beside/below it.
- The CTA is visible in the initial 390×844 viewport at y=781–828.

One click opens `/demo`, immediately showing a populated report and the persistent “Demo — sample data, nothing is saved” banner with Reset demo and Start for real.

## Release-blocking defects

### Critical

1. **One incomplete request terminates the receiver and loses the sample.** A listener started for five seconds received a body shorter than its declared `Content-Length`. Curl got an empty response (`curl` exit 52), the receiver exited 1 with `Could not read body: Resource temporarily unavailable`, and no report was written. Any client/network fault can end a sampling window and discard earlier accepted events.

2. **Malformed NDJSON is reported as successfully accepted.** `{not-json` returned HTTP 202 with an empty response and produced a zero-event report. A mixed payload containing two valid lines and one invalid line also returned 202, counted two events, and emitted no warning. This makes a bad drain look quiet or complete instead of telling the operator how to recover.

3. **The web demo's retention estimate is wrong by about 100×.** The real bundled sample produces 558.1 KiB for 7 days and 2.3 MiB for 30 days. `/demo` displays 5.7 KiB and 24.5 KiB. Retention is a core decision output in the researched brief.

### High

1. **The packaged CLI demo does not work after installation.** `cargo package --locked` and `cargo install --path target/package/drain-check-0.1.0` succeed, but running `drain-check demo --json` from a clean consumer directory exits 1: `Could not read examples/drain.ndjson: No such file or directory`. The command reads a repository-relative path at `src/main.rs:25` instead of embedding the sample. It also writes to one predictable global temp filename rather than an isolated temp directory.

2. **“Discards bodies after aggregation” is not the implementation.** During the entire receiver window, `src/main.rs:47-53` stores every parsed event in `events` and every raw request body in `saved`, even when `--save-sample` is absent. Aggregation happens only after the window. `bodies_saved: false` means “not written to disk,” not “discarded after aggregation.” This contradicts the landing page, privacy page, README, help text, brief, and declared claim.

3. **No API rate limit exists.** A 25-way burst of 100 POSTs yielded 100 HTTP 202 responses and zero 429 responses. No threshold was observed through 100 requests, and no `Retry-After` header was possible. The final report counted all 100 events. This fails the explicit endpoint requirement.

4. **The brief's false-positive handling constraint is missing.** `listen --help` has no ignore/suppression/configured-pattern option. A benign 32-character request ID is labeled a high-confidence secret, and there is no way to mark or suppress it. The report text is advice, not false-positive handling.

5. **The 390px landing page has two serious Axe violations.** `scrollable-region-focusable` affects `.terminal` and `.install > pre`; neither horizontally scrollable region is keyboard-focusable. Axe found no serious/critical issues on the other routes.

6. **Available quality gates do not all pass.** `cargo fmt --all -- --check` fails across both Rust sources. `cargo clippy --all-targets --all-features -- -D warnings` fails on `clippy::len-zero`. A strict TypeScript check fails because `@axe-core/playwright` resolves `playwright-core@1.62.1` while the pinned browser package uses `1.58.2`, producing an incompatible `Page` type in `site/e2e.test.ts:41`.

7. **The claims contract is incomplete and two declared tests do not prove their claims.** This is independently release-blocking under the supplied claims rules even though the three listed test commands return zero.

### Medium

- The CLI says “Press Ctrl-C to stop early,” but SIGINT exits without producing the report (`timeout -s INT ...` exit 124; report absent).
- Live responses omit the configured Content-Security-Policy. Consequently `frame-ancestors 'none'` is not enforced. The build copies `staticwebapp.config.json` to `dist/`, outside the deployed `dist/site/` root. HSTS, `X-Content-Type-Options`, and `Referrer-Policy` are present.
- Every response, including content-hashed JS/CSS, uses `Cache-Control: public, must-revalidate, max-age=30`; immutable asset caching is absent.
- At 200% root text size on a 390px viewport, document width grows to 471px and primary content/CTA is clipped horizontally.
- Mobile header/footer links are only 16–26px high, below the required 44px touch target.
- After keyboard activation of the demo link, focus lands on `<body>`, not the new `<h1>`; the heading is not focusable even though `render()` calls `h1.focus()`.
- A single event containing two array items reports `$.items[].id.present_in = 2`, so “present in” is occurrence count rather than event presence and can exceed the event count.
- `forwarding --url 'not a url'` exits 0 and emits an unusable configuration. `listen --duration 0 --port 0` also exits 0 while announcing the unreachable literal port 0.

### Low

- Unknown routes render the styled 404 screen but return HTTP 200.
- The Open Graph image is 630×331, not the required/claimed 1200×630.
- The footer exposes semantic version `v0.1.0` but no build/commit identity.
- The CLI-class landing page shows static terminal text, not the required self-hosted terminal recording.

## End-to-end CLI evidence

### Passing behavior

- `cargo test --all-targets --all-features --locked`: 3/3 unit tests pass.
- `cargo test --doc --locked`: passes, but there are zero doctests.
- `cargo build --release --locked`: passes; binary is 1.4 MiB.
- `cargo package --locked`: passes; 32 files, 200.7 KiB unpacked / 117.9 KiB compressed.
- Installed `drain-check --version`: `0.1.0`.
- Installed `inspect /dev/stdin --sample-seconds 10 --json`: correctly reports one event and the sensitive `$.password` field.
- Repository sample inspection: 3 events, 0.005 events/sec, 17 field paths, 3 findings, average 189 bytes, 558.1 KiB/7 days and 2.3 MiB/30 days.
- Empty input and a zero-second sample do not panic; they return a zero report.
- Missing file and invalid file input return exit 1 with a specific error. Unknown options return exit 2 with usage.
- Normal receiver POST returns 202 and reports all 3 bundled events and 3 findings.
- Explicit `--save-sample` returns `bodies_saved: true`; the saved file exactly matches the posted 571-byte sample.
- The receiver binds with `Ipv4Addr::LOCALHOST`; no public listener path exists in the CLI.
- 100 well-formed requests at 25-way concurrency were all processed.

### Failing behavior

- Clean-consumer installed demo: exit 1 due to missing relative `examples/drain.ndjson`.
- Invalid receiver payload: HTTP 202, zero events, no warning.
- Mixed receiver payload: HTTP 202, valid lines silently counted and invalid line dropped.
- Short declared body: receiver exit 1, no report.
- SIGINT: no report.
- Invalid forwarding URL: exit 0 and invalid template.
- False-positive case: a benign 32-character ID is a high-confidence secret with no suppression route.

## Live web, privacy, security, and deployment evidence

- Root, `/demo`, `/privacy`, and `/terms` return HTTPS 200 and render correct unique titles, one `<h1>`, one `<main>`, `lang=en`, and image alt text.
- Factory `verify-url.sh` passes: load 1116 ms, no console/page errors, title/lang/main/alt checks pass.
- Browser runs on all routes at 1440×900 and 390×844 recorded no console/page errors and no third-party origins.
- The demo writes no browser storage. Reset demo is keyboard-operable with Space and removes the reserved `demo:drain-check` key.
- No analytics, telemetry, remote font, third-party runtime script, authentication, billing, or AI endpoint exists. Sign-in checks are not applicable.
- `npm audit --audit-level=high`: 0 vulnerabilities.
- Candidate/live equality: root SHA-256 `a49c17bbcda25fd4205a1cd478a5fd984dfb271ff5cd6fb4f420d5ef2c42b531`; built JS, CSS, hero, favicon, robots, and sitemap also compare byte-for-byte.
- The deployment therefore matches the candidate despite lacking a visible commit identifier.
- This is a CLI with a static companion site, not a PWA; service-worker/offline-update checks are not applicable.
- This is not a hosted backend; health/build API checks are not applicable. The local receiver's concurrency, persistence, and rate limiting were tested above.

## Accessibility and responsive evidence

- Keyboard order is logical: skip link, wordmark, Demo, How it works, Privacy, sample CTA. Every tested focused element has a visible 3px amber outline with 4px offset.
- Reset demo works with Space. Links work with Enter. No keyboard trap was observed.
- Normal-size pages have no document-level horizontal overflow at 390px.
- Reduced motion computes to 0.01 ms animation duration.
- Axe serious/critical: desktop all routes 0; mobile demo/privacy/terms/404 0; mobile landing 2 serious findings.
- Dark-only presentation is explicitly justified in `.factory/design.md`; Axe reports no contrast violation.

## Performance and asset budgets

Lighthouse 12.8.2, live mobile profile:

| Category/metric | Result |
| --- | ---: |
| Performance | 99 |
| Accessibility | 100 |
| Best practices | 100 |
| SEO | 100 |
| FCP | 0.9 s |
| LCP | 1.3 s |
| TBT | 140 ms |
| CLS | 0 |

The explicit 390px Axe run is authoritative for the serious accessibility finding that Lighthouse's profile missed.

Initial live transfer is about 68.7 KiB: HTML 412 B encoded, JS 2.95 KiB, CSS 1.92 KiB, and hero WebP 62.24 KiB. Raw JS is 7.04 KiB and raw CSS is 5.63 KiB. All stated byte budgets pass. Lighthouse notes the LCP image is discovered late because markup is rendered by JS and is oversized for mobile, but measured LCP still passes.

## Required repair order

1. Make request parsing transactional and non-fatal: reject malformed NDJSON with 4xx, continue the window, and preserve already aggregated data.
2. Aggregate per event/request and drop raw/parsed bodies immediately unless `--save-sample` is explicitly set.
3. Embed the demo sample in the binary, use an isolated unpredictable temp directory, and retest from an installed package outside the repo.
4. Generate browser demo values from the same report fixture/code path; add quantitative parity claim tests.
5. Add configured sensitive-field patterns and explicit ignore/suppression handling.
6. Add rate limiting with 429 and `Retry-After`; test its threshold.
7. Replace text-presence claim tests with outcome tests and inventory every public claim.
8. Fix all format, Clippy, TypeScript, Axe, keyboard focus, touch target, and 200% text issues.
9. Deploy the CSP/config inside the actual publish root and add immutable caching for hashed assets.
