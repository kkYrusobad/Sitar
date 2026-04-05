use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use adw::prelude::*;
use adw::Application;
use anyhow::Result;
use glib::ControlFlow;
use gtk4 as gtk;
use libadwaita as adw;

use crate::config::{self, Config, SnapPreset};
use crate::media::{MprisBackend, Playback};
use crate::snap::{next_snap_from_drag, DragDelta};
use crate::theme;

// Icon glyphs: edit these constants to swap icon symbols.
// Icon size and hitbox are controlled in src/theme.rs (#icon-btn, #icon-minimize, #mini-speaker, #mini-card).
const ICON_PREV: &str = "";
const ICON_PLAY: &str = "";
const ICON_PAUSE: &str = "";
const ICON_NEXT: &str = "";
const ICON_MINIMIZE: &str = "";
const ICON_MINI_SPEAKER: &str = "";

fn make_icon_button(widget_name: &str, icon: &str) -> gtk::Button {
    let label = gtk::Label::builder()
        .label(icon)
        .xalign(0.5)
        .justify(gtk::Justification::Center)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    label.set_single_line_mode(true);
    label.set_yalign(0.5);
    label.set_hexpand(true);
    label.set_vexpand(true);

    let button = gtk::Button::new();
    button.set_child(Some(&label));
    button.set_widget_name(widget_name);
    button.set_has_frame(false);
    button.set_focusable(false);
    button.set_halign(gtk::Align::Center);
    button.set_valign(gtk::Align::Center);
    button.set_cursor_from_name(Some("pointer"));
    button
}

fn set_icon_button_label(button: &gtk::Button, icon: &str) {
    if let Some(label) = button
        .child()
        .and_then(|child| child.downcast::<gtk::Label>().ok())
    {
        label.set_label(icon);
    }
}

pub fn run() -> Result<()> {
    let _ = adw::init();
    let app = Application::builder()
        .application_id("dev.kky.sitar")
        .build();

    app.connect_activate(build_ui);
    app.run();
    Ok(())
}

fn build_ui(app: &Application) {
    let full_width = 248;
    let full_height = 84;
    let idle_width = 125;
    let idle_height = 56;
    let mini_size = 30;

    let config = match config::load_or_create() {
        Ok(c) => c,
        Err(err) => {
            eprintln!("sitar: failed to load config: {err}");
            Config::default()
        }
    };

    theme::apply_theme(config.theme_variant);

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Sitar")
        .default_width(full_width)
        .default_height(full_height)
        .resizable(false)
        .build();
    window.set_widget_name("player-window");
    window.set_size_request(full_width, full_height);

    let card = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(4)
        .margin_start(4)
        .margin_end(4)
        .margin_top(4)
        .margin_bottom(4)
        .build();
    card.set_widget_name("player-card");
    card.set_valign(gtk::Align::Start);
    card.set_vexpand(false);

    let meta_column = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(1)
        .hexpand(true)
        .valign(gtk::Align::Center)
        .build();

    let title = gtk::Label::builder().label("").xalign(0.0).build();
    title.set_widget_name("title");

    let subtitle = gtk::Label::builder().label("").xalign(0.0).build();
    subtitle.set_widget_name("subtitle");

    let source = gtk::Label::builder().label("").xalign(0.0).build();
    source.set_widget_name("source");

    title.set_ellipsize(gtk::pango::EllipsizeMode::End);
    subtitle.set_ellipsize(gtk::pango::EllipsizeMode::End);
    source.set_ellipsize(gtk::pango::EllipsizeMode::End);

    title.set_visible(false);
    subtitle.set_visible(false);
    source.set_visible(false);

    let controls = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(3)
        .halign(gtk::Align::Start)
        .valign(gtk::Align::Center)
        .build();
    controls.set_widget_name("controls");

    let prev_btn = make_icon_button("icon-btn", ICON_PREV);
    let play_btn = make_icon_button("icon-btn", ICON_PLAY);
    let next_btn = make_icon_button("icon-btn", ICON_NEXT);
    let minimize_btn = make_icon_button("icon-minimize", ICON_MINIMIZE);

    controls.append(&minimize_btn);
    controls.append(&prev_btn);
    controls.append(&play_btn);
    controls.append(&next_btn);

    title.set_text("No active source...");
    title.set_visible(true);
    subtitle.set_visible(false);
    source.set_visible(false);
    prev_btn.set_visible(false);
    play_btn.set_visible(false);
    next_btn.set_visible(false);
    prev_btn.set_sensitive(false);
    play_btn.set_sensitive(false);
    next_btn.set_sensitive(false);
    card.add_css_class("idle");
    controls.add_css_class("idle");

    meta_column.append(&title);
    meta_column.append(&subtitle);
    meta_column.append(&source);

    card.append(&meta_column);
    card.append(&controls);

    let mini_card = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(false)
        .vexpand(false)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    mini_card.set_widget_name("mini-card");

    let mini_speaker_btn = make_icon_button("mini-speaker", ICON_MINI_SPEAKER);
    mini_speaker_btn.set_halign(gtk::Align::Center);
    mini_speaker_btn.set_valign(gtk::Align::Center);
    mini_card.set_cursor_from_name(Some("pointer"));
    mini_card.append(&mini_speaker_btn);

    let stack = gtk::Stack::builder()
        .transition_type(gtk::StackTransitionType::Crossfade)
        .transition_duration(180)
        .hhomogeneous(false)
        .vhomogeneous(false)
        .build();
    stack.add_named(&card, Some("full"));
    stack.add_named(&mini_card, Some("mini"));
    stack.set_visible_child_name("full");

    window.set_content(Some(&stack));
    window.set_size_request(idle_width, idle_height);
    window.set_default_size(idle_width, idle_height);

    let backend = Rc::new(RefCell::new(MprisBackend::new().ok()));
    let cfg_state = Rc::new(RefCell::new(config));
    let is_minimized = Rc::new(RefCell::new(false));
    let is_idle = Rc::new(RefCell::new(true));

    {
        let stack = stack.clone();
        let window = window.clone();
        let is_minimized = Rc::clone(&is_minimized);
        minimize_btn.connect_clicked(move |_| {
            if *is_minimized.borrow() {
                return;
            }
            *is_minimized.borrow_mut() = true;
            stack.set_visible_child_name("mini");

            let window = window.clone();
            glib::timeout_add_local_once(Duration::from_millis(130), move || {
                window.set_size_request(mini_size, mini_size);
                window.set_default_size(mini_size, mini_size);
            });
        });
    }

    {
        let stack_btn = stack.clone();
        let window_btn = window.clone();
        let is_minimized_btn = Rc::clone(&is_minimized);
        let is_idle_btn = Rc::clone(&is_idle);
        mini_speaker_btn.connect_clicked(move |_| {
            if !*is_minimized_btn.borrow() {
                return;
            }
            *is_minimized_btn.borrow_mut() = false;
            let (target_w, target_h) = if *is_idle_btn.borrow() {
                (idle_width, idle_height)
            } else {
                (full_width, full_height)
            };
            window_btn.set_size_request(target_w, target_h);
            window_btn.set_default_size(target_w, target_h);
            stack_btn.set_visible_child_name("full");
        });

        let stack_click = stack.clone();
        let window_click = window.clone();
        let is_minimized_click = Rc::clone(&is_minimized);
        let is_idle_click = Rc::clone(&is_idle);
        let mini_click = gtk::GestureClick::new();
        mini_click.connect_released(move |_, _, _, _| {
            if !*is_minimized_click.borrow() {
                return;
            }
            *is_minimized_click.borrow_mut() = false;
            let (target_w, target_h) = if *is_idle_click.borrow() {
                (idle_width, idle_height)
            } else {
                (full_width, full_height)
            };
            window_click.set_size_request(target_w, target_h);
            window_click.set_default_size(target_w, target_h);
            stack_click.set_visible_child_name("full");
        });
        mini_card.add_controller(mini_click);
    }

    {
        let backend = Rc::clone(&backend);
        prev_btn.connect_clicked(move |_| {
            if let Some(backend) = backend.borrow_mut().as_mut() {
                backend.previous();
            }
        });
    }
    {
        let backend = Rc::clone(&backend);
        play_btn.connect_clicked(move |_| {
            if let Some(backend) = backend.borrow_mut().as_mut() {
                backend.play_pause();
            }
        });
    }
    {
        let backend = Rc::clone(&backend);
        next_btn.connect_clicked(move |_| {
            if let Some(backend) = backend.borrow_mut().as_mut() {
                backend.next();
            }
        });
    }

    {
        let title = title.clone();
        let subtitle = subtitle.clone();
        let source = source.clone();
        let play_btn = play_btn.clone();
        let prev_btn = prev_btn.clone();
        let next_btn = next_btn.clone();
        let window = window.clone();
        let card = card.clone();
        let controls = controls.clone();
        let is_idle = Rc::clone(&is_idle);
        let is_minimized = Rc::clone(&is_minimized);
        let backend = Rc::clone(&backend);

        glib::timeout_add_seconds_local(1, move || {
            let mut backend_ref = backend.borrow_mut();
            let Some(backend) = backend_ref.as_mut() else {
                return ControlFlow::Continue;
            };

            let track = backend.refresh();
            let has_title = !track.title.trim().is_empty();
            let has_artist = !track.artist.trim().is_empty();
            let has_source = !track.player_name.trim().is_empty();
            let now_idle = !(has_title || has_artist);

            if now_idle {
                title.set_text("No active source...");
                title.set_visible(true);
                subtitle.set_visible(false);
                source.set_visible(false);
                prev_btn.set_visible(false);
                play_btn.set_visible(false);
                next_btn.set_visible(false);
                prev_btn.set_sensitive(false);
                play_btn.set_sensitive(false);
                next_btn.set_sensitive(false);
            } else {
                if has_title {
                    title.set_text(&track.title);
                }
                title.set_visible(has_title);

                if has_artist {
                    subtitle.set_text(&track.artist);
                }
                subtitle.set_visible(has_artist);

                if has_source {
                    source.set_text(&track.player_name);
                }
                source.set_visible(has_source);

                prev_btn.set_visible(true);
                play_btn.set_visible(true);
                next_btn.set_visible(true);
                prev_btn.set_sensitive(true);
                play_btn.set_sensitive(true);
                next_btn.set_sensitive(true);

                let play_icon = match track.playback {
                    Playback::Playing => ICON_PAUSE,
                    Playback::Paused | Playback::Stopped | Playback::Unknown => ICON_PLAY,
                };
                set_icon_button_label(&play_btn, play_icon);
            }

            let was_idle = *is_idle.borrow();
            if was_idle != now_idle {
                *is_idle.borrow_mut() = now_idle;

                if now_idle {
                    card.add_css_class("idle");
                    controls.add_css_class("idle");
                } else {
                    card.remove_css_class("idle");
                    controls.remove_css_class("idle");
                }

                if !*is_minimized.borrow() {
                    let (target_w, target_h) = if now_idle {
                        (idle_width, idle_height)
                    } else {
                        (full_width, full_height)
                    };
                    window.set_size_request(target_w, target_h);
                    window.set_default_size(target_w, target_h);
                }
            }

            ControlFlow::Continue
        });
    }

    {
        let window_weak_full = window.downgrade();
        let cfg_state_full = Rc::clone(&cfg_state);
        let drag = gtk::GestureDrag::new();

        drag.connect_drag_end(move |_, dx, dy| {
            let mut cfg = cfg_state_full.borrow().clone();
            cfg.snap = next_snap_from_drag(cfg.snap, DragDelta { dx, dy });
            let _ = config::save(&cfg);
            *cfg_state_full.borrow_mut() = cfg.clone();

            if let Some(window) = window_weak_full.upgrade() {
                apply_snap(&window, cfg.snap, cfg.snap_margin_px, cfg.use_layer_shell);
            }
        });

        card.add_controller(drag);
    }

    {
        let cfg = cfg_state.borrow().clone();
        apply_snap(&window, cfg.snap, cfg.snap_margin_px, cfg.use_layer_shell);
    }

    window.present();
}

fn apply_snap(
    window: &adw::ApplicationWindow,
    snap: SnapPreset,
    margin: i32,
    use_layer_shell: bool,
) {
    #[cfg(feature = "layer-shell")]
    {
        if use_layer_shell {
            crate::snap::apply_layer_anchor(
                window.upcast_ref::<gtk::ApplicationWindow>(),
                snap,
                margin,
            );
        }
    }

    #[cfg(not(feature = "layer-shell"))]
    {
        let _ = (window, snap, margin, use_layer_shell);
    }
}
