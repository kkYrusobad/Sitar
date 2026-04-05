# Sitar

Sitar is a minimal floating music player written in Rust.

Goals for v1:
- Standalone runtime (no Niruv dependency)
- MPRIS-first media detection and controls
- Minimal UI with Niruv-inspired Gruvbox styling
- Drag-to-snap behavior with persisted preset anchors

## Current status

Initial implementation scaffold includes:
- GTK4 + libadwaita app shell
- MPRIS backend with active-player selection
- Ultra-compact rounded player card with icon controls
- Conditional metadata rendering with minimal placeholder when idle
- Config persistence at `~/.config/sitar/config.json`
- Optional layer-shell anchoring via Cargo feature

## Build

```bash
cd Sitar
cargo run
```

With optional layer-shell support:

```bash
cd Sitar
cargo run --features layer-shell
```

## Notes

- MPRIS covers native players and browsers that expose media controls.
- On compositors without layer-shell support, snap is persisted but placement is best-effort.
