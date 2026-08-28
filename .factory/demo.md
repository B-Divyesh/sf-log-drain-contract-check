# Demo sandbox

- Web demo: `/demo` or `https://log-drain-contract-check.sociobot.in/demo`.
- CLI demo: `cargo run -- demo --json`.
- The CLI sample is `examples/drain.ndjson`: three checkout and worker events, including a fake authorization-shaped value and an email-shaped value.
- The web demo has no writable state. Reset demo removes the reserved `demo:drain-check` storage key if it exists. Start for real returns home.
- The receiver is independent of the web demo. `drain-check demo` writes only a temporary report at the operating system temp path.
