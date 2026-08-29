# Drain Check repair handoff

## Release status

PASS. Release-blocking findings from verifier report `6698a10d4406833144deb5c2e6a2935df2563a66` against candidate `fc83213d0b4bef15d7d049310d3b6245903bf418` are repaired. The deployed product code is commit `771756d83a27b8c38266853abeecec3e286be7d5`.

- Product: <https://log-drain-contract-check.sociobot.in>
- Artifact class: CLI with a static landing/docs site
- Deployment: Azure Static Web Apps, production environment
- Deployment ID: `ffdc1309-a5c5-4691-ad5b-5c5fea8b4ee3`
- Deployed artifact: `dist/site`

## Repairs

- Replaced end-of-window event/body batching with immediate metadata aggregation. Parsed values are dropped after each event. Raw request bodies are dropped after a transaction unless `--save-sample` is explicit.
- Made NDJSON request handling transactional. Empty, malformed, mixed-validity, incomplete, oversized, and invalid-header requests get a clear 4xx response and do not terminate the listener or alter prior aggregates.
- Added a rolling request limit. The default is 20 valid requests per second; excess requests receive HTTP 429 and `Retry-After: 1`.
- Added `--sensitive-field` and `--ignore-field` controls. Ignore values accept an exact JSON path or a prefix ending in `*`.
- Added SIGINT handling. Ctrl-C ends the window and writes a report using the actual elapsed sampling time.
- Embedded the sample NDJSON in the binary. `drain-check demo` now works from any directory and creates a unique temporary report directory.
- Corrected array field presence to count events, not repeated array values.
- Rejected zero listener duration, port zero, zero rate limit, and forwarding values without an HTTP(S) URL and host.
- Generated the browser demo report through the Rust analyzer during every site build. The displayed 7-day and 30-day values are now `558.1 KiB` and `2.3 MiB`.
- Replaced copy-presence checks with observable claim tests and inventoried every public product claim in `.factory/claims.json`.
- Fixed mobile scroll-region accessibility, 44px navigation/footer targets, route-heading focus, 200% text overflow, and reduced-motion behavior.
- Added a self-hosted, selectable CLI recording with keyboard replay. The first action now stays within the 390×844 first viewport.
- Pinned Playwright and `playwright-core` to 1.58.2 and restored strict TypeScript compatibility.
- Put `staticwebapp.config.json` inside `dist/site`, added CSP and security headers, and added immutable caching for hashed assets.
- Added a real static 404 response, a 1200×630 Open Graph image, route metadata, and a visible build identifier.
- Restricted the publishable crate to its runtime sources, embedded sample, and documentation.

## Regression coverage

Rust tests cover exact sample metrics, value-free reports, per-event array presence, custom/ignored detectors, transactional malformed NDJSON, incomplete bodies, listener survival, rate-limit headers, explicit sample saving, loopback binding, invalid CLI values, installed demo portability, and SIGINT report writing. The library example is a compiling doctest.

Browser tests cover every declared claim, all routes at 1440×900 and 390×844, serious/critical Axe findings, console errors, one H1/main, first-screen CTA visibility, keyboard navigation and reset, route focus, reduced motion, 44px targets, 200% text resizing, no horizontal overflow, no third-party requests, and empty browser storage.

## Verification evidence

Clean toolchain: Rust/Cargo 1.98.0, Node 22.23.2, npm 10.9.8, Playwright 1.58.2.

```sh
npm ci
# 96 packages installed; 0 vulnerabilities

cargo fmt --all -- --check
cargo test --all-targets --all-features --locked
# 13 tests passed: 4 library, 7 binary, 2 release integration

cargo test --doc --locked
# 1 doctest passed

cargo clippy --all-targets --all-features --locked -- -D warnings
npm run typecheck
npm test
# 25 browser and claim tests passed

npm run build
# dist/site; JS 10.65 KB raw / 4.14 KB gzip; CSS 6.56 KB raw / 2.18 KB gzip

cargo build --release --locked
# Linux binary: 1,542,216 bytes

cargo package --locked
# 10 files; 52.4 KiB unpacked / 14.9 KiB compressed
```

Every exact command in `.factory/claims.json` passed independently from a clean browser context.

Package consumer verification:

```sh
cargo install --path target/package/drain-check-0.1.0 --root "$TEMP_ROOT" --locked
cd "$TEMP_ROOT"
./bin/drain-check demo --json
```

Result: exit 0 from outside the repository; 3 events; 17 fields; `558.1 KiB` and `2.3 MiB`; `bodies_saved: false`; report under `/tmp/drain-check-demo-<random>/report.json`.

Production browser verification:

- `/`, `/demo`, `/privacy`, and `/terms`: HTTP 200.
- `/definitely-missing`: styled page with HTTP 404.
- Desktop 1440×900 and mobile 390×844: zero serious/critical Axe violations, zero application console errors, zero third-party origins, and no horizontal overflow.
- Keyboard activation of the sample link moves focus to the `/demo` H1.
- Factory `verify-url.sh`: 816 ms load, correct title/lang/main/H1/alt text, zero errors.
- Deployed `index.html` SHA-256 equals the local artifact: `85c7fde86184a9c350ae694a09c3e8a7845df612679afcef06d37e80eba5e1ba`.
- Live footer identity: `v0.1.0+771756d83a27`.
- CSP includes `frame-ancestors 'none'`; HSTS, `X-Content-Type-Options`, and `Referrer-Policy` are present.
- Hashed JS response: `Cache-Control: public, max-age=31536000, immutable`.
- Live Lighthouse mobile: performance 100, accessibility 100, best practices 100, SEO 100; FCP 0.8 s, LCP 1.2 s, TBT 0 ms, CLS 0.

## Privacy, offline, identity, and service scope

The website performs no analytics, remote font, third-party script, storage, authentication, billing, or AI request. The CLI has no telemetry or outbound client. Its only network role is the loopback HTTP listener requested by the operator. Browser request recording and CLI report inspection cover these statements.

This product remains a CLI with a static companion site, not a PWA or hosted backend. Service-worker update, hosted health API, authentication, payment, and live AI checks are not applicable. The installed CLI demo and analyzer run without network access.

## Known gaps and next steps

No known release-blocking gaps remain. Registry publication is intentionally left to the factory owner; the verified crate is ready for publishing with `cargo publish` when credentials and release approval are available.
