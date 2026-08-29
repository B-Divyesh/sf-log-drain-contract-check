# Changelog

## 0.1.1

- Reject colliding report and saved-sample paths before the receiver binds.
- Require a file sample window of at least one second.
- Parse forwarding destinations as HTTP(S) URLs and safely encode generated configuration values.
- Reconcile detector findings with reviewed field counts in CLI and web demo output.
- Document the source-checkout install path and separate forwarding configuration from reports.

## 0.1.0

- First local drain sample receiver and field report.
- Reject malformed and incomplete requests without ending the sample window.
- Aggregate accepted events immediately, add rate limiting, and support detector overrides.
- Embed the CLI demo sample and generate matching web report values.
