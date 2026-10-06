use std::sync::{Arc};

use ratatui::widgets::Row;
use rodio::Player;

use crate::model::cues::{audio_file::AudioFile, cue::{Cue, CueColumn, CueType, FollowState}};

pub struct FadeCue {
    name: String,
    number: f32,
    cue_type: CueType,
    duration: f32,
    follow: FollowState,
    // colour: Option<String>,
    icon: String,
    target: Option<f32>
}

#[allow(dead_code)]
impl FadeCue {
    /// Pauses a cue. Does nothing if there is no cue stored or if no cue is playing.
    fn pause_cue(&self) -> Option<bool> { None }

    /// Stops a cue. Does nothing if there is no cue stored or if no cue is playing.
    fn stop_cue(&self) -> Option<bool> { None }

    /// Fade + Stops a cue.
    fn fade_stop_cue(&self, _time: f32) -> Option<bool> { None }

    fn get_target(&self) -> Option<f32> {
        self.target
    }

    fn set_target(&mut self, target: f32) {
        self.target = Some(target)
    }

    fn get_number(&self) -> f32 {
        self.number
    }

    fn set_number(&mut self) { }
}

impl FadeCue {
    pub fn new(number: f32, name: String, duration: f32,
        follow: FollowState, target: Option<f32>) -> Self {

        Self {
            number,
            name,
            duration,
            follow,
            cue_type: CueType::Fade,
            // colour: None,
            icon: String::new(),
            target
        }
    }
}
