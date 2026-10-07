use color_eyre::eyre::eyre;
use rodio::{Decoder, Player};
use std::{io::Cursor, sync::{Arc, atomic::AtomicU64}};

use crate::model::audio_config::AudioDevice;

#[allow(dead_code)]
// #[derive(Debug)]
pub struct AudioFile {
    name: String,
    file_path: String,
    bytes: Arc<[u8]>,
    player: Arc<Player>,
    now_playing: Arc<AtomicU64>,
}

impl AudioFile {
    /// # Errors
    ///
    /// Will return `Err` if no default sink
    pub fn new(name: String, file_path: String, mixer: &AudioDevice) -> color_eyre::Result<Self> {
        // Read File info. Arc it so it can be referenced repeatedly using a cursor
        let bytes: Arc<[u8]> = std::fs::read(&file_path)?.into();

        let mixer_sink = mixer.sink
            .as_ref()
            .ok_or_else(|| eyre!("Error no mixer"))?
            .mixer();

        let player: Arc<Player> = Arc::new(Player::connect_new(mixer_sink));

        let now_playing: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));

        Ok(Self {
            name,
            file_path,
            bytes,
            player,
            now_playing
        })
    }

    /// Reference to Rodio player
    #[must_use]
    pub fn get_player(&self) -> &Player {
        &self.player
    }

    /// Async capable reference to Rodio player
    #[must_use]
    pub fn get_player_arc(&self) -> Arc<Player> {
        Arc::clone(&self.player)
    }

    /// Async capable reference to currently playing AU64
    #[must_use]
    pub fn get_playing(&self) -> Arc<AtomicU64> {
        Arc::clone(&self.now_playing)
    }

    /// Plays file from the source
    ///
    /// # Errors
    ///
    /// Will return `Err` if no default sink
    ///
    pub fn play_file(&self) -> color_eyre::Result<()> {
        // Decode that sound file into a source
        let source: Decoder<Cursor<Arc<[u8]>>> = Decoder::try_from(Cursor::new(self.bytes.clone()))?;

        self.player.append(source);
        Ok(())
    }
}

