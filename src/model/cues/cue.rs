use std::sync::{Arc, atomic::{AtomicU64, Ordering}};

use crate::model::cues::{audio_file::AudioFile, cue_base::{CueType, FollowState}};
use crate::model::cues::cue_base::{HasCueBase};

use ratatui::widgets::Row;
use rodio::Player;

#[derive(Debug, PartialEq, Eq)]
pub enum CueColumn {
    Playing,
    Name,
    Number,
    Duration,
    Follow
}
pub trait CueVals {
    /// Returns cue in table row format for Ratatui
    fn get_row(&self) -> Row<'_> ;
    //
    // /// Play functionality for a cue. Note this is not just
    // /// "Play" but rather what happens when Space is pressed
    // fn play_cue(&self) -> color_eyre::Result<()>;

    /// Get Cue Type
    fn get_type(&self) -> CueType;

    /// Get Name (Label) of a cue
    fn get_name(&self) -> String;

    /// Set Name (Label) of a cue
    fn set_name(&mut self, name: String);

    /// Get Number of a cue
    fn get_number(&self) -> f32;

    /// Set Number of a cue
    fn set_number(&mut self, number: f32);

    /// Get Follow State of a cue
    fn get_follow_state(&self) -> String;

    /// Set Follow State of a cue
    fn set_follow_state(&self) -> String;

    /// Get (Playing Status) icon of a cue
    fn get_icon(&self) -> &String;

    /// Set (Playing Status) icon of a cue
    fn set_icon(&mut self);

    /// Edit value of a cell
    fn edit_cell_value(&mut self, column: CueColumn, value: String);
}
pub trait Cue: CueVals {
    fn play_cue(&self) -> color_eyre::Result<()> {
        Ok(())
    }

    fn pause_cue(&self) -> Option<bool> {
        None
    }

    fn stop_cue(&self) -> Option<bool> {
        None
    }

    fn fade_stop_cue(&self, time: f32) -> Option<bool> {
        None
    }

    fn get_target(&self) -> Option<f32> {
        None
    }

    fn set_target(&mut self, target: f32) { }
}

// Implement for all types where T ==(has the trait)=> Cue
impl <T: HasCueBase> CueVals for T {
    fn get_row(&self) -> Row<'_>  {
        Row::new(vec![
            self.get_base().icon.clone(),
            self.get_base().number.to_string(),
            self.get_base().name.clone(),
            self.get_base().duration.to_string()
        ])
    }

    fn get_type(&self) -> CueType { self.get_base().cue_type }

    fn get_name(&self) -> String { self.get_base().name.clone() }

    fn set_name(&mut self, name: String) { self.set_base().name = name }

    fn get_number(&self) -> f32 { self.get_base().number }

    fn set_number(&mut self, number: f32) { self.set_base().number = number }

    fn get_follow_state(&self) -> String {
        match self.get_base().follow {
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

    fn set_follow_state(&self) -> String {
        todo!()
    }

    fn get_icon(&self) -> &String {
        todo!()
    }

    fn set_icon(&mut self) {
        todo!()
    }

    fn edit_cell_value(&mut self, column: CueColumn, value: String) {
        if column == CueColumn::Playing {
            self.set_base().icon = value
        };
    }
}
