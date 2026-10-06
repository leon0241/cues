use ratatui::widgets::Row;

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

    fn set_name(&mut self) -> String;

    fn get_number(&self) -> f32;

    fn set_number(&mut self);

    fn follow_state(&self) -> String;

    fn edit_cell_value(&mut self, column: CueColumn, value: String);

    fn get_icon(&self) -> &String;

    fn set_icon(&mut self);


    // Target Methods

    fn pause_cue(&self) -> Option<bool>;

    fn stop_cue(&self) -> Option<bool>;

    fn fade_stop_cue(&self, time: f32) -> Option<bool>;

    // Modifier Methods

    fn get_target(&self) -> Option<f32>;

    fn set_target(&mut self, target: f32);
}
