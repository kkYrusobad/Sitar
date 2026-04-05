use gtk::gdk;
use gtk4 as gtk;

use crate::config::ThemeVariant;

pub fn apply_theme(variant: ThemeVariant) {
    let css = css_for_variant(variant);
    let provider = gtk::CssProvider::new();
    provider.load_from_data(css);

    if let Some(display) = gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

fn css_for_variant(variant: ThemeVariant) -> &'static str {
    match variant {
        ThemeVariant::Soft => {
            r#"
#player-window {
  border-radius: 14px;
  overflow: hidden;
}
window {
  background: #32302f;
}
#player-card {
  background: #3c3836;
  /*border: 1px solid #504945;*/
  border: none;
  border-radius: 12px;
  padding: 6px;
  min-height: 52px;
}
#player-card.idle {
  padding: 6px;
  min-height: 34px;
}
#title {
  color: #ebdbb2;
  font-family: \"JetBrainsMono Nerd Font\";
  font-weight: 700;
  font-size: 11px;
}
#subtitle {
  color: #d5c4a1;
  font-family: \"JetBrainsMono Nerd Font\";
  font-size: 9px;
}
#source {
  color: #a9b665;
  font-family: \"JetBrainsMono Nerd Font\";
  font-size: 9px;
}
#controls {
  margin-left: 0;
  margin-top: 4px;
}
#controls.idle {
  margin-top: 2px;
}
/* Icon size tuning (Soft variant):
   - #icon-btn: main playback icons on full card
   - #icon-minimize: minimize icon on full card
   - #mini-speaker: icon inside minimized mode
   - #mini-card: minimized bubble size/background */
#icon-minimize {
  font-family: \"JetBrainsMono Nerd Font\";
  font-size: 11px;
  min-width: 20px;
  min-height: 20px;
  padding: 0;
  border-radius: 999px;
  /*background: rgba(125, 174, 163, 0.10);*/
  background: transparent;
  border: none;
  color: #7daea3;
}
#icon-minimize:hover {
  background: rgba(125, 174, 163, 0.22);
  border-color: #7daea3;
}
#icon-minimize:active {
  background: rgba(125, 174, 163, 0.30);
}
#icon-btn {
  font-family: \"JetBrainsMono Nerd Font\";
  font-size: 11px;
  min-width: 22px;
  min-height: 22px;
  padding: 0;
  border-radius: 999px;
  background: transparent;
  border: none;
  color: #ebdbb2;
}
#icon-btn:disabled {
  opacity: 0.45;
}
#icon-btn:hover {
  background: rgba(169, 182, 101, 0.14);
}
#icon-btn:active {
  background: rgba(169, 182, 101, 0.24);
}
#icon-btn > label,
#icon-minimize > label,
#mini-speaker > label {
  margin: 0;
  padding: 0;
  min-width: 1em;
  min-height: 1em;
  line-height: 1;
}
#mini-speaker > label {
  margin-top: -1px;
}
#mini-card {
  background: #3c3836; 
  border: 1px solid #5a524f;
  border-radius: 999px;
  min-width: 26px;
  min-height: 26px;
  padding: 1px;
  transition: background-color 140ms ease, border-color 140ms ease, box-shadow 140ms ease;
}
#mini-card:hover {
  background: rgba(80, 73, 69, 0.94);
  border-color: #7c6f64;
  box-shadow: 0 0 0 1px rgba(216, 166, 87, 0.18);
}
#mini-speaker {
  font-family: \"JetBrainsMono Nerd Font\";
  font-size: 12px;
  min-width: 22px;
  min-height: 22px;
  border-radius: 999px;
  padding: 0;
  margin: 0;
  color: #ebdbb2;
  background: transparent;
  border: none;
  box-shadow: none;
  transition: color 140ms ease;
}
#mini-speaker:hover {
  background: transparent;
  border: none;
  box-shadow: none;
  color: #ebdbb2;
}
#mini-speaker:active {
  background: transparent;
  border: none;
  box-shadow: none;
  color: #d8a657;
}
"#
        }
        ThemeVariant::Medium => {
            r#"
#player-window {
  border-radius: 14px;
  overflow: hidden;
}
window {
  background: #282828;
}
#player-card {
  background: #32302f;
  border: 1px solid #504945;
  border-radius: 12px;
  padding: 6px;
  min-height: 52px;
}
#player-card.idle {
  padding: 4px;
  min-height: 34px;
}
#title {
  color: #ebdbb2;
  font-family: \"JetBrainsMono Nerd Font\";
  font-weight: 700;
  font-size: 11px;
}
#subtitle {
  color: #d5c4a1;
  font-family: \"JetBrainsMono Nerd Font\";
  font-size: 9px;
}
#source {
  color: #a9b665;
  font-family: \"JetBrainsMono Nerd Font\";
  font-size: 9px;
}
#controls {
  margin-left: 0;
  margin-top: 4px;
}
#controls.idle {
  margin-top: 2px;
}
/* Icon size tuning (Medium variant):
   - #icon-btn: main playback icons on full card
   - #icon-minimize: minimize icon on full card
   - #mini-speaker: icon inside minimized mode
   - #mini-card: minimized bubble size/background */
#icon-minimize {
  font-family: \"JetBrainsMono Nerd Font\";
  font-size: 11px;
  min-width: 20px;
  min-height: 20px;
  padding: 0;
  border-radius: 999px;
  background: rgba(125, 174, 163, 0.10);
  border: 1px solid #5a7a73;
  color: #7daea3;
}
#icon-minimize:hover {
  background: rgba(125, 174, 163, 0.22);
  border-color: #7daea3;
}
#icon-minimize:active {
  background: rgba(125, 174, 163, 0.30);
}
#icon-btn {
  font-family: \"JetBrainsMono Nerd Font\";
  font-size: 12px;
  min-width: 22px;
  min-height: 22px;
  padding: 0;
  border-radius: 999px;
  background: transparent;
  border: none;
  color: #ebdbb2;
}
#icon-btn:disabled {
  opacity: 0.45;
}
#icon-btn:hover {
  background: rgba(169, 182, 101, 0.14);
}
#icon-btn:active {
  background: rgba(169, 182, 101, 0.24);
}
#icon-btn > label,
#icon-minimize > label,
#mini-speaker > label {
  margin: 0;
  padding: 0;
  min-width: 1em;
  min-height: 1em;
  line-height: 1;
}
#mini-speaker > label {
  margin-top: -1px;
}
#mini-card {
  background: rgba(50, 48, 47, 0.92);
  border: 1px solid #5a524f;
  border-radius: 999px;
  min-width: 26px;
  min-height: 26px;
  padding: 1px;
  transition: background-color 140ms ease, border-color 140ms ease, box-shadow 140ms ease;
}
#mini-card:hover {
  background: rgba(80, 73, 69, 0.94);
  border-color: #7c6f64;
  box-shadow: 0 0 0 1px rgba(216, 166, 87, 0.18);
}
#mini-speaker {
  font-family: \"JetBrainsMono Nerd Font\";
  font-size: 12px;
  min-width: 22px;
  min-height: 22px;
  border-radius: 999px;
  padding: 0;
  margin: 0;
  color: #d8a657;
  background: transparent;
  border: none;
  box-shadow: none;
  transition: color 140ms ease;
}
#mini-speaker:hover {
  background: transparent;
  border: none;
  box-shadow: none;
  color: #ebdbb2;
}
#mini-speaker:active {
  background: transparent;
  border: none;
  box-shadow: none;
  color: #d8a657;
}
"#
        }
        ThemeVariant::Hard => {
            r#"
#player-window {
  border-radius: 14px;
  overflow: hidden;
}
window {
  background: #1d2021;
}
#player-card {
  background: #282828;
  border: 1px solid #504945;
  border-radius: 12px;
  padding: 6px;
  min-height: 52px;
}
#player-card.idle {
  padding: 4px;
  min-height: 34px;
}
#title {
  color: #ebdbb2;
  font-family: \"JetBrainsMono Nerd Font\";
  font-weight: 700;
  font-size: 11px;
}
#subtitle {
  color: #d5c4a1;
  font-family: \"JetBrainsMono Nerd Font\";
  font-size: 9px;
}
#source {
  color: #a9b665;
  font-family: \"JetBrainsMono Nerd Font\";
  font-size: 9px;
}
#controls {
  margin-left: 0;
  margin-top: 4px;
}
#controls.idle {
  margin-top: 2px;
}
/* Icon size tuning (Hard variant):
   - #icon-btn: main playback icons on full card
   - #icon-minimize: minimize icon on full card
   - #mini-speaker: icon inside minimized mode
   - #mini-card: minimized bubble size/background */
#icon-minimize {
  font-family: \"JetBrainsMono Nerd Font\";
  font-size: 11px;
  min-width: 20px;
  min-height: 20px;
  padding: 0;
  border-radius: 999px;
  background: rgba(125, 174, 163, 0.10);
  border: 1px solid #5a7a73;
  color: #7daea3;
}
#icon-minimize:hover {
  background: rgba(125, 174, 163, 0.22);
  border-color: #7daea3;
}
#icon-minimize:active {
  background: rgba(125, 174, 163, 0.30);
}
#icon-btn {
  font-family: \"JetBrainsMono Nerd Font\";
  font-size: 12px;
  min-width: 22px;
  min-height: 22px;
  padding: 0;
  border-radius: 999px;
  background: transparent;
  border: none;
  color: #ebdbb2;
}
#icon-btn:disabled {
  opacity: 0.45;
}
#icon-btn:hover {
  background: rgba(169, 182, 101, 0.14);
}
#icon-btn:active {
  background: rgba(169, 182, 101, 0.24);
}
#icon-btn > label,
#icon-minimize > label,
#mini-speaker > label {
  margin: 0;
  padding: 0;
  min-width: 1em;
  min-height: 1em;
  line-height: 1;
}
#mini-speaker > label {
  margin-top: -1px;
}
#mini-card {
  background: rgba(40, 40, 40, 0.94);
  border: 1px solid #5a524f;
  border-radius: 999px;
  min-width: 26px;
  min-height: 26px;
  padding: 1px;
  transition: background-color 140ms ease, border-color 140ms ease, box-shadow 140ms ease;
}
#mini-card:hover {
  background: rgba(80, 73, 69, 0.94);
  border-color: #7c6f64;
  box-shadow: 0 0 0 1px rgba(216, 166, 87, 0.18);
}
#mini-speaker {
  font-family: \"JetBrainsMono Nerd Font\";
  font-size: 12px;
  min-width: 22px;
  min-height: 22px;
  border-radius: 999px;
  padding: 0;
  margin: 0;
  color: #d8a657;
  background: transparent;
  border: none;
  box-shadow: none;
  transition: color 140ms ease;
}
#mini-speaker:hover {
  background: transparent;
  border: none;
  box-shadow: none;
  color: #ebdbb2;
}
#mini-speaker:active {
  background: transparent;
  border: none;
  box-shadow: none;
  color: #d8a657;
}
"#
        }
    }
}
