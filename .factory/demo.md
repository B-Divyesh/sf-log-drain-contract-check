# Demo sandbox

- Web demo: `/demo` or `https://log-drain-contract-check.sociobot.in/demo`.
- CLI demo: `drain-check demo --json` from any working directory, including an installed package.
- The CLI embeds `examples/drain.ndjson`: three checkout and worker events, including a fake authorization-shaped value and an email-shaped value.
- The web demo has no writable state. Reset demo removes the reserved `demo:drain-check` storage key if it exists. Start for real returns home.
- The browser demo has no storage namespace because it makes no writes. Reset removes the reserved `demo:drain-check` key defensively.
- The receiver is independent of the web demo. Each `drain-check demo` run creates an unpredictable temporary directory, writes `report.json` there, and prints the path.
