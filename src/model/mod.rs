//! # Model
//!
//! Handles the model of the software
//! - Audio Config handles Rodio
//! - Cue handles individual cue types
//! - Cuestack handles cuestacks (lists of cues)
//! - Editor handles the editor
//! - Full Model combines Cuestack and Editor for ease
//! - Keymaps handles keyboard button mapping

pub mod full_model;

pub use full_model::Model;
