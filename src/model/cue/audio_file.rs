use color_eyre::eyre::eyre;
use rodio::{Decoder, Player};
use std::{
    io::Cursor,
    sync::{Arc, atomic::AtomicU16},
};

use crate::model::audio_config::AudioDevice;

/// Audio File
///
/// Contains all the rodio logic required to play
/// audio file cues.
#[allow(dead_code)]
pub struct AudioFile {
    name: String,
    file_path: String,
    bytes: Arc<[u8]>,
    player: Arc<Player>,
    now_playing: Arc<AtomicU16>,
}

impl AudioFile {
    /// Creates a new `AudioFile`
    ///
    /// A (relative) file path is needed, as well as an `AudioDevice`
    /// More information can be found in [`model/audio_config.rs`]
    ///
    /// # Errors
    ///
    /// Will return `Err` if no default sink can be found.
    pub fn new(name: String, file_path: String, mixer: &AudioDevice) -> color_eyre::Result<Self> {
        // Read File info. Arc it so it can be referenced repeatedly using a cursor
        let bytes: Arc<[u8]> = std::fs::read(&file_path)?.into();

        let mixer_sink = mixer
            .sink
            .as_ref()
            .ok_or_else(|| eyre!("Error no mixer"))?
            .mixer();

        let player: Arc<Player> = Arc::new(Player::connect_new(mixer_sink));

        let now_playing: Arc<AtomicU16> = Arc::new(AtomicU16::new(0));

        Ok(Self {
            name,
            file_path,
            bytes,
            player,
            now_playing,
        })
    }

    /// Absolute reference to player
    ///
    /// This is a reference to the Rodio Player.
    #[must_use]
    pub fn get_player(&self) -> &Player {
        &self.player
    }

    /// Async capable reference to player
    ///
    /// This returns an Arc Clone of the Player. Useful for
    /// when the reference needs to outlive the function
    /// such as a play cue function.
    #[must_use]
    pub fn get_player_arc(&self) -> Arc<Player> {
        Arc::clone(&self.player)
    }

    /// Async capable reference to status Atomic U16
    ///
    /// This returns an Arc clone of the now playing variable
    /// of the file (an `AtomicU16`) which tracks ownership of
    /// a modifier. Useful for cues like fades which can override
    /// each other.
    ///
    /// If anything higher than 65535 is needed I will be shocked.
    #[must_use]
    pub fn get_playing(&self) -> Arc<AtomicU16> {
        Arc::clone(&self.now_playing)
    }

    /// Plays a file
    ///
    /// Generates a Rodio Decoder from stored Bytes
    /// within the `AudioFile` struct.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the decoder cannot read the file
    pub fn play_file(&self) -> color_eyre::Result<()> {
        // Decode that sound file into a source
        let source: Decoder<Cursor<Arc<[u8]>>> =
            Decoder::try_from(Cursor::new(self.bytes.clone()))?;

        self.player.append(source);
        Ok(())
    }

    /// Returns the audio file path
    #[must_use]
    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    /// Sets the file path to the file.
    ///
    /// **NOTE**: This does not read the file / store its contents.
    /// This means that `try_read()` will have to be called every time
    /// a new `file_path` is set.
    pub fn set_file_path(&mut self, file_path: &str) {
        self.file_path = file_path.to_string();
    }

    /// Reads byes from the stored file path
    ///
    /// This is necessary whenever a new file path is stored,
    /// however the function is separate to separate the potential
    /// `Err` result from not being able to read the file.
    ///
    /// # Errors
    ///
    /// Will return `Err` if error reading file
    pub fn try_read(&mut self) -> color_eyre::Result<()> {
        self.bytes = std::fs::read(self.file_path.clone())?.into();
        Ok(())
    }
}
