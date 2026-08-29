# Copy audit

Audited 2026-08-29 against the rendered landing page. Commands, navigation labels, and metric fragments are listed separately. Word counts treat an inline command as one word.

## Landing-page sentences

| Location | Sentence | Words | Result |
| --- | --- | ---: | --- |
| H1 | Check a log drain before forwarding. | 6 | Pass |
| First screen | For platform teams checking volume, field types, and sensitive data before enabling a log drain. | 15 | Pass |
| First screen | Opens the bundled report. | 4 | Pass |
| First screen | Writes no browser data. | 4 | Pass |
| First screen fact | The receiver binds to `127.0.0.1`. | 5 | Pass |
| First screen fact | Accepted bodies are discarded by default. | 6 | Pass |
| First screen fact | Free under the MIT License. | 5 | Pass |
| Recording help | A text recording of the bundled CLI demo. | 8 | Pass |
| Recording help | Use the replay button to play it again. | 8 | Pass |
| How it works | Run the receiver locally. | 4 | Pass |
| How it works | Run one bounded window. | 4 | Pass |
| How it works | Review the report. | 3 | Pass |
| How it works | Check field paths and likely sensitive data. | 7 | Pass |
| How it works | Generate a forwarding configuration. | 4 | Pass |
| How it works | Review the generated configuration. | 4 | Pass |
| Data handling | The receiver discards accepted bodies after aggregation by default. | 9 | Pass |
| Data handling | Saving accepted bodies requires `--save-sample`. | 5 | Pass |
| Local setup | Clone the public source repository on GitHub, then run the receiver. | 11 | Pass |
| Local setup | Point your temporary HTTP drain to `http://127.0.0.1:8787/`. | 8 | Pass |
| Local setup | Use `--ignore-field '$.request_id'` to suppress a reviewed false positive. | 9 | Pass |
| Footer | Drain Check samples a log drain before you forward it. | 10 | Pass |

No sentence exceeds 22 words. No banned word appears.

## Labels and recorded output

| Location | Text | Result |
| --- | --- | --- |
| First-screen label | Local 10-minute sample | Direct section name |
| First-screen action | Try it with sample data | Result-naming verb |
| Demo exit action | View local setup | Names the screen the action opens |
| Art caption | The receiver stays local | Narrow tested claim |
| Recording | Reviewed 3 events in 600s. 17 field paths. 3 findings in 2 field paths. | Matches the bundled report |
| Section label | How it works | Direct section name |
| Section heading | Review a drain in three steps | Direct section name |
| Section heading | What Drain Check does not retain | Direct section name |
| Section heading | Start a bounded receiver | Result-naming verb |

## Terminology

| Concept | Term used |
| --- | --- |
| log delivery integration | drain |
| one bounded set of events | sample |
| CLI output | report |
| HTTP payload | body |
| detector warning | finding |
| data needing review | sensitive data |
| destination setup | forwarding configuration |
| approved detector exception | ignored field |
| running local process | receiver |
| report location | field path |

Catalog description: `Check local log drains for volume, field paths, and sensitive data before forwarding.` (13 words, 82 characters).
