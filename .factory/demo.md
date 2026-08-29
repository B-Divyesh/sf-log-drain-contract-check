# Demo sandbox

- Web demo: `/?demo=1` or `https://log-drain-contract-check.sociobot.in/?demo=1`.
- Compatibility route: `/demo` opens the same isolated sample.
- CLI demo: `drain-check demo --json` from any working directory, including an installed package.
- The CLI embeds `examples/drain.ndjson`: three checkout and worker events, including a fake authorization-shaped value and an email-shaped value.
- The web demo has no writable state. Reset demo removes only the reserved `demo:drain-check` storage key. View local setup returns to `/`.
- The browser demo never reads or writes real-data keys. Its reserved namespace is `demo:drain-check`; reset leaves all other keys untouched.
- The receiver is independent of the web demo. Each `drain-check demo` run creates an unpredictable temporary directory, writes `report.json` there, and prints the path.
