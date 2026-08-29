# Independent verification 5 — PASS

- Candidate: `ff327ece23be77a1c4720adb599c0b52828990ac`
- Live URL: <https://log-drain-contract-check.sociobot.in>
- Verified: 2026-08-29 UTC
- Environment: Node 22.23.2, npm 10.9.8, Rust/Cargo 1.98.0, Playwright 1.58.2.
- Decision: **PASS — the candidate is accepted.**

The supplied checkout began clean at the candidate SHA. The live footer reports
`v0.1.0+ff327ece23be`; after a fresh production build, SHA-256 of live and
local `main-DqrGKSm-.js`, `main-DHXSyaAv.css`, and `drain-console.webp` matched
exactly. This rules out the previously reported deployment-only discrepancy.

## Mandatory claims gate

`.factory/claims.json` exists with 13 claims. Following `npm ci` (96 packages,
0 vulnerabilities), every literal command was run separately and sequentially
from the clean candidate checkout. All passed:

| Claim | Result | Observable evidence |
| --- | --- | --- |
| `sample-demo` | PASS | One click reaches the bundled report with 3 events, 17 paths, 558.1 KiB/7 days, and 2.3 MiB/30 days. |
| `local-only` | PASS | Browser claim test records same-origin traffic/no storage; Rust regression proves loopback bind. |
| `discard-default` | PASS | Unique accepted value is absent from report. |
| `contract-report` | PASS | Fixture proves rate, size, fields/types, findings, and retention. |
| `forwarding-config` | PASS | Separate forwarding configuration renders and URL validation/encoding regression passes. |
| `source-checkout` | PASS | Fresh public GitHub clone runs `cargo run --locked -- --help`. |
| `false-positive-controls` | PASS | Custom pattern and reviewed exact/prefix suppression pass. |
| `rate-limit` | PASS | Configured limit returns 429 with `Retry-After: 1`. |
| `request-recovery` | PASS | Bad/incomplete request returns 400 without losing accepted event. |
| `explicit-save` | PASS | Only accepted bodies save when explicitly requested. |
| `interrupt-report` | PASS | SIGINT writes a valid partial report. |
| `portable-demo` | PASS | Installed binary runs bundled demo from a fresh consumer directory. |
| `mit-license` | PASS | Shipped license contains the MIT grant. |

An earlier interrupted batch caused one temporary `forwarding-config` browser
test failure against a stale local Vite server. It passed when rerun alone,
in the complete 31-test suite, and in the final sequential clean replay above;
it is not reproducible product evidence.

## Cold first-read and demo

**PASS.** A cold live visit says:

- What it does: “Inspect a log drain before forwarding.”
- Who it is for: platform teams checking volume, field types, and sensitive data.
- What to click: “Try it with sample data,” with “Opens the bundled report. Writes no browser data.”

The action is visible at 390×844 and opens `/?demo=1` in one click. The demo
has the persistent “Demo — sample data, nothing is saved” banner, keyboard
operable Reset demo, and Start for real. Reset leaves unrelated storage alone;
fresh demo contexts had empty local/session storage and no cookies.

## Product exercise: CLI and receiver

- `drain-check inspect examples/drain.ndjson --sample-seconds 600 --json` returned 3 events, 17 fields, 3 findings, 558.1 KiB/7 days, 2.3 MiB/30 days, and `bodies_saved: false`.
- Invalid `forwarding --url ftp://…` and `inspect --sample-seconds 0` exit 2 with actionable validation messages.
- A real loopback listener accepted a valid POST (202), rejected malformed NDJSON (400), then accepted another valid POST (202). Its report retained two events, `bodies_saved: false`, and did not contain the unique secret-shaped value.
- With `--rate-limit 1`, the first POST was 202 and the second 429 with `Retry-After: 1`. The documented default allowance is **20 accepted requests per rolling second**.
- `cargo package --locked` created and verified the package (10 files, 58.5 KiB; 16.5 KiB compressed). A `cargo install --path . --root <temp>` consumer invoked `drain-check demo --json` from another directory, produced the expected 3/17/3 report, and wrote `/tmp/drain-check-demo-*/report.json`, outside the repository.

There is no hosted product API, authentication, payment/unlock endpoint, PWA,
or service worker. Entra, backend-concurrency/persistence, and offline-update
checks therefore do not apply.

## Local quality gates

All passed:

```text
npm ci
npm test                                      # 31 passed
npm run typecheck
npm run build                                 # writes dist/site
cargo test --all-targets --all-features --locked  # 15 passed
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --locked
```

Exact production-build budgets: JS 11.33 kB / **4.26 kB gzip**, CSS 6.71 kB /
**2.21 kB gzip**, hero WebP 62.2 kB. There are no remote fonts or scripts.

## Live deployment, privacy, accessibility, and headers

- `/`, `/?demo=1`, `/demo`, `/privacy`, and `/terms` return 200; `/missing` is the designed HTTP 404.
- Playwright Axe had zero serious/critical findings on all six routes at both 1440×900 and 390×844. Every route has one H1 and one main landmark. The mobile first action is visible; no overflow was observed.
- Keyboard focus is a visible 3 px `#ffd66b` outline. Reset works with Space. Reduced motion changes banner animation duration to 0.01 ms.
- Fresh landing-to-demo traffic was exactly the document, same-origin JS/CSS, and same-origin hero image. No third-party requests, cookies, localStorage, or sessionStorage writes occurred. No normal-route console or page errors occurred.
- Live headers include `Content-Security-Policy: default-src 'self' … frame-ancestors 'none'`, HSTS, `nosniff`, and strict-origin referrer policy. Hashed JS/CSS are immutable for one year; HTML revalidates after 30 seconds.
- The factory `verify-url.sh` passed in 752 ms with title, `lang=en`, one H1, main landmark, image alt text, button labels, and no console errors. Evidence is in `.factory/verification-artifacts/verify-5/`.

Direct navigation to the intentionally HTTP-404 route produces Chrome's expected
network “Failed to load resource: 404” console message; it has no page error and
is inherent to serving the required real 404 status. It is not emitted on normal
routes.

## Defects by severity

No release-blocking, high, medium, or low defects found.
