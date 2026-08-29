# Adversarial first-read review 4 — Drain Check

**Verdict: FAIL.** The first screen, sample demo, live privacy behavior, declared
claim commands, accessibility checks, and current factory deployment work. One
earlier plain-words finding has regressed and is blocking under the round-4
history rule. Four additional findings remain. PASS requires zero findings and
no partly tested claim.

## Cold first read

I opened <https://log-drain-contract-check.sociobot.in/> cold in separate fresh
Playwright contexts at 390 × 844 and 1440 × 900. I did not scroll before
answering:

- **What it does:** samples a log drain locally, then reports its volume,
  fields, and possible sensitive data before forwarding.
- **For whom:** platform teams checking a drain before enabling it.
- **What to click first:** **Try it with sample data**. The adjacent text says,
  “Opens the bundled report. Writes no browser data.”

All three answers are available above the fold at both widths. On the 390 px
view, the primary action occupies y=396–443 in an 844 px viewport. The exact
first-screen headline is “Inspect a log drain before forwarding,” followed by
“For platform teams checking volume, field types, and sensitive data before
enabling a log drain.” This part is not blocking.

## Findings

### F-4-1 / F-2-2 — BLOCKING — the previously repaired demo-exit label regressed

- **Location / exact quote:** live `/?demo=1` banner and
  `site/src/main.ts`: **“Start for real”**.
- **History evidence:** review 2 identified this as F-2-2 because the action
  only returns to the landing page. Polish 2 and polish 3 changed it to “View
  local setup” and recorded the finding as closed. Commit `9f931c0` restored
  the old text, and both the current browser test and live deployment now
  require it again.
- **Why this blocks:** the link does not start a receiver or a real sample. It
  routes to `/`, where the visitor still has to clone the repository and run a
  command. The label promises a result that the click does not produce. This
  is an exact regression of an earlier finding, which this review must reopen
  as blocking with the same ID.
- **Concrete fix:** restore **View local setup** or **Leave demo and view local
  setup** in the banner, `.factory/demo.md`, and the tagged browser test. Keep
  the current route, H1 focus, and demo-data isolation behavior.

### F-4-2 — MAJOR — the README makes an unlisted, overbroad static-host deployment promise

- **Location / exact quote:** README lines 83–85: “For another static host,
  upload the contents of `dist/site` unchanged. Keep
  `staticwebapp.config.json` with the output so the documented routes, 404
  page, security headers, and immutable asset caching are deployed together.”
- **Evidence:** `.factory/claims.json` has no claim for generic static-host
  deployment. `site-build-output` only checks that two files exist. Serving
  the built directory unchanged with a plain static server returned 404 for
  `/privacy` and `/missing`, and supplied none of the documented CSP or cache
  headers. `staticwebapp.config.json` is provider-specific; an arbitrary host
  does not interpret it.
- **Why this matters:** a developer following the alternative deployment
  instruction can ship broken deep links and omit the security headers while
  believing the README says those behaviors travel with the directory.
- **Concrete fix:** replace the quoted copy with: “For Azure Static Web Apps,
  keep `staticwebapp.config.json` at the deployment root. On other hosts,
  recreate its rewrites, 404 response, headers, and cache rules in that host’s
  configuration.” Do not make a generic-host claim unless a named host and an
  end-to-end deployment test are added to `.factory/claims.json`.

### F-4-3 — MINOR — `--platform` can inject an uncommented configuration line

- **Location / exact behavior:** `src/main.rs`, `forwarding_config()`. Running
  `drain-check forwarding --url https://receiver.example/logs --platform
  $'generic-http\nurl = "https://attacker.invalid"'` prints the injected URL
  assignment before the validated destination.
- **Why this matters:** `--platform` is an exposed CLI input in the real
  forwarding-config workflow. An operator or wrapper that passes untrusted or
  malformed label text can produce an ambiguous configuration even though URL
  input itself is validated. The current handoff and verification 9 also
  record this defect; it is not merely theoretical.
- **Concrete fix:** reject `\r`, `\n`, and other control characters in
  `--platform`, or prefix every rendered platform line with `# `. Add a CLI
  regression test to `@claim:forwarding-config` proving an injected assignment
  cannot appear.

### F-4-4 — MINOR — the no-storage claim test does not cover all browser storage

- **Location / exact claim:** `.factory/claims.json`, `local-only`: “The
  website makes no third-party requests or storage writes”; README: “The
  website requests only same-origin files and writes no browser storage.”
- **Evidence:** the tagged test inspects cookies, `localStorage`, and
  `sessionStorage`, but it does not inspect IndexedDB, Cache Storage, service
  workers, or OPFS. This review manually checked IndexedDB, Cache Storage, and
  service-worker registrations and found them empty, so the current live
  behavior is correct; the declared claim test still would not detect a future
  write to those stores.
- **Why this matters:** “browser storage” is broader than two Web Storage
  objects. The claim is only partly guarded by its required executable test.
- **Concrete fix:** extend `@claim:local-only` through the full
  landing → demo → reset → exit flow and assert empty IndexedDB databases,
  Cache Storage, service-worker registrations, and OPFS state, as well as the
  existing cookies and Web Storage assertions.

### F-4-5 — MINOR — the copy uses two names for the same report metric and receiver

- **Location / exact quotes:** landing recording: “17 fields.” Demo and README:
  “17 field paths.” Landing: “The receiver binds to `127.0.0.1`.” README:
  “The listener binds only to `127.0.0.1`.”
- **Why this matters:** the plain-words contract requires one term for one
  concept. “Fields” can mean top-level keys, while the report actually counts
  nested JSON paths. “Listener” and “receiver” also name the same running
  process in adjacent onboarding material.
- **Concrete fix:** change the recording to **“17 field paths.”** Use
  **receiver** throughout prose; retain `listen` only as the command name.

## Copy audit

Counts are whitespace-delimited after removing Markdown punctuation. Commands,
URLs, and code arguments therefore count as the words a reader sees. No prose
sentence exceeds 22 words, and no banned marketing adjective appears.

### Landing-page sentences

| Location | Sentence | Words | Flag |
| --- | --- | ---: | --- |
| H1 | Inspect a log drain before forwarding | 6 | — |
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
| Recording | 17 fields. | 2 | F-4-5 |
| Recording | 3 findings across 2 fields. | 5 | — |
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
| Local setup | Point your temporary HTTP drain to `http://127.0.0.1:8787/`. | 7 | — |
| Local setup | Use `--ignore-field '$.request_id'` to suppress a reviewed false positive. | 9 | — |
| Footer | Drain Check samples a log drain before you forward it. | 10 | — |

### README sentences

| Line | Sentence | Words | Flag |
| --- | --- | ---: | --- |
| 3 | Inspect a log drain before forwarding it. | 7 | — |
| 5 | Drain Check is for small platform teams adding a managed drain. | 11 | — |
| 5 | It opens a bounded local receiver, summarizes volume and field types, flags likely sensitive data, and writes a JSON report. | 20 | — |
| 9 | Clone the public source repository first: | 6 | — |
| 15 | Uses the sample embedded in the binary and prints the unique report directory. | 13 | — |
| 18 | Point a temporary HTTP drain to `http://127.0.0.1:8787/`. | 7 | — |
| 21 | The listener binds only to `127.0.0.1`. | 6 | F-4-5 |
| 21 | It aggregates accepted events as they arrive, then drops their parsed values and bodies. | 14 | — |
| 21 | Pass `--save-sample sample.ndjson` only when you intentionally want accepted bodies written to disk. | 13 | — |
| 21 | `--output` and `--save-sample` must name different files; Drain Check refuses a collision before opening the listener. | 16 | F-4-5 |
| 23 | Malformed or incomplete requests return HTTP 400 without ending the sample window. | 12 | — |
| 23 | The default rolling limit accepts 20 requests per second, then returns HTTP 429 with `Retry-After: 1`. | 16 | — |
| 23 | Change it with `--rate-limit`. | 4 | — |
| 25 | You can also inspect an existing newline-delimited JSON file: | 9 | — |
| 31 | `inspect --sample-seconds` must be at least 1. | 7 | — |
| 31 | `--json` prints the report to standard output for scripts. | 9 | — |
| 31 | Add `--sensitive-field session_key` for a team-specific field name. | 8 | — |
| 31 | Add `--ignore-field '$.request_id'` after reviewing a false positive. | 8 | — |
| 31 | Keys with punctuation use bracket-quoted paths, such as `$['http.method']`. | 9 | — |
| 31 | A trailing `*` ignores a path prefix. | 7 | — |
| 31 | `cargo run -- --help` lists all commands and options. | 9 | — |
| 35 | Event count, average received event size, and events per second. | 10 | — |
| 35 | NDJSON line delimiters are excluded. | 5 | — |
| 36 | Field paths, observed types, and how many events contained each path. | 11 | — |
| 37 | Conservative risk findings for field names, token-shaped values, and email-shaped values. | 11 | — |
| 38 | Retention estimates for 7 and 30 days. | 7 | — |
| 39 | A recommendation to review the sample before forwarding. | 8 | — |
| 41 | Review each finding before forwarding. | 5 | — |
| 41 | The bundled sample reports 3 events, 17 field paths, 558.1 KiB for 7 days, and 2.3 MiB for 30 days. | 20 | — |
| 45 | Run the separate `forwarding` command after reviewing the report: | 9 | — |
| 51 | The command accepts an HTTP(S) URL and safely encodes it in the generated configuration. | 14 | — |
| 55 | Open the web demo for the bundled sample report. | 9 | — |
| 55 | The command-line equivalent is `cargo run -- demo --json`. | 9 | — |
| 55 | See `.factory/demo.md` for demo storage and reset details. | 8 | — |
| 70 | The static site build writes deployable files and deployment configuration to `dist/site`. | 12 | — |
| 74–75 | Build the site first, then deploy the complete `dist/site` directory with the factory static deployment command: | 16 | — |
| 83 | For another static host, upload the contents of `dist/site` unchanged. | 10 | F-4-2 |
| 83–85 | Keep `staticwebapp.config.json` with the output so the documented routes, 404 page, security headers, and immutable asset caching are deployed together. | 20 | F-4-2 |
| 89 | The website requests only same-origin files and writes no browser storage. | 11 | F-4-4 test coverage |
| 89 | The CLI receives data on its loopback listener. | 8 | F-4-5 |
| 89 | Read the deployed Privacy page and Terms. | 7 | — |
| 89 | Licensed under MIT. | 3 | — |

### Headings, labels, controls, and terminology

- **Local 10-minute sample**, **How it works**, **Review a drain in three
  steps**, **What Drain Check does not retain**, **Run it locally**, and
  **Start a bounded receiver** name their sections without metaphor.
- **Try it with sample data**, **Replay recording**, **Reset demo**, **Read
  local setup**, and **Return home** name their result. **Start for real** is
  the one failed control label; see F-4-1 / F-2-2.
- The README headings — **Install and run**, **What the report contains**,
  **Generate a forwarding configuration**, **Demo**, **Develop, test, and
  build**, **Deploy the static site**, and **Privacy and license** — make sense
  out of context.
- Terminology is otherwise stable around *drain*, *sample*, *report*, *body*,
  *finding*, *sensitive data*, and *forwarding configuration*. F-4-5 records
  the two remaining collisions.

## Demo, sandbox, and CLI behavior

- One click from `/` opens `/?demo=1`. The first 390 px screen already shows
  the demo banner, “Review this drain sample,” and populated 3-event, 0.005/sec,
  17-path, and 3-finding metrics.
- The persistent banner says “Demo — sample data, nothing is saved” and has
  Reset plus an exit action. Reset removed only `demo:drain-check`; a seeded
  `real:review-4=keep` value survived reset, exit, and browser Back.
- The landing → demo → reset → exit flow made only four same-origin requests.
  It set no cookie and left `sessionStorage`, IndexedDB, Cache Storage, and
  service-worker registrations empty. No offline claim is made.
- From an unrelated temporary directory, the clean-clone binary ran `demo
  --json`, wrote its report to `/tmp/drain-check-demo-VQwlPg/report.json`, and
  reported 3 events, 17 field paths, 3 findings, 558.1 KiB/7 days, 2.3 MiB/30
  days, and `bodies_saved: false`. It wrote nothing into the working directory
  except the review’s captured stdout/stderr files.

The demo itself is realistic and isolated. F-4-1 concerns the promise made by
its exit label, not the sample data or storage boundary.

## Claims verification

I cloned the current repository into a fresh temporary directory, ran
`npm ci`, and executed all 18 literal `test` commands from
`.factory/claims.json` separately and in file order.

| Claim ID | Result |
| --- | --- |
| `sample-demo` | PASS |
| `local-only` | PASS; incomplete storage-class coverage is F-4-4 |
| `discard-default` | PASS |
| `contract-report` | PASS |
| `forwarding-config` | PASS; platform-label hardening is F-4-3 |
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
| `site-build-output` | PASS; it does not cover F-4-2’s generic-host promise |
| `mit-license` | PASS |

No literal claim command failed. The full clean-clone suite also passed:
`npm test` 40/40, typecheck, production build, 25/25 Rust tests, Rustfmt, and
Clippy with warnings denied. The build emitted 11.31 kB JavaScript (4.26 kB
gzip) and 6.78 kB CSS (2.21 kB gzip).

F-4-2 is the only claim-like landing/README statement with no matching entry.
F-4-4 is a listed claim whose tagged test observes only part of its stated
storage scope.

## Earlier finding closure

I read reviews 1–3, polish reports 1–3, and the current handoff. I then checked
each earlier finding against both live behavior and current source.

| Earlier finding | Current result |
| --- | --- |
| F-1-1 unavailable registry install | Fixed: the live page and README use the public GitHub checkout; `source-checkout` passes. |
| F-1-2 report/template mismatch | Fixed: the report has a recommendation and the separate forwarding command renders a real configuration. |
| F-1-3 risk-count mismatch | Fixed: all three findings render and the metric says they span two fields. |
| F-1-4 broad unlisted privacy wording | Fixed: the broad search/vendor/telemetry slogans remain absent; narrow behavior is declared. F-4-4 is a new test-scope issue. |
| F-1-5 mood section labels | Fixed: direct labels remain live and in source. |
| F-1-6 inconsistent contract/intent language | Fixed: the old phrases remain absent. F-4-5 is a separate metric/process terminology issue. |
| F-1-7 unclear Terms/404 headings | Fixed: live H1s are “Terms for using Drain Check” and “Page not found.” |
| F-2-1 unlisted CLI promises | Fixed: four narrow claims were added and the untestable detector-limit sentence was removed. |
| F-2-2 demo exit label | **REGRESSED / BLOCKING:** “Start for real” is live again after two polish reports verified “View local setup.” |
| F-3-1 unlisted build-output promise | Fixed: `site-build-output` exists and passes. It does not cover the later generic-host copy in F-4-2. |
| F-3-2 package/release-status promise | Fixed: the sentence remains deleted. |

## Structure, accessibility, links, and visual identity

- `/`, `/?demo=1`, `/demo`, `/privacy`, and `/terms` return 200. `/missing`
  returns a designed HTTP 404 with **Page not found** and a working home link.
- Every route has the required title pattern, one H1, one main landmark,
  description, canonical URL, Open Graph/Twitter metadata, favicon, shared
  header/footer, Privacy and Terms links, and a skip link. The favicon,
  apple-touch icon, OG image, `robots.txt`, and `sitemap.xml` return 200.
- Every crawled live link returned its intended status. The public GitHub
  source link returned 200. Route changes and browser Back move focus to the
  destination H1 and update the polite announcement.
- Across all six routes at 390 × 844 and 1440 × 900, Axe reported zero
  violations and no page overflowed horizontally. Normal routes produced no
  console or page error.
- The live response includes HSTS, `nosniff`, strict-origin referrer policy,
  and a response-header CSP with `frame-ancestors 'none'`.
- The dark pixel/demoscene receiver, scanline texture, local art, squared
  controls, and terminal rhythm match `.factory/design.md`. The page is
  recognisable from a thumbnail and is not a generic gradient-card SaaS
  template.

The structure finding is documentation-specific F-4-2; the current factory
deployment itself does not have broken routing.

## Missed leverage

No additional feature is implied strongly enough to be a finding. The CLI can
inspect NDJSON, export a JSON report, suppress reviewed findings, and generate
a forwarding configuration. Sync would contradict the local inspection
boundary. Sending log samples through an AI gateway would add a privacy and
cost boundary to a deterministic task without a clear user benefit. No AI
feature or embedded provider key is present.

## What would make this perfect

Restore the result-naming demo exit, scope the deployment instructions to
hosts that actually interpret them, reject control characters in platform
labels, make the no-storage claim test cover every browser store, and use
“field paths” and “receiver” consistently. Then rerun every clean-clone claim
command and the complete cold mobile/desktop review. Until all five findings
are gone, the required verdict remains FAIL.
