# Changelog

## 0.1.1

- Reject colliding report and saved-sample paths before the receiver binds.
- Require a file sample window of at least one second.
- Parse forwarding destinations as HTTP(S) URLs and safely encode generated configuration values.

## 0.1.0

- First local drain sample receiver and contract report.
- Reject malformed and incomplete requests without ending the sample window.
- Aggregate accepted events immediately, add rate limiting, and support detector overrides.
- Embed the CLI demo sample and generate matching web report values.
