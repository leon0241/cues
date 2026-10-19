//! # Update
//!
//! Handles changes to items in the model
//! - Keymaps process ``Messages`` into changes in the system
//! - Event Handler sends ``Messages`` in accordance to keymaps assigned within Model/Keymaps

pub mod event_handler;
pub mod keymaps;
