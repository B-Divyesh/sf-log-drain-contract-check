# Adversarial first-read review 3 — Drain Check

**Verdict: FAIL.** The deployed product is clear, tryable, private in its web
demo, and its declared product claims pass. Two README build/release promises
are not in the claims contract. The review standard permits PASS only with no
findings.

## Cold first read

I opened `https://log-drain-contract-check.sociobot.in/` in new browser
contexts at 390 × 844 and 1440 × 900, without scrolling.

- **What it does:** it samples a log drain before forwarding and reports its
  volume, fields, and possible sensitive data.
- **For whom:** platform teams checking a drain before enabling it.
- **What to click first:** **Try it with sample data**. The note says it opens
  the bundled report and writes no browser data.

All three answers were available on the first screen at both sizes. On the
390 px view, the primary action occupied y=423–470 in an 844 px viewport. This
first-read check passes.

## Findings

### F-3-1 — MINOR — README build-output promise is absent from the claims contract

- **Location / exact quote:** README line 70: “The static site build lands in
  `dist/site` with its deployment configuration at that root.”
- **Why this fails the contract:** this is a concrete result that a developer
  can rely on, yet no `.factory/claims.json` entry names it. `npm run build`
  happened to pass during this review, but an undeclared quality command is not
  the required one-claim/one-tagged-test contract.
- **Concrete fix:** add a `site-build-output` claim and
  `@claim:site-build-output` test that runs `npm run build:site` from a clean
  checkout and asserts `dist/site/index.html` and
  `dist/site/staticwebapp.config.json`; or remove this promise.

### F-3-2 — MINOR — README package/release-status promise is absent from the claims contract

- **Location / exact quote:** README line 70: “`cargo package` prepares the
  CLI crate for publishing; this repository does not publish it.”
- **Why this fails the contract:** this is two concrete packaging/release
  assertions with no claims entry. The present quality gate runs `cargo
  package`, but it neither declares nor proves the public-release-status
  assertion. A visitor can use the statement to decide whether the documented
  installation path is available.
- **Concrete fix:** delete the sentence because it is not needed for first use.
  If it is retained, split it into narrow declared claims: test package creation
  from a clean checkout, and define and test the release-status assertion
  against the intended registry/source of truth.

## Copy audit

Counts use whitespace-delimited words; inline code, a URL, and a command each
count as one word. Command-only lines and navigation labels are not sentences.
No sentence exceeds 22 words. No banned marketing adjective, undefined product
synonym, mood heading, or non-result-naming button was found. The two flags are
the unlisted technical claims above.

### Landing page

| Location | Sentence / text | Words | Flag |
| --- | --- | ---: | --- |
| H1 | Inspect a log drain before forwarding | 6 | — |
| Hero lede | For platform teams checking volume, field types, and sensitive data before enabling a log drain. | 15 | — |
| Hero note | Opens the bundled report. | 4 | — |
| Hero note | Writes no browser data. | 4 | — |
| Hero fact | The receiver binds to `127.0.0.1`. | 5 | — |
| Hero fact | Accepted bodies are discarded by default. | 6 | — |
| Hero fact | Free under the MIT License. | 5 | — |
| Recording help | A text recording of the bundled CLI demo. | 8 | — |
| Recording help | Use the replay button to play it again. | 8 | — |
| Recording | Reviewed 3 events in 600s. | 5 | — |
| Recording | 17 fields. | 2 | — |
| Recording | 3 findings across 2 fields. | 6 | — |
| Recording | Report: `/tmp/drain-check-demo-[unique]/report.json` | 2 | — |
| Recording | Send POST requests to this endpoint after report review. | 9 | — |
| Step 1 | Listen locally. | 2 | — |
| Step 1 | Run one bounded window. | 4 | — |
| Step 2 | Review the report. | 3 | — |
| Step 2 | Check fields and likely sensitive data. | 6 | — |
| Step 3 | Generate a forwarding configuration. | 4 | — |
| Step 3 | Review the generated configuration. | 4 | — |
| Data handling | The receiver discards accepted bodies after aggregation by default. | 9 | — |
| Data handling | Saving accepted bodies requires `--save-sample`. | 5 | — |
| Local setup | Clone the public source repository on GitHub, then run the receiver. | 11 | — |
| Local setup | Point your temporary HTTP drain to `http://127.0.0.1:8787/`. | 8 | — |
| Local setup | Use `--ignore-field '$.request_id'` to suppress a reviewed false positive. | 9 | — |
| Footer | Drain Check samples a log drain before you forward it. | 10 | — |

Direct labels and headings are **Local 10-minute sample**, **How it works**,
**Review a drain in three steps**, **What Drain Check does not retain**, and
**Start a bounded receiver**. Controls are **Try it with sample data**,
**Replay recording**, **Reset demo**, **View local setup**, and **Read local
setup**; each names an action or destination. Terminology remains consistent:
*drain*, *sample*, *report*, *body*, *finding*, *sensitive data*, and
*forwarding configuration*.

### README

| Line | Sentence / text | Words | Flag |
| --- | --- | ---: | --- |
| 3 | Inspect a log drain before forwarding it. | 7 | — |
| 5 | Drain Check is for small platform teams adding a managed drain. | 11 | — |
| 5 | It opens a bounded local receiver, summarizes volume and field types, flags likely sensitive data, and writes a JSON report. | 20 | — |
| 9 | Clone the public source repository first: | 6 | — |
| 15 | Uses the sample embedded in the binary and prints the unique report directory. | 11 | — |
| 18 | Point a temporary HTTP drain to `http://127.0.0.1:8787/`. | 7 | — |
| 21 | The listener binds only to `127.0.0.1`. | 6 | — |
| 21 | It aggregates accepted events as they arrive, then drops their parsed values and bodies. | 14 | — |
| 21 | Pass `--save-sample sample.ndjson` only when you intentionally want accepted bodies written to disk. | 12 | — |
| 21 | `--output` and `--save-sample` must name different files; Drain Check refuses a collision before opening the listener. | 16 | — |
| 23 | Malformed or incomplete requests return HTTP 400 without ending the sample window. | 11 | — |
| 23 | The default rolling limit accepts 20 requests per second, then returns HTTP 429 with `Retry-After: 1`. | 16 | — |
| 23 | Change it with `--rate-limit`. | 4 | — |
| 25 | You can also inspect an existing newline-delimited JSON file: | 9 | — |
| 31 | `inspect --sample-seconds` must be at least 1. | 5 | — |
| 31 | `--json` prints the report to standard output for scripts. | 9 | — |
| 31 | Add `--sensitive-field session_key` for a team-specific field name. | 8 | — |
| 31 | Add `--ignore-field '$.request_id'` after reviewing a false positive. | 8 | — |
| 31 | A trailing `*` ignores a path prefix. | 6 | — |
| 31 | `cargo run -- --help` lists all commands and options. | 7 | — |
| 35 | Event count, average body size, and events per second. | 9 | — |
| 36 | Field paths, observed types, and how many events contained each path. | 11 | — |
| 37 | Conservative risk findings for field names, token-shaped values, and email-shaped values. | 11 | — |
| 38 | Retention estimates for 7 and 30 days. | 7 | — |
| 39 | A recommendation to review the sample before forwarding. | 8 | — |
| 41 | Review each finding before forwarding. | 5 | — |
| 41 | The bundled sample reports 3 events, 17 field paths, 558.1 KiB for 7 days, and 2.3 MiB for 30 days. | 20 | — |
| 45 | Run the separate `forwarding` command after reviewing the report: | 8 | — |
| 51 | The command accepts an HTTP(S) URL and safely encodes it in the generated configuration. | 14 | — |
| 55 | Open the web demo for the bundled sample report. | 9 | — |
| 55 | The command-line equivalent is `cargo run -- demo --json`. | 5 | — |
| 55 | See `.factory/demo.md` for demo storage and reset details. | 7 | — |
| 70 | The static site build lands in `dist/site` with its deployment configuration at that root. | 14 | F-3-1 |
| 70 | `cargo package` prepares the CLI crate for publishing; this repository does not publish it. | 12 | F-3-2 |
| 74 | The website requests only same-origin files and writes no browser storage. | 10 | — |
| 74 | The CLI receives data on its loopback listener. | 8 | — |
| 74 | Read the deployed Privacy page and Terms. | 6 | — |
| 74 | Licensed under MIT. | 3 | — |

The README headings name their sections. The code blocks are copy-pasteable
commands, not prose sentences. The only copy flags are the two unlisted claims.

## Demo, sandbox, and privacy verification

- One click from the live landing page opened `/?demo=1`. Its first screen was
  already a populated three-event report: 0.005 events/sec, 17 field paths,
  three findings across two fields, detector actions, 558.1 KiB/7 days, 2.3
  MiB/30 days, and the real forwarding command.
- The persistent banner read “Demo — sample data, nothing is saved”, with
  **Reset demo** and **View local setup**. In a fresh context, both storage
  areas and cookies were empty. After injecting `real:review-sentinel=keep`
  and `demo:drain-check=reset-me`, Reset removed only the demo key. Leaving the
  demo returned to `/` and focused the landing H1.
- The live request log contained only the document, same-origin JavaScript,
  CSS, and the same-origin hero WebP. It made no third-party request and
  emitted no normal-route console or page error. The product makes no offline
  claim.
- From a new temporary working directory, the compiled `drain-check demo
  --json` used its embedded sample, printed a unique `/tmp/drain-check-demo-*`
  report path, returned the advertised metrics, and reported `bodies_saved:
  false`.

## Claims verification

I cloned this checkout into a fresh temporary directory, ran `npm ci`, then
ran every literal command in `.factory/claims.json` separately. All 17 passed.
`npm test` then passed 35/35 tests in that clone; `npm run typecheck` and
`npm run build` also completed successfully, with `dist/site/` present.

| Claim id | Result |
| --- | --- |
| sample-demo | PASS |
| local-only | PASS |
| discard-default | PASS |
| contract-report | PASS |
| forwarding-config | PASS |
| source-checkout | PASS |
| false-positive-controls | PASS |
| rate-limit | PASS |
| request-recovery | PASS |
| explicit-save | PASS |
| separate-output-paths | PASS |
| minimum-sample-duration | PASS |
| json-stdout | PASS |
| complete-help | PASS |
| interrupt-report | PASS |
| portable-demo | PASS |
| mit-license | PASS |

## Earlier finding closure

I read `review-1.md`, `review-2.md`, `polish-1.md`, `polish-2.md`, and the
prior handoff, then checked each earlier finding against the live site and
current code.

| Earlier finding | Current confirmation |
| --- | --- |
| F-1-1 | The unavailable registry install is gone; the linked public source checkout works and its claim passes. |
| F-1-2 | The report names a recommendation; a separate, real forwarding command validates and renders the configuration. |
| F-1-3 | The live report visibly renders all three findings and labels them across two fields. |
| F-1-4 | Broad vendor/search/telemetry wording is absent; the remaining privacy behavior is narrow and tested. |
| F-1-5 | The former mood-only labels are replaced with direct section names. |
| F-1-6 | The live product consistently uses report, sensitive data, and forwarding configuration. |
| F-1-7 | `/terms` says “Terms for using Drain Check”; the designed HTTP 404 says “Page not found.” |
| F-2-1 | The prior CLI behavior promises now have their own declared tests; F-3-1 and F-3-2 are separate build/release-doc gaps. |
| F-2-2 | The demo exit control is now “View local setup” and routes to that destination. |

None of the earlier IDs is unfixed, half-fixed, or regressed.

## Structure, accessibility, and leverage

- `/`, `/?demo=1`, `/demo`, `/privacy`, and `/terms` returned 200; `/missing`
  returned a designed 404. Each app route had a route-specific title, one H1,
  one main landmark, description, canonical URL, OG image, Twitter metadata,
  favicon, and the shared header/footer/legal links. `robots.txt` and
  `sitemap.xml` were present.
- The complete live-link crawl returned 200 for every internal route and the
  public GitHub source link. Keyboard navigation to Demo and browser Back moved
  focus to the destination H1 and updated the polite route announcement.
- Live Axe checks at 390 px and 1440 px found zero serious or critical issues
  on all six routes. No route overflowed at 390 px. The expected browser error
  from directly loading the deliberate 404 was excluded from normal-route
  console checks.
- The locally hosted pixel/demoscene receiver illustration, scanline panel,
  squared controls, and terminal recording match `.factory/design.md` and are
  distinct from a generic SaaS template. No remote font, CDN script, tracking
  request, or provider key was found.
- The brief implies deterministic local inspection, report export, and a
  forwarding configuration. Those are present. An AI feature would add a data
  boundary without improving this bounded local pre-flight task; no AI leverage
  finding applies.

## What would make this perfect

Either declare and test the two remaining README build/release assertions, or
remove the release-status sentence and make build output a tested claim. Rerun
the fresh-clone claim loop afterwards. With no unlisted claim left, the next
round can be PASS.
