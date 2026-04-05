use mpris::{PlaybackStatus, Player, PlayerFinder};

#[derive(Debug, Clone, Default)]
pub struct TrackInfo {
    pub player_name: String,
    pub title: String,
    pub artist: String,
    pub playback: Playback,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Playback {
    Playing,
    Paused,
    Stopped,
    #[default]
    Unknown,
}

pub struct MprisBackend {
    finder: PlayerFinder,
    active_bus_name: Option<String>,
}

impl MprisBackend {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            finder: PlayerFinder::new()?,
            active_bus_name: None,
        })
    }

    pub fn refresh(&mut self) -> TrackInfo {
        let mut players = match self.finder.find_all() {
            Ok(p) => p,
            Err(_) => return TrackInfo::default(),
        };

        if players.is_empty() {
            self.active_bus_name = None;
            return TrackInfo::default();
        }

        players.sort_by(|a, b| {
            let ap = matches!(a.get_playback_status().ok(), Some(PlaybackStatus::Playing));
            let bp = matches!(b.get_playback_status().ok(), Some(PlaybackStatus::Playing));
            bp.cmp(&ap)
        });

        let selected_index = players
            .iter()
            .position(|p| {
                self.active_bus_name
                    .as_ref()
                    .map(|bus| p.bus_name_player_name_part() == bus)
                    .unwrap_or(false)
            })
            .unwrap_or(0);

        let selected = &players[selected_index];
        self.active_bus_name = Some(selected.bus_name_player_name_part().to_string());
        Self::to_track_info(selected)
    }

    pub fn play_pause(&mut self) {
        if let Some(player) = self.active_player() {
            let _ = player.play_pause();
        }
    }

    pub fn next(&mut self) {
        if let Some(player) = self.active_player() {
            let _ = player.next();
        }
    }

    pub fn previous(&mut self) {
        if let Some(player) = self.active_player() {
            let _ = player.previous();
        }
    }

    fn active_player(&self) -> Option<Player> {
        let players = self.finder.find_all().ok()?;
        if let Some(bus) = &self.active_bus_name {
            if let Some(player) = players
                .into_iter()
                .find(|p| p.bus_name_player_name_part() == bus)
            {
                return Some(player);
            }
        }
        self.finder.find_active().ok()
    }

    fn to_track_info(player: &Player) -> TrackInfo {
        let (title, artist) = match player.get_metadata() {
            Ok(metadata) => {
                let title = metadata.title().unwrap_or_default().to_string();
                let artist = metadata
                    .artists()
                    .map(|artists| artists.join(", "))
                    .unwrap_or_default();
                (title, artist)
            }
            Err(_) => (String::new(), String::new()),
        };

        let playback = match player.get_playback_status().ok() {
            Some(PlaybackStatus::Playing) => Playback::Playing,
            Some(PlaybackStatus::Paused) => Playback::Paused,
            Some(PlaybackStatus::Stopped) => Playback::Stopped,
            _ => Playback::Unknown,
        };

        TrackInfo {
            player_name: player.identity().to_string(),
            title,
            artist,
            playback,
        }
    }
}
