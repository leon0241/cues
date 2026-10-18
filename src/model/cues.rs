pub mod audio_cue;
pub mod audio_file;
pub mod cue_base;
pub mod fade_cue;
pub mod pause_cue;
pub mod start_cue;
pub mod stop_cue;

use crate::model::cues::cue_base::{CueType, FollowState, HasCueBase, PlayState};

use ratatui::widgets::Row;

#[derive(Debug, PartialEq, Eq)]
pub enum CueColumn {
    Playing,
    Name,
    Number,
    Target,
    PreWait,
    PostWait,
    Duration,
    Follow,
}

impl CueColumn {
    #[must_use]
    pub const fn from_index(i: usize) -> Option<Self> {
        match i {
            0 => Some(Self::Playing),
            // 1 is the cuetype icon which will never be edited
            2 => Some(Self::Number),
            3 => Some(Self::Name),
            4 => Some(Self::Target),
            5 => Some(Self::PreWait),
            6 => Some(Self::Duration),
            7 => Some(Self::PostWait),
            8 => Some(Self::Follow),
            // There are no columns above 8
            1_usize | 9_usize.. => None,
        }
    }
}
pub trait CueVals: HasCueBase {
    /// Returns cue in table row format for Ratatui
    fn get_row(&self) -> Row<'_>;
    //
    // /// Play functionality for a cue. Note this is not just
    // /// "Play" but rather what happens when Space is pressed
    // fn play_cue(&self) -> color_eyre::Result<()>;

    /// Get Cue Type
    fn cue_type(&self) -> CueType;

    /// Get Name (Label) of a cue
    fn name(&self) -> String;

    /// Set Name (Label) of a cue
    fn set_name(&mut self, name: String);

    /// Get Number of a cue
    fn number(&self) -> f32;

    /// Set Number of a cue
    fn set_number(&mut self, number: f32);

    /// Get Follow State of a cue
    fn follow_state(&self) -> String;

    /// Set Follow State of a cue
    fn set_follow_state(&mut self, state: FollowState);

    /// Get (Playing Status) of a cue
    fn status(&self) -> PlayState;

    /// Set (Playing Status) of a cue
    fn set_status(&mut self, icon: PlayState);
}
pub trait Cue: CueVals {
    /// # Errors
    ///
    /// Will return `Err` if cue cannot be played
    fn play_cue(&self) -> color_eyre::Result<()> {
        Ok(())
    }

    fn pause_cue(&self) -> Option<bool> {
        None
    }

    fn stop_cue(&self) -> Option<bool> {
        None
    }

    #[allow(unused_variables)]
    fn fade_stop_cue(&self, time: f32) -> Option<bool> {
        None
    }

    fn target(&self) -> Option<f32> {
        None
    }

    #[allow(unused_variables)]
    fn set_target(&mut self, target: f32) {}
}

// Implement for all types where T ==(has the trait)=> Cue
impl<T: HasCueBase> CueVals for T {
    fn get_row(&self) -> Row<'_> {
        Row::new(vec![
            self.base().status.icon().to_string(),
            self.base().cue_type.icon().to_string(),
            self.base().number.to_string(),
            self.base().name.clone(),
            self.base().prewait.to_string(),
            self.base().duration.to_string(),
            self.base().postwait.to_string(),
            self.base().follow.icon().to_string(),
        ])
    }

    fn cue_type(&self) -> CueType {
        self.base().cue_type
    }

    fn name(&self) -> String {
        self.base().name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.set_base().name = name;
    }

    fn number(&self) -> f32 {
        self.base().number
    }

    fn set_number(&mut self, number: f32) {
        self.set_base().number = number;
    }

    fn follow_state(&self) -> String {
        match self.base().follow {
            FollowState::Follow => String::from("F"),
            FollowState::Continue => String::from("C"),
            FollowState::None => String::new(),
        }
    }

    fn set_follow_state(&mut self, state: FollowState) {
        self.set_base().follow = state;
    }

    fn status(&self) -> PlayState {
        self.base().status
    }

    fn set_status(&mut self, status: PlayState) {
        self.set_base().status = status;
    }
}
