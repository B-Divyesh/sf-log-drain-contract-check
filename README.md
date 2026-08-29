# Drain Check

Inspect a log drain before forwarding it.

Drain Check is for small platform teams adding a managed drain. It opens a bounded local receiver, summarizes volume and field types, flags likely secrets, and writes a JSON report. It is not a log destination.

## Install and run

You need Rust 1.75+ and Node 20+ to build the docs site.

```sh
cargo run -- demo --json
# Uses the sample embedded in the binary and prints the unique report directory.

cargo run -- listen --duration 600 --port 8787 --output report.json
# Point a temporary HTTP drain to http://127.0.0.1:8787/.
```

The listener binds only to `127.0.0.1`. It aggregates accepted events as they arrive, then drops their parsed values and bodies. Pass `--save-sample sample.ndjson` only when you intentionally want accepted bodies written to disk.

Malformed or incomplete requests return HTTP 400 without ending the sample window. The default rolling limit accepts 20 requests per second, then returns HTTP 429 with `Retry-After: 1`. Change it with `--rate-limit`.

You can also inspect an existing newline-delimited JSON file:

```sh
cargo run -- inspect examples/drain.ndjson --sample-seconds 600 --output report.json --json
cargo run -- forwarding --url https://receiver.example/logs
```

`--json` prints the report to standard output for scripts. Add `--sensitive-field session_key` for a team-specific field name. Add `--ignore-field '$.request_id'` after reviewing a false positive. A trailing `*` ignores a path prefix. `cargo run -- --help` lists all commands and options.

## What the report contains

- Event count, average body size, and events per second.
- Field paths, observed types, and how many events contained each path.
- Conservative risk findings for field names, token-shaped values, and email-shaped values.
- Retention estimates for 7 and 30 days.
- A destination-neutral HTTP forwarding template.

Risk findings are prompts for review. They do not prove a value is sensitive, and they cannot find every secret. The bundled sample reports 3 events, 17 field paths, 558.1 KiB for 7 days, and 2.3 MiB for 30 days.

## Demo

Open [the web demo](https://log-drain-contract-check.sociobot.in/demo) for a self-contained sample report. The command-line equivalent is `cargo run -- demo --json`. See [.factory/demo.md](.factory/demo.md) for the sandbox contract.

## Develop, test, and build

```sh
cargo test --all-targets --all-features --locked
npm ci
npm test
npm run typecheck
npm run build:site
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --locked
```

The static site build lands in `dist/site` with its deployment configuration at that root. `cargo package` prepares the CLI crate for publishing; this repository does not publish it.

## Privacy and license

There is no telemetry. The CLI only receives data on its loopback listener. Read the deployed [Privacy page](https://log-drain-contract-check.sociobot.in/privacy) and [Terms](https://log-drain-contract-check.sociobot.in/terms). Licensed under [MIT](LICENSE).
