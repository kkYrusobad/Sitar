# Sitar

Sitar is a minimal floating MPRIS player controller written in Rust for Wayland desktops.

Sitar provides a compact full card with playback controls and a minimized pill mode that can be restored in one click.

## Features

- GTK4 + libadwaita app shell
- MPRIS backend with active-player selection
- Compact full mode and minimized pill mode
- Drag-to-snap behavior with persisted anchor presets
- Gruvbox-inspired theme variants (soft, medium, hard)
- Config persistence at `~/.config/sitar/config.json`
- Optional layer-shell anchoring via Cargo feature

## Requirements

- Linux Wayland session
- GTK4 and libadwaita runtime libraries
- One or more MPRIS-capable media players (native players or browsers)
- Nerd Font installed if you want the default icon glyph set to render correctly

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

Release build:

```bash
cd Sitar
cargo build --release
```

Binary output:

```text
Sitar/target/release/sitar
```

## Local install

```bash
cd Sitar
install -Dm755 target/release/sitar ~/.local/bin/sitar
```

## Configuration

The config file is created automatically at:

```text
~/.config/sitar/config.json
```

Supported keys:

- `snap`: `top-left`, `top-right`, `bottom-left`, `bottom-right`, `center`
- `use_layer_shell`: `true` or `false`
- `snap_margin_px`: integer margin in pixels
- `theme_variant`: `soft`, `medium`, `hard`
- `animation_mode`: `subtle`, `balanced`, `expressive`
- `animation_speed`: float value

Example:

```json
{
	"snap": "top-right",
	"use_layer_shell": true,
	"snap_margin_px": 12,
	"theme_variant": "soft",
	"animation_mode": "balanced",
	"animation_speed": 1.0
}
```

## Controls

- Full mode: use previous, play/pause, next, and minimize icon buttons.
- Minimized mode: click the pill (or speaker icon) to restore full mode.
- Drag in full mode to change snap preset and persist it.

## Theming and icon customization

- Edit icon glyph constants in `src/app.rs`.
- Tune icon size, hitbox, and pill styling in `src/theme.rs`.

## Publish checklist

Before publishing a release:

```bash
cd Sitar
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo package
```

If `cargo package` succeeds, publish with:

```bash
cargo publish
```

## Notes

- MPRIS covers native players and browsers that expose media controls.
- On compositors without layer-shell support, snap is persisted but placement is best-effort.
- When no title or artist metadata is available, Sitar enters a compact idle state, shows "Nothing playing", and keeps only the minimize control visible.
