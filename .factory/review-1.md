# Adversarial first-read review 1 — Drain Check

**Verdict: FAIL.** One blocking finding prevents the advertised local install from working. There are also report/demo honesty, claims-coverage, and plain-language findings below. A PASS requires zero findings.

## Cold first read

I opened `https://log-drain-contract-check.sociobot.in/` in new Playwright browser contexts at 390 × 844 and 1440 × 900, without scrolling.

- **What it does:** inspect a short, local log-drain sample before forwarding it, and show volume, fields, and privacy risks.
- **For whom:** platform teams that are turning on a log drain.
- **What to click first:** **Try it with sample data**; the adjacent text says it opens a sample report without saving data.

Those three answers were available on the first screen at both widths. The mobile primary action was fully visible. The first-read check itself passes.

## Findings

### F-1-1 — BLOCKING — the advertised installation command fails

- **Location / exact quote:** landing page, “Start a bounded receiver”: `cargo install drain-check`.
- **Evidence:** from a fresh temporary install root on 2026-08-29, `cargo install --root <temp> drain-check` returned: `error: could not find \`drain-check\` in registry \`crates-io\` with version \`*\``. `cargo search drain-check --limit 5` returned no package. The current handoff also says publication is still a future factory workflow.
- **Why this fails first use:** this is the only copy-paste installation command on the product page. A visitor who moves from the successful demo to the real job immediately gets a registry error. The nearby “# or clone this repository” is only a comment; it supplies neither a repository link nor runnable clone instructions.
- **Concrete fix:** either publish the exact version and add a clean-machine `cargo install drain-check` claim test, or remove the unavailable command. Until publication, show a public, copy-pasteable path, for example `git clone https://github.com/B-Divyesh/sf-log-drain-contract-check.git && cd sf-log-drain-contract-check && cargo run -- listen --duration 600 --port 8787`, and make the repository name a link. Test that path in a fresh temporary directory.

### F-1-2 — MAJOR — the site says the report includes a forwarding template, but the report does not

- **Location / exact quote:** README, “What the report contains”: “A destination-neutral HTTP forwarding template.” The live demo also labels a static placeholder block “Forwarding template.”
- **Evidence:** `cargo run -- demo --json` writes `"forwarding": "Forward only after review. Keep the receiving URL in your platform's secret store."`; it does not contain a forwarding configuration. The separate `cargo run -- forwarding --url https://receiver.example/logs` command does generate one.
- **Why this misleads:** a reader can reasonably expect the sample report being reviewed to contain a generated configuration. Instead, the report contains advice and the site shows an unrelated, hard-coded placeholder configuration.
- **Concrete fix:** move this capability into a separately named “Generate a forwarding configuration” section and show the real `forwarding --url` input and output. Change the report list to “A forwarding recommendation” unless the report is changed to contain a real template. Add a `forwarding-config` entry to `.factory/claims.json` with a test that asserts a supplied URL is safely rendered in the generated configuration.

### F-1-3 — MAJOR — the sample's displayed risk count cannot be reconciled with the demo report

- **Location / exact quote:** landing recording: “Reviewed 3 events in 600s. 17 fields. 3 possible risks.” Demo metrics: “2 review fields.”
- **Evidence:** the sample has three detector findings, but `findingMarkup()` stores results in a map keyed by path, so the two findings on `$.request.authorization` collapse into one displayed article. The demo gives no “3 findings across 2 fields” explanation and does not show the omitted detector.
- **Why this misleads:** the first try-out is supposed to prove the product with realistic sample data. A visitor cannot see the third advertised risk or determine why the count differs.
- **Concrete fix:** label the metric “3 findings across 2 fields” and render both detectors for the authorization field, or change the recording to the exact count the demo deliberately shows. Extend `@claim:sample-demo` to assert the visible count and all three detector labels.

### F-1-4 — MINOR — privacy and product claims are not all listed in the claims contract

- **Locations / exact quotes:** landing, “It does not store logs, search logs, or send them to a vendor.” README, “It is not a log destination.” README, “There is no telemetry.”
- **Why this is a finding:** none of those statements has an exact `.factory/claims.json` entry. `local-only` only asserts no third-party browser requests/storage and loopback binding; it does not prove “does not search logs,” “not a log destination,” or the broader “no telemetry” wording. `discard-default` proves bodies/values are discarded, not every clause in the landing statement.
- **Concrete fix:** remove the untestable clauses, or add narrow claims and observable tests. For example: “The receiver discards accepted bodies by default” (existing discard test); “The website sends no requests except to this origin” (request-log test); and “The receiver binds to 127.0.0.1” (existing Rust test). Do not retain “does not search logs” or “no telemetry” unless their observable definitions are tested.

### F-1-5 — MINOR — landing eyebrows use mood language instead of section names

- **Location / exact quotes:** “LOCAL PRE-FLIGHT / 10-MINUTE WINDOW” and “THE SHORT PATH.”
- **Why this is a finding:** “pre-flight” is a metaphor and “short path” carries no information when heard out of context. They make a cold visitor translate the copy before learning what section they are in.
- **Concrete fix:** rewrite the first as “LOCAL 10-MINUTE SAMPLE”; remove “THE SHORT PATH” because the following heading already says “Review a drain in three steps,” or rewrite it as “HOW IT WORKS.”

### F-1-6 — MINOR — the landing uses inconsistent, unexplained names for the same output

- **Location / exact quotes:** “Read the contract.” followed by “Check fields and likely secrets.” Elsewhere the page says “sample report” and “report.” “Forward with intent.” is also a mood slogan rather than an action.
- **Why this is a finding:** “contract” is never defined, and it competes with “report” for the same result. “With intent” does not tell a visitor what happens next.
- **Concrete fix:** use “Review the report.” and “Check fields and likely sensitive data.” Replace “Forward with intent.” with “Generate a forwarding configuration.”

### F-1-7 — MINOR — non-landing route headings are not plain page names

- **Location / exact quotes:** `/terms` h1, “Use Drain Check at your own boundary”; `/missing` h1, “That screen is not in this receiver.”
- **Why this is a finding:** “boundary” and “receiver” do not identify the page's purpose to a screen-reader user reading headings out of context.
- **Concrete fix:** use “Terms for using Drain Check” and “Page not found.” The supporting text can retain the local-receiver wording.

## Copy audit

Word counts use whitespace-delimited words; an inline command is one word. Commands, product names, navigation labels, and headings that are not sentences are listed separately. No audited prose sentence exceeds 22 words. Flags above are referenced in the final column.

### Landing page prose

| Location | Sentence | Words | Flag |
| --- | --- | ---: | --- |
| Hero lede | For platform teams who need volume, fields, and privacy risks before a drain stays on. | 15 | — |
| Hero action note | Opens a sample report. | 4 | — |
| Hero action note | Nothing is saved. | 3 | — |
| Hero fact | Listens only on your machine. | 5 | Claim coverage: F-1-4 |
| Hero fact | Discards bodies after aggregation. | 4 | — |
| Hero fact | Free under the MIT License. | 5 | — |
| Recording help | A text recording of the bundled CLI demo. | 8 | — |
| Recording help | Use the replay button to play it again. | 8 | — |
| Recording | Reviewed 3 events in 600s. | 5 | F-1-3 |
| Recording | 17 fields. | 2 | — |
| Recording | 3 possible risks. | 3 | F-1-3; terminology differs from “review fields” |
| Recording | Report: `/tmp/drain-check-demo-[unique]/report.json` | 2 | — |
| Steps | Run one bounded window. | 4 | — |
| Steps | Check fields and likely secrets. | 5 | F-1-6 (“secrets” differs from later “sensitive fields”) |
| Steps | Use the generated template. | 4 | F-1-2 |
| Limits | It does not store logs, search logs, or send them to a vendor. | 13 | F-1-4 |
| Limits | Use `--save-sample` only when you choose to retain accepted bodies. | 10 | — |
| Local setup | Then point your temporary HTTP drain to `http://127.0.0.1:8787/`. | 8 | — |
| Local setup | Use `--ignore-field '$.request_id'` to suppress a reviewed false positive. | 9 | — |
| Footer | Drain Check samples a log drain before you forward it. | 10 | — |

### Landing page labels and non-sentence copy

| Location | Copy | Words | Flag |
| --- | --- | ---: | --- |
| Header | DRΛIN CHECK | 2 | — |
| Header | Demo | 1 | — |
| Header | How it works | 3 | — |
| Header | Privacy | 1 | — |
| Hero eyebrow | LOCAL PRE-FLIGHT / 10-MINUTE WINDOW | 4 | F-1-5 |
| H1 | Inspect a log drain before forwarding | 6 | — |
| Primary action | Try it with sample data | 5 | — |
| Art caption | THE RECEIVER STAYS LOCAL | 4 | — |
| Button | Replay recording | 2 | — |
| Steps eyebrow | THE SHORT PATH | 3 | F-1-5 |
| Steps H2 | Review a drain in three steps | 6 | — |
| Step label | Listen locally. | 2 | — |
| Step label | Read the contract. | 3 | F-1-6 |
| Step label | Forward with intent. | 3 | F-1-6 |
| Limits H2 | What Drain Check does not do | 6 | — |
| Setup eyebrow | RUN IT LOCALLY | 3 | — |
| Setup H2 | Start a bounded receiver | 4 | F-1-1 (the following install command fails) |

### README prose

| README location | Sentence | Words | Flag |
| --- | --- | ---: | --- |
| 3 | Inspect a log drain before forwarding it. | 7 | — |
| 5 | Drain Check is for small platform teams adding a managed drain. | 11 | — |
| 5 | It opens a bounded local receiver, summarizes volume and field types, flags likely secrets, and writes a JSON report. | 19 | — |
| 5 | It is not a log destination. | 6 | F-1-4 |
| 9 | You need Rust 1.75+ and Node 20+ to build the docs site. | 12 | — |
| 13 | Uses the sample embedded in the binary and prints the unique report directory. | 11 | — |
| 16 | Point a temporary HTTP drain to `http://127.0.0.1:8787/`. | 7 | — |
| 19 | The listener binds only to `127.0.0.1`. | 6 | — |
| 19 | It aggregates accepted events as they arrive, then drops their parsed values and bodies. | 13 | — |
| 19 | Pass `--save-sample sample.ndjson` only when you intentionally want accepted bodies written to disk. | 12 | — |
| 19 | `--output` and `--save-sample` must name different files; Drain Check refuses a collision before opening the listener. | 16 | — |
| 21 | Malformed or incomplete requests return HTTP 400 without ending the sample window. | 11 | — |
| 21 | The default rolling limit accepts 20 requests per second, then returns HTTP 429 with `Retry-After: 1`. | 16 | — |
| 21 | Change it with `--rate-limit`. | 4 | — |
| 23 | You can also inspect an existing newline-delimited JSON file. | 9 | — |
| 30 | `inspect --sample-seconds` must be at least 1. | 5 | — |
| 30 | `forwarding` accepts a parsed HTTP(S) URL and safely encodes it in the generated configuration. | 14 | F-1-2 (no listed forwarding-config claim) |
| 30 | `--json` prints the report to standard output for scripts. | 9 | — |
| 30 | Add `--sensitive-field session_key` for a team-specific field name. | 8 | — |
| 30 | Add `--ignore-field '$.request_id'` after reviewing a false positive. | 8 | — |
| 30 | A trailing `*` ignores a path prefix. | 6 | — |
| 30 | `cargo run -- --help` lists all commands and options. | 7 | — |
| 34 | Event count, average body size, and events per second. | 9 | — |
| 35 | Field paths, observed types, and how many events contained each path. | 11 | — |
| 36 | Conservative risk findings for field names, token-shaped values, and email-shaped values. | 11 | — |
| 37 | Retention estimates for 7 and 30 days. | 7 | — |
| 38 | A destination-neutral HTTP forwarding template. | 6 | F-1-2; “destination-neutral” is jargon |
| 40 | Risk findings are prompts for review. | 6 | — |
| 40 | They do not prove a value is sensitive, and they cannot find every secret. | 14 | — |
| 40 | The bundled sample reports 3 events, 17 field paths, 558.1 KiB for 7 days, and 2.3 MiB for 30 days. | 17 | — |
| 44 | Open the web demo for a self-contained sample report. | 9 | Replace “self-contained” with “bundled”; “sandbox contract” below is jargon |
| 44 | The command-line equivalent is `cargo run -- demo --json`. | 5 | — |
| 44 | See `.factory/demo.md` for the sandbox contract. | 6 | Rewrite: “See `.factory/demo.md` for demo storage and reset details.” |
| 59 | The static site build lands in `dist/site` with its deployment configuration at that root. | 13 | — |
| 59 | `cargo package` prepares the CLI crate for publishing; this repository does not publish it. | 12 | Supports F-1-1 |
| 63 | There is no telemetry. | 4 | F-1-4 |
| 63 | The CLI only receives data on its loopback listener. | 9 | — |
| 63 | Read the deployed Privacy page and Terms. | 6 | — |
| 63 | Licensed under MIT. | 3 | — |

README headings and shell commands are labels/examples rather than sentences. Their result-naming verbs are appropriate except for the unavailable landing installation path in F-1-1.

## Demo, privacy, and CLI sandbox checks

- Clicking **Try it with sample data** from a new context opened `/demo` in one action. Its first screen immediately showed the bundled report (3 events, 17 paths, retention figures, detector review).
- The persistent banner read exactly “Demo — sample data, nothing is saved” and included **Reset demo** and **Start for real**.
- Before reset, browser `localStorage` and `sessionStorage` were empty. I inserted a sentinel `real:drain-check=keep`, activated Reset demo, and confirmed the sentinel remained unchanged; only the documented `demo:drain-check` namespace can be removed. No browser data is written by the demo.
- Live Playwright request log for `/` contained only the page, same-origin JS/CSS, and same-origin hero image. No cookies, third-party requests, console errors, or external fonts/scripts were observed. The product makes no offline claim, so no offline assertion was applicable.
- From a temporary directory, `cargo run --manifest-path /work/repo/Cargo.toml -- demo --json` used the bundled sample and wrote a report to a unique `/tmp/drain-check-demo-*` directory. The portable-demo claim test independently passed from the fresh clone.

## Claims verification

I cloned the repository into a new temporary directory, ran `npm ci`, then executed every command in `.factory/claims.json` separately. All 11 passed; no listed claim test failed and no listed claim remains untested.

| Claim id | Result |
| --- | --- |
| `sample-demo` | PASS |
| `local-only` | PASS |
| `discard-default` | PASS |
| `contract-report` | PASS |
| `false-positive-controls` | PASS |
| `rate-limit` | PASS |
| `request-recovery` | PASS |
| `explicit-save` | PASS |
| `interrupt-report` | PASS |
| `portable-demo` | PASS |
| `mit-license` | PASS |

`npm test` also passed locally (25 tests), as did `npm run typecheck` and `npm run build`; the production build emitted 4.14 kB gzip JavaScript.

## History, structure, and visual checks

- There are no earlier `.factory/review-*.md` or `.factory/polish-*.md` files, so there are no earlier finding IDs to re-verify. I read the current handoff and verification material; its “publish when ready” note corroborates F-1-1 rather than resolving it.
- Live `/`, `/demo`, `/privacy`, and `/terms` each returned 200 with a route-specific title, one h1, description, and canonical URL. `/missing` returned a designed 404 with status 404. Favicon, apple touch icon, OG/Twitter metadata, `robots.txt`, and `sitemap.xml` are present.
- All crawled internal links returned 200. Keyboard navigation to the demo moved focus to its h1; browser Back returned home and focused its h1. No live console errors occurred.
- The header/footer, skip link, navigation count, focus styling, target sizing, mobile layout, reduced-motion rule, and local Axe serious/critical checks passed. The 390 px page kept the main action in view.
- The pixel/demoscene receiver art, locally hosted assets, and dark instrument-panel styling are distinct from a generic SaaS template and match `.factory/design.md`.
- The brief does not imply an AI step: inspection, secret detection, reporting, and forwarding configuration are deterministic local tasks. No AI feature is missing or decorative.

## What would make this perfect

Publish a verified installation artifact (or give one real clone command), make the report and demo show exactly what the product actually generates, put every user-relevant promise under a narrow executable claim, and replace the remaining metaphor/jargon headings with direct names. Re-run this full cold review after those changes; only then can the verdict become PASS.
