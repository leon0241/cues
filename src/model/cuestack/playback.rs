//! # Cuestack Playback
//!
//! Implementation for any playback based functions within the cuestack

// TODO: these are all private methods so don't need docs but should do it anyways

use crate::model::{
    cue::{CueType, PlayState, audio_cue::PlayableCue},
    cuestack::CueStack,
    keymaps::PlaybackCmd,
};

impl CueStack {
    /// Performs an action on a specified cue index.
    ///
    /// This will only work for Audio Cues.
    ///
    /// # Errors
    ///
    /// This will return an error when trying to play a cue which cannot be played.
    pub(crate) fn playback_action(
        &mut self,
        i: usize,
        action: PlaybackCmd,
    ) -> color_eyre::Result<()> {
        match action {
            PlaybackCmd::PlayCue => {
                self.play_cue(i)?;
            }
            PlaybackCmd::PauseCue => {
                self.pause_cue(i);
            }
            PlaybackCmd::StopCue => {
                // If Stop is implemented
                self.stop_cue(i);
            }
            PlaybackCmd::FadeStopCue => {
                self.fade_stop_cue(i);
            }
        }
        Ok(())
    }

    /// Plays the Audio Cue at index i. This also updates the `PlayState` of the cue
    ///
    /// See `Cue::play_cue()` for more information.
    ///
    /// Returns if the cue does not exist, or if the cue is not an Audio cue
    fn play_cue(&mut self, i: usize) -> color_eyre::Result<()> {
        let Some(cue) = self.cues.get_mut(i) else {
            return Ok(());
        };

        if let CueType::Audio(audio_cue) = cue.cue_type() {
            audio_cue.play_cue()?;
            cue.set_status(PlayState::Playing);
        }
        Ok(())
    }

    /// Pauses the Audio Cue at index i. This also updates the `PlayState` of the cue
    ///
    /// See `Cue::pause_cue()` for more information.
    ///
    /// Returns if the cue does not exist, or if the cue is not an Audio cue
    fn pause_cue(&mut self, i: usize) {
        let Some(cue) = self.cues.get_mut(i) else {
            return;
        };

        if let CueType::Audio(audio_cue) = cue.cue_type() {
            match audio_cue.pause_cue() {
                // Unpaused -> Paused
                Some(true) => {
                    cue.set_status(PlayState::Stopped);
                }
                // Paused -> Unpaused
                Some(false) => {
                    cue.set_status(PlayState::Playing);
                }
                // Else (no file / not playing)
                _ => {}
            }
        } // If pause is implemented
    }

    /// Stops the Audio Cue at index i. This also updates the `PlayState` of the cue
    ///
    /// See `Cue::stop_cue()` for more information.
    ///
    /// Returns if the cue does not exist, or if the cue is not an Audio cue
    fn stop_cue(&mut self, i: usize) {
        let Some(cue) = self.cues.get_mut(i) else {
            return;
        };

        if let CueType::Audio(audio_cue) = cue.cue_type()
            && let Some(_i) = audio_cue.stop_cue()
        {
            cue.set_status(PlayState::Stopped);
        }
    }

    /// Fade and Stops the Audio Cue at index i. This also updates the `PlayState` of the cue
    ///
    /// See `Cue::fade_stop_cue()` for more information.
    ///
    /// Returns if the cue does not exist, or if the cue is not an Audio cue
    fn fade_stop_cue(&mut self, i: usize) {
        let Some(cue) = self.cues.get_mut(i) else {
            return;
        };

        if let CueType::Audio(audio_cue) = cue.cue_type()
            && let Some(_i) = audio_cue.fade_stop_cue(3_f32)
        {
            cue.set_status(PlayState::Fading);
        }
    }
}
