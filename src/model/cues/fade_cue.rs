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
    file: Option<AudioFile>,
    icon: String,
    target: Option<f32>
}

#[allow(dead_code)]
impl Cue for FadeCue {
    fn get_row(&self) -> ratatui::widgets::Row<'_>  {
        Row::new(vec![
            self.icon.clone(),
            self.number.to_string(),
            self.name.clone(),
            self.duration.to_string()
        ])
    }

    /// Plays a cue. Does nothing if there is no cue stored.
    fn play_cue(&self) -> color_eyre::Result<()> {
        unimplemented!()
    }


    fn get_type(&self) -> CueType {
        self.cue_type
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&self) -> String {
        todo!()
    }

    fn follow_state(&self) -> String {
        match self.follow {
            FollowState::Follow => {
                String::from("F")
            }
            FollowState::Continue => {
                String::from("C")
            }
            FollowState::None => {
                String::from("")
            }
        }
    }

    fn edit_cell_value(&mut self, column: CueColumn, value: String) {
        if column == CueColumn::Playing {
            self.icon = value
        };
    }

    fn get_icon(&self) -> &String {
        todo!()
    }

    fn set_icon(&self) {
        todo!()
    }

    /// Pauses a cue. Does nothing if there is no cue stored or if no cue is playing.
    fn pause_cue(&self) -> Option<bool> { None }

    /// Stops a cue. Does nothing if there is no cue stored or if no cue is playing.
    fn stop_cue(&self) -> Option<bool> { None }

    /// Fade + Stops a cue.
    fn fade_stop_cue(&self, _time: f32) -> Option<bool> { None }

    fn get_target(&self) -> Option<f32> { None }

    fn set_target(&self, target: f32) -> Option<f32> { None }
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
            file: None,
            icon: String::new(),
            target
        }
    }

    /// Absolute reference to player
    pub fn player(&self) -> &Player {
        self.file.as_ref().unwrap().get_player()
    }

    /// Async capable reference to player
    pub fn player_arc(&self) -> Arc<Player> {
        self.file.as_ref().unwrap().get_player_arc()
    }

    /// Async capable reference to status Atomic U64
    pub fn now_playing(&self) -> Option<Arc<std::sync::atomic::AtomicU64>> {
        self.file.as_ref().map(|file| file.get_playing())
    }

}
