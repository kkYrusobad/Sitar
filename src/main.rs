mod app;
mod config;
mod media;
mod snap;
mod theme;

use anyhow::Result;

fn main() -> Result<()> {
    app::run()
}
