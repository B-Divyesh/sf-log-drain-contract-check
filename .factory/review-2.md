# Adversarial first-read review 2 — Drain Check

**Verdict: FAIL.** The live product is clear and genuinely tryable, and every declared claim test passed. Two minor findings remain: several README promises are not individually declared in the claims contract, and one demo action does not say what result it produces. A PASS requires zero findings.

## Cold first read

I opened `https://log-drain-contract-check.sociobot.in/` in new browser contexts at 390 × 844 and 1440 × 900, at scroll position zero.

- **What it does:** it inspects a local log-drain sample before it is forwarded, showing event volume, fields, and potentially sensitive data.
- **For whom:** platform teams checking a drain before enabling it.
- **What to click first:** **Try it with sample data**. The adjacent text says it opens the bundled report and writes no browser data.

All three answers are present on the first screen at both sizes. At 390 px, the primary action occupies y=423–470 in an 844 px viewport. It is visible without scrolling. This check passes.

## Findings

### F-2-1 — MINOR — several README claims are outside `.factory/claims.json`

- **Location / exact quotes:** README line 9, “You need Rust 1.75+ to run the CLI.” Line 21, “`--output` and `--save-sample` must name different files; Drain Check refuses a collision before opening the listener.” Line 31, “`inspect --sample-seconds` must be at least 1.” “`--json` prints the report to standard output for scripts.” and “`cargo run -- --help` lists all commands and options.” Line 41, “They do not prove a value is sensitive, and they cannot find every secret.”
- **Why this is a finding:** these are specific statements a CLI user can rely on, but none has a matching claims entry. Some behavior happens to be exercised incidentally by a broader Rust test, but the contract requires one listed, observable test for every claim; an incidental assertion is not a declared promise. The detector-limit sentence is not observable as written and therefore cannot remain under the claims rule without a measurable definition.
- **Concrete fix:** add narrow entries and tagged tests for the Rust 1.75 minimum (compile in that toolchain), distinct output/sample paths, invalid sample duration, JSON stdout, and complete help text. For the detector limitation, replace the untestable sentence with the usable instruction “Review every finding before forwarding.” Alternatively, define an observable boundary and test it. Do not claim PASS while these statements remain unlisted.

### F-2-2 — MINOR — the demo exit control does not name its result

- **Location / exact quote:** demo banner button/link, “Start for real”.
- **Why this is a finding:** this is a CLI product. The action only routes to the landing page’s local setup instructions; it neither starts a receiver nor distinguishes itself from a real run. A first-time visitor can reasonably expect it to begin a local sample.
- **Concrete fix:** rename it **View local setup** (or **Leave demo and view local setup**) and retain the existing route to `/`. This names the resulting screen and meets the result-naming-button rule.

## Copy audit

Word counts use whitespace-delimited words, treating one inline command or URL as one word. Commands, navigation labels, metric fragments, and headings are included where they carry a sentence or instruction. No landing or README sentence exceeds 22 words. The flags point to the findings above.

### Landing page

| Location | Sentence or instruction | Words | Flag |
| --- | --- | ---: | --- |
| H1 | Inspect a log drain before forwarding. | 6 | — |
| Hero lede | For platform teams checking volume, field types, and sensitive data before enabling a log drain. | 15 | — |
| Hero note | Opens the bundled report. | 4 | — |
| Hero note | Writes no browser data. | 4 | — |
| Hero fact | The receiver binds to `127.0.0.1`. | 5 | — |
| Hero fact | Accepted bodies are discarded by default. | 6 | — |
| Hero fact | Free under the MIT License. | 5 | — |
| Art caption | The receiver stays local. | 4 | — |
| Recording help | A text recording of the bundled CLI demo. | 8 | — |
| Recording help | Use the replay button to play it again. | 8 | — |
| Recording | Reviewed 3 events in 600s. | 5 | — |
| Recording | 17 fields. | 2 | — |
| Recording | 3 findings across 2 fields. | 6 | — |
| Recording | Report: `/tmp/drain-check-demo-[unique]/report.json` | 2 | — |
| Recording | Send POST requests to this endpoint after report review. | 9 | — |
| Step | Listen locally. | 2 | — |
| Step | Run one bounded window. | 4 | — |
| Step | Review the report. | 3 | — |
| Step | Check fields and likely sensitive data. | 6 | — |
| Step | Generate a forwarding configuration. | 4 | — |
| Step | Review the generated configuration. | 4 | — |
| Retention section | The receiver discards accepted bodies after aggregation by default. | 9 | — |
| Retention section | Saving accepted bodies requires `--save-sample`. | 5 | — |
| Local setup | Clone the public source repository on GitHub, then run the receiver. | 11 | — |
| Local setup | Point your temporary HTTP drain to `http://127.0.0.1:8787/`. | 8 | — |
| Local setup | Use `--ignore-field '$.request_id'` to suppress a reviewed false positive. | 8 | — |
| Footer | Drain Check samples a log drain before you forward it. | 10 | — |

| Landing labels, headings, and controls | Review | Flag |
| --- | --- | --- |
| `LOCAL 10-MINUTE SAMPLE`; `HOW IT WORKS`; `RUN IT LOCALLY` | Names a section; no mood-only heading. | — |
| `Review a drain in three steps`; `What Drain Check does not retain`; `Start a bounded receiver` | Direct, contextual headings. | — |
| `Try it with sample data`; `Replay recording`; `Read local setup` | Verbs name their output or destination. | — |
| `Start for real` | Does not name the landing/setup result. | F-2-2 |

Terminology is consistent: **drain** (integration), **sample** (bounded events), **report** (CLI output), **body** (payload), **finding** (detector warning), **sensitive data** (data to review), and **forwarding configuration** (destination setup). The platform-specific terms “receiver,” “field type,” and “HTTP” are necessary to the stated platform-team audience and are explained by nearby copy or commands.

### README

| Line | Sentence or claim | Words | Flag |
| --- | --- | ---: | --- |
| 3 | Inspect a log drain before forwarding it. | 7 | — |
| 5 | Drain Check is for small platform teams adding a managed drain. | 11 | — |
| 5 | It opens a bounded local receiver, summarizes volume and field types, flags likely sensitive data, and writes a JSON report. | 20 | — |
| 9 | You need Rust 1.75+ to run the CLI. | 8 | F-2-1 |
| 9 | Clone the public source repository first. | 6 | — |
| 15 | Uses the sample embedded in the binary and prints the unique report directory. | 11 | — |
| 18 | Point a temporary HTTP drain to `http://127.0.0.1:8787/`. | 7 | — |
| 21 | The listener binds only to `127.0.0.1`. | 6 | — |
| 21 | It aggregates accepted events as they arrive, then drops their parsed values and bodies. | 14 | — |
| 21 | Pass `--save-sample sample.ndjson` only when you intentionally want accepted bodies written to disk. | 12 | — |
| 21 | `--output` and `--save-sample` must name different files; Drain Check refuses a collision before opening the listener. | 16 | F-2-1 |
| 23 | Malformed or incomplete requests return HTTP 400 without ending the sample window. | 11 | — |
| 23 | The default rolling limit accepts 20 requests per second, then returns HTTP 429 with `Retry-After: 1`. | 14 | — |
| 23 | Change it with `--rate-limit`. | 3 | — |
| 25 | You can also inspect an existing newline-delimited JSON file. | 9 | — |
| 31 | `inspect --sample-seconds` must be at least 1. | 5 | F-2-1 |
| 31 | `--json` prints the report to standard output for scripts. | 8 | F-2-1 |
| 31 | Add `--sensitive-field session_key` for a team-specific field name. | 7 | — |
| 31 | Add `--ignore-field '$.request_id'` after reviewing a false positive. | 7 | — |
| 31 | A trailing `*` ignores a path prefix. | 6 | — |
| 31 | `cargo run -- --help` lists all commands and options. | 6 | F-2-1 |
| 35 | Event count, average body size, and events per second. | 8 | — |
| 36 | Field paths, observed types, and how many events contained each path. | 10 | — |
| 37 | Conservative risk findings for field names, token-shaped values, and email-shaped values. | 10 | — |
| 38 | Retention estimates for 7 and 30 days. | 7 | — |
| 39 | A recommendation to review the sample before forwarding. | 8 | — |
| 41 | Risk findings are prompts for review. | 6 | — |
| 41 | They do not prove a value is sensitive, and they cannot find every secret. | 14 | F-2-1 |
| 41 | The bundled sample reports 3 events, 17 field paths, 558.1 KiB for 7 days, and 2.3 MiB for 30 days. | 20 | — |
| 45 | Run the separate `forwarding` command after reviewing the report. | 8 | — |
| 51 | The command accepts an HTTP(S) URL and safely encodes it in the generated configuration. | 14 | — |
| 55 | Open the web demo for the bundled sample report. | 9 | — |
| 55 | The command-line equivalent is `cargo run -- demo --json`. | 5 | — |
| 55 | See `.factory/demo.md` for demo storage and reset details. | 7 | — |
| 70 | The static site build lands in `dist/site` with its deployment configuration at that root. | 14 | — |
| 70 | `cargo package` prepares the CLI crate for publishing; this repository does not publish it. | 12 | — |
| 74 | The website requests only same-origin files and writes no browser storage. | 10 | — |
| 74 | The CLI receives data on its loopback listener. | 8 | — |
| 74 | Read the deployed Privacy page and Terms. | 7 | — |
| 74 | Licensed under MIT. | 3 | — |

README headings — **Install and run**, **What the report contains**, **Generate a forwarding configuration**, **Demo**, **Develop, test, and build**, and **Privacy and license** — name their sections. Its command blocks are runnable commands rather than prose sentences. No marketing adjective, mood heading, or ambiguous product synonym was found.

## Demo, CLI sandbox, and privacy checks

- One click from the landing page reaches `/?demo=1`; no account or setup is required. The first demo screen already contains the populated three-event report, event rate, 17 field paths, three findings across two fields, detector actions, retention estimates, and the real forwarding command.
- The persistent banner says exactly “Demo — sample data, nothing is saved.” Reset demo is keyboard-operable. From an injected `real:drain-check` local-storage sentinel and a `demo:drain-check` test key, Reset removes only the demo key; Start for real returns to `/` and does not read or write a real-data key.
- In a fresh browser context, landing-to-demo requested only the document, same-origin JS, CSS, and `drain-console.webp`. Cookies, localStorage, sessionStorage, IndexedDB, Cache Storage, and service-worker registrations were empty. No third-party request or console/page error occurred.
- I built the CLI and ran `drain-check demo --json` from a new temporary working directory. It wrote `/tmp/drain-check-demo-*/report.json`, not the repository, and printed the bundled 3-event / 17-field / 3-finding report with `bodies_saved: false`.
- The product does not make an offline claim, so an offline capability check is not applicable. It has no AI feature, provider key, hosted account, or sync boundary. The brief asks for local inspection and forwarding configuration; the shipped JSON report and separate forwarding command meet the implied export/configuration leverage. An AI step would not improve this bounded local pre-flight job and would add a privacy boundary.

## Claims verification

I made a fresh local clone of this checkout, ran `npm ci`, and executed every literal command in `.factory/claims.json` separately and in file order. All 13 passed. A full replay then passed **31/31** tests.

| Claim | Result | Observed evidence |
| --- | --- | --- |
| `sample-demo` | PASS | One click opens the populated report, exact metrics, banner, and empty storage. |
| `local-only` | PASS | Request log is same-origin only; storage stays empty; Rust test proves loopback bind. |
| `discard-default` | PASS | Accepted unique values are absent from the report. |
| `contract-report` | PASS | Fixture asserts count, size, fields/types, findings, and both retention values. |
| `forwarding-config` | PASS | HTTP(S) validation and safe configuration rendering are observed. |
| `source-checkout` | PASS | A fresh public GitHub clone runs Cargo help. |
| `false-positive-controls` | PASS | Custom sensitive patterns and exact/prefix suppression work. |
| `rate-limit` | PASS | The configured limit returns 429 and `Retry-After: 1`. |
| `request-recovery` | PASS | Bad/incomplete requests return 400 while prior events remain. |
| `explicit-save` | PASS | Accepted bodies are written only after explicit opt-in. |
| `interrupt-report` | PASS | SIGINT writes a valid partial report. |
| `portable-demo` | PASS | The embedded sample runs outside the repository in a unique temporary directory. |
| `mit-license` | PASS | The shipped LICENSE carries the MIT grant. |

No declared claim test failed. F-2-1 is an inventory defect: it concerns statements that have no declared claim entry, not a failure of the listed 13.

## History, structure, and visual checks

I read `review-1.md`, `polish-1.md`, all `verification-*.md` files, and the previous handoff. Every earlier finding is actually fixed in both live behavior and source:

| Earlier finding | Confirmation |
| --- | --- |
| F-1-1 unavailable install | The landing and README provide the linked public source checkout. The GitHub link returns 200; fresh source-checkout claim passes. |
| F-1-2 forwarding template mismatch | The report now says forwarding recommendation; the separately headed forwarding command is real and tested. |
| F-1-3 risk-count mismatch | The demo visibly renders three detector rows and says “3 findings across 2 fields.” |
| F-1-4 broad privacy claims | The prior untestable vendor/search/telemetry statements are gone. Narrow privacy behavior has a declared, passing claim. |
| F-1-5 mood eyebrows | The old “PRE-FLIGHT” and “SHORT PATH” text is gone; current section labels are direct. |
| F-1-6 inconsistent output language | “report,” “sensitive data,” and “forwarding configuration” are used consistently; “contract” and “with intent” are absent. |
| F-1-7 unclear terms/404 headings | `/terms` has “Terms for using Drain Check”; `/missing` has “Page not found.” |

- Live `/`, `/?demo=1`, `/demo`, `/privacy`, and `/terms` return 200. `/missing` returns a designed 404. `robots.txt`, `sitemap.xml`, all landing internal links, and the external public-source link return 200.
- Every checked route has its own title in the required pattern, one H1, one main landmark, a meta description, canonical URL, Open Graph image, Twitter card, and favicon. On 390 px each route has `scrollWidth = innerWidth = 390`.
- Keyboard navigation to demo and browser Back both move focus to the destination H1. The shared header, skip link, footer, Privacy, and Terms links are present on all routes. The 404 has a working home route.
- The deployed CSP restricts sources to self and includes response-header `frame-ancestors 'none'`; HSTS, `nosniff`, and strict-origin referrer policy are present. The pixel/demoscene terminal panel, static scanlines, local image, squared controls, and no-gradient treatment match the recorded visual thesis and are distinct from a generic SaaS template.
- The full local accessibility suite passed at desktop and 390 px with zero serious/critical Axe violations. It also exercises focus, touch targets, 200% text, and reduced motion. Normal live pages showed no console errors.

## What would make this perfect

1. Close F-2-1 by giving every remaining concrete README promise an explicit claims entry and tagged, observable test; remove or rewrite the untestable detector-limit wording.
2. Close F-2-2 by renaming “Start for real” to “View local setup.”
3. Rerun the clean-clone claim commands and the 390 px cold demo flow after those copy/contract edits. At that point, the product would be PASS-adjacent with no identified remaining work.
