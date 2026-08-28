# Drain Check

Inspect a log drain before forwarding it.

Drain Check is for small platform teams adding a managed drain. It opens a bounded local receiver, summarizes volume and field types, flags likely secrets, and writes a reviewable JSON report. It is not a log destination.

## Install and run

You need Rust 1.75+ and Node 20+ to build the docs site.

```sh
cargo run -- demo
# Writes a report in your temporary directory from examples/drain.ndjson.

cargo run -- listen --duration 600 --port 8787 --output report.json
# Point a temporary HTTP drain to http://127.0.0.1:8787/.
```

The listener binds only to `127.0.0.1`. It discards bodies after aggregation. Pass `--save-sample sample.ndjson` only when you intentionally want raw bodies written to disk.

You can also inspect an existing newline-delimited JSON file:

```sh
cargo run -- inspect examples/drain.ndjson --sample-seconds 600 --output report.json --json
cargo run -- forwarding --url https://receiver.example/logs
```

`--json` prints the report to standard output for scripts. `cargo run -- --help` lists all commands and options.

## What the report contains

- Event count, average body size, and events per second.
- Field paths, observed types, and how often each appeared.
- Conservative risk findings for field names, token-shaped values, and email-shaped values.
- Retention estimates for 7 and 30 days.
- A destination-neutral HTTP forwarding template.

Risk findings are prompts for review. They do not prove a value is sensitive, and they cannot find every secret.

## Demo

Open [the web demo](https://log-drain-contract-check.sociobot.in/demo) for a self-contained sample report. The command-line equivalent is `cargo run -- demo --json`. See [.factory/demo.md](.factory/demo.md) for the sandbox contract.

## Develop, test, and build

```sh
cargo test
npm install
npm test
npm run build:site
```

The static site build lands in `dist/site` with `index.html` at that root. `cargo package` prepares the CLI crate for publishing; this repository does not publish it.

## Privacy and license

There is no telemetry. The CLI only receives data from the address you configure. Read the deployed [Privacy page](https://log-drain-contract-check.sociobot.in/privacy) and [Terms](https://log-drain-contract-check.sociobot.in/terms). Licensed under [MIT](LICENSE).
