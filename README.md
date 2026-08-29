# Drain Check

Inspect a log drain before forwarding it.

Drain Check is for small platform teams adding a managed drain. It opens a bounded local receiver, summarizes volume and field types, flags likely sensitive data, and writes a JSON report.

## Install and run

Clone the public source repository first:

```sh
git clone https://github.com/B-Divyesh/sf-log-drain-contract-check.git
cd sf-log-drain-contract-check
cargo run -- demo --json
# Uses the sample embedded in the binary and prints the unique report directory.

cargo run -- listen --duration 600 --port 8787 --output report.json
# Point a temporary HTTP drain to http://127.0.0.1:8787/.
```

The listener binds only to `127.0.0.1`. It aggregates accepted events as they arrive, then drops their parsed values and bodies. Pass `--save-sample sample.ndjson` only when you intentionally want accepted bodies written to disk. `--output` and `--save-sample` must name different files; Drain Check refuses a collision before opening the listener.

Malformed or incomplete requests return HTTP 400 without ending the sample window. The default rolling limit accepts 20 requests per second, then returns HTTP 429 with `Retry-After: 1`. Change it with `--rate-limit`.

You can also inspect an existing newline-delimited JSON file:

```sh
cargo run -- inspect examples/drain.ndjson --sample-seconds 600 --output report.json --json
```

`inspect --sample-seconds` must be at least 1. `--json` prints the report to standard output for scripts. Add `--sensitive-field session_key` for a team-specific field name. Add `--ignore-field '$.request_id'` after reviewing a false positive. Keys with punctuation use bracket-quoted paths, such as `$['http.method']`. A trailing `*` ignores a path prefix. `cargo run -- --help` lists all commands and options.

## What the report contains

- Event count, average received event size, and events per second. NDJSON line delimiters are excluded.
- Field paths, observed types, and how many events contained each path.
- Conservative risk findings for field names, token-shaped values, and email-shaped values.
- Retention estimates for 7 and 30 days.
- A recommendation to review the sample before forwarding.

Review each finding before forwarding. The bundled sample reports 3 events, 17 field paths, 558.1 KiB for 7 days, and 2.3 MiB for 30 days.

## Generate a forwarding configuration

Run the separate `forwarding` command after reviewing the report:

```sh
cargo run -- forwarding --url https://receiver.example/logs
```

The command accepts an HTTP(S) URL and safely encodes it in the generated configuration.

## Demo

Open [the web demo](https://log-drain-contract-check.sociobot.in/?demo=1) for the bundled sample report. The command-line equivalent is `cargo run -- demo --json`. See [.factory/demo.md](.factory/demo.md) for demo storage and reset details.

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

The static site build writes deployable files and deployment configuration to `dist/site`.

## Deploy the static site

Build the site first, then deploy the complete `dist/site` directory with the
factory static deployment command:

```sh
npm ci
npm run build:site
/opt/fleet/lib/deploy-static.sh log-drain-contract-check dist/site
```

For another static host, upload the contents of `dist/site` unchanged. Keep
`staticwebapp.config.json` with the output so the documented routes, 404 page,
security headers, and immutable asset caching are deployed together.

## Privacy and license

The website requests only same-origin files and writes no browser storage. The CLI receives data on its loopback listener. Read the deployed [Privacy page](https://log-drain-contract-check.sociobot.in/privacy) and [Terms](https://log-drain-contract-check.sociobot.in/terms). Licensed under [MIT](LICENSE).
