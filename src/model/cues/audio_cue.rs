use std::sync::{Arc, atomic::{AtomicU64, Ordering}};

use ratatui::widgets::Row;
use rodio::Player;

use crate::model::cues::{audio_file::AudioFile, cue::{Cue, CueColumn, CueType, FollowState}};

pub struct AudioCue {
    name: String,
    number: f32,
    cue_type: CueType,
    duration: f32,
    follow: FollowState,
    // colour: Option<String>,
    file: Option<AudioFile>,
    icon: String
}

trait AudioCueFn {
    fn player(&self) -> &Player;

    fn player_arc(&self) -> Arc<Player>;

    fn now_playing(&self) -> Option<Arc<std::sync::atomic::AtomicU64>>;
}

impl AudioCueFn for AudioCue {
    /// Absolute reference to player
    fn player(&self) -> &Player {
        self.file.as_ref().unwrap().get_player()
    }

    /// Async capable reference to player
    fn player_arc(&self) -> Arc<Player> {
        self.file.as_ref().unwrap().get_player_arc()
    }

    /// Async capable reference to status Atomic U64
    fn now_playing(&self) -> Option<Arc<std::sync::atomic::AtomicU64>> {
        self.file.as_ref().map(|file| file.get_playing())
    }
}

impl AudioCue {
    pub fn new(number: f32, name: String, duration: f32,
        follow: FollowState, file: Option<AudioFile>) -> Self {
        Self {
            number,
            name,
            duration,
            follow,
            cue_type: CueType::Audio,
            // colour: None,
            file,
            icon: String::new(),
        }
    }
}
