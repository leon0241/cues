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

    fn set_name(&mut self) -> String {
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

    fn set_icon(&mut self) {
        todo!()
    }

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
