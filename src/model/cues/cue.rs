use std::any::Any;

use ratatui::widgets::Row;

use crate::model::cues::audio_file::AudioFile;

#[derive(Debug, PartialEq, Eq)]
pub enum CueColumn {
    Playing,
    Name,
    Number,
    Duration,
    Follow
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum FollowState {
    #[default]
    None,
    Follow,
    Continue
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum CueType {
    #[default]
    Audio,
    Start,
    Stop,
    Pause,
    Fade,
    Network,
    Group
}

pub trait Cue {
    // fn new( &self ) -> Box<dyn Cue>;
    /// Returns a table row for TUI
    fn get_row(&self) -> Row<'_> ;

    fn play_cue(&self) -> color_eyre::Result<()>;

    fn get_type(&self) -> CueType;

    fn get_name(&self) -> String;

    fn set_name(&self) -> String;

    fn follow_state(&self) -> String;

    fn edit_cell_value(&mut self, column: CueColumn, value: String);

    fn get_icon(&self) -> &String;

    fn set_icon(&self);
}

pub trait Target {
    fn pause_cue(&self) -> bool;

    fn stop_cue(&self);

    fn fade_stop_cue(&self, time: f32);
}


pub trait Modifier {
    fn get_target(&self) -> i32;

    fn set_target(&self) -> i32;
}
