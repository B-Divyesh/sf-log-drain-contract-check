# Adversarial first-read review 5 — Drain Check

**Verdict: PASS.** This round found no blocking, major, or minor findings. The
live first screen is clear at phone and desktop widths, the bundled sample is
isolated and immediately useful, every declared claim passed from a clean
clone, and all earlier findings remain fixed. A future change must retain
these checks; PASS means there is nothing left to do in this reviewed build.

## Cold first read

I opened <https://log-drain-contract-check.sociobot.in/> in separate fresh
Playwright contexts at 390 × 844 and 1440 × 900, without scrolling.

- **What it does:** checks a bounded local log-drain sample for volume, field
  paths, and sensitive data before the drain is forwarded.
- **For whom:** platform teams before they enable a log drain.
- **What to click first:** **Try it with sample data**. The adjacent copy says,
  “Opens the bundled report. Writes no browser data.”

All three answers are present in the first viewport at both sizes. The phone
view places the 44 px primary action above the illustration; there is no
ambiguous or hidden first action. The first-read check passes.

## Findings

None. No `F-5-k` identifier is assigned because there are zero findings.

## Copy audit

Word counts are whitespace-delimited. Code commands and URLs count as one word
where they form reader-facing instructions. The audit includes prose sentences
and sentence-like recorded output; headings, navigation, and controls are
checked immediately afterwards. No entry exceeds 22 words. No jargon needing
translation, banned marketing adjective, inconsistent term, empty slogan, or
unresultful button remains.

### Landing page

| Location | Sentence | Words | Result |
| --- | --- | ---: | --- |
| H1 | Check a log drain before forwarding | 6 | Pass |
| First screen | For platform teams checking volume, field types, and sensitive data before enabling a log drain. | 15 | Pass |
| Action note | Opens the bundled report. | 4 | Pass |
| Action note | Writes no browser data. | 4 | Pass |
| Fact | The receiver binds to `127.0.0.1`. | 5 | Pass |
| Fact | Accepted bodies are discarded by default. | 6 | Pass |
| Fact | Free under the MIT License. | 5 | Pass |
| Art caption | The receiver stays local. | 4 | Pass |
| Recording help | A text recording of the bundled CLI demo. | 8 | Pass |
| Recording help | Use the replay button to play it again. | 8 | Pass |
| Recording | Reviewed 3 events in 600s. | 5 | Pass |
| Recording | 17 field paths. | 3 | Pass |
| Recording | 3 findings in 2 field paths. | 6 | Pass |
| Recording | Report: `/tmp/drain-check-demo-[unique]/report.json` | 2 | Pass |
| Recording | Send POST requests to this endpoint after report review. | 9 | Pass |
| Step 1 | Run the receiver locally. | 4 | Pass |
| Step 1 | Run one bounded window. | 4 | Pass |
| Step 2 | Review the report. | 3 | Pass |
| Step 2 | Check field paths and likely sensitive data. | 7 | Pass |
| Step 3 | Generate a forwarding configuration. | 4 | Pass |
| Step 3 | Review the generated configuration. | 4 | Pass |
| Data handling | The receiver discards accepted bodies after aggregation by default. | 9 | Pass |
| Data handling | Saving accepted bodies requires `--save-sample`. | 5 | Pass |
| Local setup | Clone the public source repository on GitHub, then run the receiver. | 11 | Pass |
| Local setup | Point your temporary HTTP drain to `http://127.0.0.1:8787/`. | 7 | Pass |
| Local setup | Use `--ignore-field '$.request_id'` to suppress a reviewed false positive. | 9 | Pass |
| Footer | Drain Check samples a log drain before you forward it. | 10 | Pass |

The section labels — **Local 10-minute sample**, **How it works**, **Review a
drain in three steps**, **What Drain Check does not retain**, **Run it locally**,
and **Start a bounded receiver** — name their content directly. The controls
are **Try it with sample data**, **Replay recording**, **Reset demo**, **View
local setup**, and **Return home**. Each either names the result or is a
standard page-navigation link. Terminology consistently uses *receiver*,
*field paths*, *sample*, *report*, *finding*, and *forwarding configuration*.

### README

| Location | Sentence | Words | Result |
| --- | --- | ---: | --- |
| Title | Check a log drain before forwarding it. | 7 | Pass |
| Introduction | Drain Check is for small platform teams adding a managed drain. | 11 | Pass |
| Introduction | It opens a bounded local receiver, summarizes volume and field types, flags likely sensitive data, and writes a JSON report. | 20 | Pass |
| Install | Clone the public source repository first: | 6 | Pass |
| Demo command comment | Uses the sample embedded in the binary and prints the unique report directory. | 13 | Pass |
| Listen command comment | Point a temporary HTTP drain to `http://127.0.0.1:8787/`. | 7 | Pass |
| Receiver behavior | The receiver binds only to `127.0.0.1`. | 6 | Pass |
| Receiver behavior | It aggregates accepted events as they arrive, then drops their parsed values and bodies. | 14 | Pass |
| Receiver behavior | Pass `--save-sample sample.ndjson` only when you intentionally want accepted bodies written to disk. | 13 | Pass |
| Receiver behavior | `--output` and `--save-sample` must name different files; Drain Check refuses a collision before starting the receiver. | 16 | Pass |
| Request handling | Malformed or incomplete requests return HTTP 400 without ending the sample window. | 12 | Pass |
| Request handling | The default rolling limit accepts 20 requests per second, then returns HTTP 429 with `Retry-After: 1`. | 16 | Pass |
| Request handling | Change it with `--rate-limit`. | 4 | Pass |
| Inspect | You can also inspect an existing newline-delimited JSON file: | 9 | Pass |
| Inspect options | `inspect --sample-seconds` must be at least 1. | 7 | Pass |
| Inspect options | `--json` prints the report to standard output for scripts. | 9 | Pass |
| Inspect options | Add `--sensitive-field session_key` for a team-specific field name. | 8 | Pass |
| Inspect options | Add `--ignore-field '$.request_id'` after reviewing a false positive. | 8 | Pass |
| Inspect options | Keys with punctuation use bracket-quoted paths, such as `$['http.method']`. | 9 | Pass |
| Inspect options | A trailing `*` ignores a path prefix. | 7 | Pass |
| Inspect options | `cargo run -- --help` lists all commands and options. | 9 | Pass |
| Report contents | Event count, average received event size, and events per second. | 10 | Pass |
| Report contents | NDJSON line delimiters are excluded. | 5 | Pass |
| Report contents | Field paths, observed types, and how many events contained each path. | 11 | Pass |
| Report contents | Conservative risk findings for field names, token-shaped values, and email-shaped values. | 11 | Pass |
| Report contents | Retention estimates for 7 and 30 days. | 7 | Pass |
| Report contents | A recommendation to review the sample before forwarding. | 8 | Pass |
| Report contents | Review each finding before forwarding. | 5 | Pass |
| Report contents | The bundled sample reports 3 events, 17 field paths, 558.1 KiB for 7 days, and 2.3 MiB for 30 days. | 20 | Pass |
| Forwarding | Run the separate `forwarding` command after reviewing the report: | 9 | Pass |
| Forwarding | The command accepts an HTTP(S) URL and safely encodes it in the generated configuration. | 14 | Pass |
| Demo | Open the web demo for the bundled sample report. | 9 | Pass |
| Demo | The command-line equivalent is `cargo run -- demo --json`. | 9 | Pass |
| Demo | See `.factory/demo.md` for demo storage and reset details. | 8 | Pass |
| Build | The static site build writes deployable files and Azure Static Web Apps configuration to `dist/site`. | 15 | Pass |
| Deploy | Build the site first, then deploy the complete `dist/site` directory with the factory static deployment command: | 16 | Pass |
| Deploy | For Azure Static Web Apps, keep `staticwebapp.config.json` at the deployment root. | 10 | Pass |
| Deploy | On other hosts, recreate its rewrites, 404 response, headers, and cache rules in that host’s configuration. | 16 | Pass |
| Privacy | The website requests only same-origin files and writes no browser storage. | 11 | Pass |
| Privacy | The CLI receives data on its loopback receiver. | 8 | Pass |
| Privacy | Read the deployed Privacy page and Terms. | 7 | Pass |
| Privacy | Licensed under MIT. | 3 | Pass |

README headings are direct: **Install and run**, **What the report contains**,
**Generate a forwarding configuration**, **Demo**, **Develop, test, and
build**, **Deploy the static site**, and **Privacy and license**.

## Demo, privacy, and CLI sandbox

- The hero link reaches `/?demo=1` in one click. At 390 px its first screen
  already shows the persistent **Demo — sample data, nothing is saved** banner,
  reset control, precise exit link, sample-report H1, and populated 3-event,
  17-field-path, three-finding metrics.
- **Reset demo** removes only `demo:drain-check`. A deliberately seeded
  `real:review5=preserve-me` key survived reset, exit, and route change. The
  page did not create the demo key in the first place.
- The whole landing → demo → reset → exit flow made four requests, all to
  `log-drain-contract-check.sociobot.in`. Cookies, session and local storage
  (other than the seeded control), IndexedDB, Cache Storage, service-worker
  registrations, and OPFS entries were empty. No offline promise is made.
- From an unrelated temporary working directory, `cargo run --manifest-path
  <clean-clone>/Cargo.toml -- demo --json` returned the realistic embedded
  three-event report (17 paths, three findings, `bodies_saved: false`) and
  printed a unique `/tmp/drain-check-demo-*/report.json` location. It did not
  use the consumer directory for the report.

## Claims verification

I made a new shallow clone of the public GitHub repository, ran `npm ci`, and
executed all 18 literal `test` commands in `.factory/claims.json` from that
clone. Every command passed.

| Claim ID | Result |
| --- | --- |
| `sample-demo` | PASS |
| `local-only` | PASS |
| `discard-default` | PASS |
| `contract-report` | PASS |
| `forwarding-config` | PASS |
| `source-checkout` | PASS |
| `false-positive-controls` | PASS |
| `rate-limit` | PASS |
| `request-recovery` | PASS |
| `explicit-save` | PASS |
| `separate-output-paths` | PASS |
| `minimum-sample-duration` | PASS |
| `json-stdout` | PASS |
| `complete-help` | PASS |
| `interrupt-report` | PASS |
| `portable-demo` | PASS |
| `site-build-output` | PASS |
| `mit-license` | PASS |

Every live landing/README behavior claim maps to one of those narrowly worded
claims: the demo metrics and isolation, same-origin/no-storage web behavior,
loopback binding, discard default, report contents, forwarding safety, source
checkout, false-positive controls, rate/recovery/save validations, CLI help,
portable demo, build output, and MIT license. No unlisted claim remains.

The complete clean-clone suite also passed: `npm test` (40/40), typecheck,
production build, Rust all-target tests (26/26), Rustfmt, and Clippy with
warnings denied. The build emits 11.33 kB JavaScript (4.25 kB gzip) and 6.78
kB CSS (2.21 kB gzip).

## Earlier-finding closure

I read reviews 1–4, polish reports 1–4, and the previous handoff, then
confirmed each item on the live site and in the current source.

| Earlier finding | Current confirmation |
| --- | --- |
| F-1-1 | Fixed: live setup and README use the public source checkout, and `source-checkout` passes. |
| F-1-2 | Fixed: the report gives a recommendation; a separate command shows a real forwarding configuration. |
| F-1-3 | Fixed: all three findings are displayed and counted as three findings in two field paths. |
| F-1-4 | Fixed: broad privacy slogans remain removed; the remaining narrow statements have claim tests. |
| F-1-5 | Fixed: direct section names remain in place. |
| F-1-6 | Fixed: “contract”/“intent” wording remains absent; output is consistently a report. |
| F-1-7 | Fixed: live Terms and 404 H1s are “Terms for using Drain Check” and “Page not found.” |
| F-2-1 | Fixed: the previously unlisted CLI behavior statements have individual claims/tests, and the untestable detector slogan is absent. |
| F-2-2 / F-4-1 | Fixed: the demo exit is **View local setup**, routes to `/`, and focuses the landing H1. |
| F-3-1 | Fixed: the build-output claim exists, passes, and deployment text names Azure separately from other hosts. |
| F-3-2 | Fixed: no unsupported package/release-status promise is present. |
| F-4-2 | Fixed: README does not promise that arbitrary static hosts interpret Azure configuration unchanged. |
| F-4-3 | Fixed: `--platform` rejects control characters; the forwarding claim test covers the injection attempt. |
| F-4-4 | Fixed: the local-only test inspects Web Storage, IndexedDB, Cache Storage, service workers, and OPFS. |
| F-4-5 | Fixed: all visitor-facing report counts say **field paths** and prose says **receiver**. |

## Structure, accessibility, and visual identity

- `/`, `/?demo=1`, `/demo`, `/privacy`, and `/terms` return 200. `/missing`
  returns a designed 404 with a working home link and the correct 404 status.
  All crawled internal links, assets, and the public source link returned their
  expected response.
- Every reviewed route has its own plain-language title, one H1, one main
  landmark, `lang="en"`, description, canonical, OG/Twitter metadata, favicon,
  shared header/footer, skip link, Privacy, and Terms. `robots.txt` and the
  sitemap are present.
- Route navigation and browser Back move focus to the new H1. Axe found zero
  violations on all six routes at 390 px and desktop. Normal-route loads had
  no browser console or page errors.
- Live headers include HSTS, `nosniff`, strict-origin referrer policy, and a
  response-header CSP with `frame-ancestors 'none'`.
- The scanlined pixel/demoscene receiver, original local illustration, dark
  instrument palette, squared controls, and terminal recording match
  `.factory/design.md`. This is not a generic SaaS template.

## Missed leverage

No feature gap is implied by the brief. The real job is covered by local
receipt, bounded sampling, a JSON report, sensitive-data review, retention
estimates, false-positive controls, and a separate forwarding configuration.
Importing NDJSON already exists. Cloud sync or an AI gateway would contradict
the local privacy boundary and does not improve this deterministic task. No AI
feature or provider key is present.

## What would make this perfect

Keep the one-click demo and its complete storage test intact, keep claims
matched to reader-facing promises, and rerun this full cold mobile/desktop and
clean-clone review after any copy, routing, or deployment change. There is no
additional product change required for this reviewed revision.
