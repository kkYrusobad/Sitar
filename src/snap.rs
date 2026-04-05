use crate::config::SnapPreset;

#[derive(Debug, Clone, Copy)]
pub struct DragDelta {
    pub dx: f64,
    pub dy: f64,
}

pub fn next_snap_from_drag(current: SnapPreset, drag: DragDelta) -> SnapPreset {
    let threshold = 36.0;
    let abs_x = drag.dx.abs();
    let abs_y = drag.dy.abs();

    if abs_x < threshold && abs_y < threshold {
        return SnapPreset::Center;
    }

    if abs_x >= abs_y {
        if drag.dx > 0.0 {
            match current {
                SnapPreset::TopLeft => SnapPreset::TopRight,
                SnapPreset::BottomLeft => SnapPreset::BottomRight,
                SnapPreset::Center => SnapPreset::TopRight,
                other => other,
            }
        } else {
            match current {
                SnapPreset::TopRight => SnapPreset::TopLeft,
                SnapPreset::BottomRight => SnapPreset::BottomLeft,
                SnapPreset::Center => SnapPreset::TopLeft,
                other => other,
            }
        }
    } else if drag.dy > 0.0 {
        match current {
            SnapPreset::TopLeft => SnapPreset::BottomLeft,
            SnapPreset::TopRight => SnapPreset::BottomRight,
            SnapPreset::Center => SnapPreset::BottomRight,
            other => other,
        }
    } else {
        match current {
            SnapPreset::BottomLeft => SnapPreset::TopLeft,
            SnapPreset::BottomRight => SnapPreset::TopRight,
            SnapPreset::Center => SnapPreset::TopRight,
            other => other,
        }
    }
}

#[cfg(feature = "layer-shell")]
pub fn apply_layer_anchor(window: &gtk4::ApplicationWindow, snap: SnapPreset, margin: i32) {
    use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

    if !window.is_layer_window() {
        window.init_layer_shell();
    }
    window.set_layer(Layer::Top);
    window.set_keyboard_mode(KeyboardMode::OnDemand);

    window.set_anchor(Edge::Top, false);
    window.set_anchor(Edge::Bottom, false);
    window.set_anchor(Edge::Left, false);
    window.set_anchor(Edge::Right, false);

    window.set_margin(Edge::Top, margin);
    window.set_margin(Edge::Bottom, margin);
    window.set_margin(Edge::Left, margin);
    window.set_margin(Edge::Right, margin);

    match snap {
        SnapPreset::TopLeft => {
            window.set_anchor(Edge::Top, true);
            window.set_anchor(Edge::Left, true);
        }
        SnapPreset::TopRight => {
            window.set_anchor(Edge::Top, true);
            window.set_anchor(Edge::Right, true);
        }
        SnapPreset::BottomLeft => {
            window.set_anchor(Edge::Bottom, true);
            window.set_anchor(Edge::Left, true);
        }
        SnapPreset::BottomRight => {
            window.set_anchor(Edge::Bottom, true);
            window.set_anchor(Edge::Right, true);
        }
        SnapPreset::Center => {
            // Center is best-effort in layer-shell mode; keep top-right fallback.
            window.set_anchor(Edge::Top, true);
            window.set_anchor(Edge::Right, true);
        }
    }
}
