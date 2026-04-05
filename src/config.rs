use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub snap: SnapPreset,
    pub use_layer_shell: bool,
    pub snap_margin_px: i32,
    pub theme_variant: ThemeVariant,
    pub animation_mode: AnimationMode,
    pub animation_speed: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SnapPreset {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ThemeVariant {
    Soft,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AnimationMode {
    Subtle,
    Balanced,
    Expressive,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            snap: SnapPreset::TopRight,
            use_layer_shell: true,
            snap_margin_px: 12,
            theme_variant: ThemeVariant::Soft,
            animation_mode: AnimationMode::Balanced,
            animation_speed: 1.0,
        }
    }
}

pub fn config_file_path() -> Result<PathBuf> {
    let proj = ProjectDirs::from("dev", "kky", "sitar")
        .context("failed to resolve ~/.config/sitar path")?;
    let cfg_dir = proj.config_dir();
    fs::create_dir_all(cfg_dir).context("failed to create config directory")?;
    Ok(cfg_dir.join("config.json"))
}

pub fn load_or_create() -> Result<Config> {
    let path = config_file_path()?;
    if !path.exists() {
        let cfg = Config::default();
        let content = serde_json::to_string_pretty(&cfg)?;
        fs::write(&path, content).context("failed to write initial config")?;
        return Ok(cfg);
    }

    let raw = fs::read_to_string(&path).context("failed to read config")?;
    match serde_json::from_str::<Config>(&raw) {
        Ok(cfg) => Ok(cfg),
        Err(_) => {
            let cfg = Config::default();
            let content = serde_json::to_string_pretty(&cfg)?;
            fs::write(&path, content).context("failed to repair invalid config")?;
            Ok(cfg)
        }
    }
}

pub fn save(cfg: &Config) -> Result<()> {
    let path = config_file_path()?;
    let content = serde_json::to_string_pretty(cfg)?;
    fs::write(path, content).context("failed to save config")?;
    Ok(())
}
