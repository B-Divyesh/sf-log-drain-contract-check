# Drain Check visual thesis

## Direction

**Pixel / demoscene instrument panel.** A log drain is an untrusted data stream entering a tiny local receiver, so the product looks like a calm diagnostic display rather than a cloud dashboard. Chunky scanlines, terminal grid marks, and compact status lamps make the sample window feel bounded and inspectable.

## Tokens

| role | token | value |
| --- | --- | --- |
| night background | `--ink` | `#10152c` |
| panel | `--panel` | `#1d2547` |
| pale text | `--paper` | `#f7f0d5` |
| muted text | `--fog` | `#b7c4dc` |
| cyan signal | `--signal` | `#62e7e8` |
| amber warning | `--amber` | `#ffd66b` |
| coral risk | `--coral` | `#ff8178` |
| green clear | `--mint` | `#97e892` |

The site is deliberately dark-only: this reflects a console used alongside logs. Every text pairing meets 4.5:1 contrast.

## Type, spacing, and interaction

The display face is the device's local monospace stack (used for short labels and the wordmark); body copy uses the local system sans stack for dense technical reading. No remote font is loaded. The 8 px spacing scale is expressed as deliberately deep section breaks (64–104 px) and tighter terminal rows (8–16 px). Buttons have squared corners, inset borders, and labels rather than mystery icons.

Status changes flicker in over 180 ms as if a terminal refreshes. The scanline texture is static; the lone visual pulse is disabled under `prefers-reduced-motion`, where state swaps instantly. Focus is a 3 px cyan outline.

## Art plan and provenance

`site/public/drain-console.webp` is an original raster illustration: a pixel-art local receiver catching bright log packets and sorting them into field, secret, and volume lanes. It was generated with `/opt/fleet/lib/gen-image.sh` (factory-image), then converted to WebP (61 KB). `og-drain-console.webp` is a 1200×630 center crop derived from that original. It contains no text, logos, stock material, or third-party asset. The exact prompt and generation metadata sit in `.factory/assets/drain-console.png.json`; the source PNG remains there too, outside the deploy bundle.

The CLI recording is rendered as selectable HTML text from a real bundled-sample run. Its replay motion reveals four characters per frame and resolves instantly when reduced motion is requested. On phones, the task and sample action precede the illustration so the first action stays in the first viewport.
