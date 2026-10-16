use color_eyre::eyre::eyre;

use crate::model::{
    cues::{Cue, cue_base::PlayState},
    cuestack::CueStack,
    keymaps::PlaybackCmd,
};

impl CueStack {
    pub(crate) fn playback_action(
        &mut self,
        i: usize,
        selection: PlaybackCmd,
    ) -> color_eyre::Result<()> {
        match selection {
            PlaybackCmd::PlayCue => {
                self.play_cue(i)?;
            }
            PlaybackCmd::PauseCue => {
                self.pause_cue(i)?;
            }
            PlaybackCmd::StopCue => {
                // If Stop is implemented
                self.stop_cue(i)?;
            }
            PlaybackCmd::FadeStopCue => {
                self.fade_stop_cue(i)?;
            }
        }
        Ok(())
    }

    fn get_cue(&mut self, i: usize) -> color_eyre::Result<&mut Box<dyn Cue>> {
        self.cues
            .get_mut(i)
            .ok_or_else(|| eyre!("index out of range"))
    }

    fn play_cue(&mut self, i: usize) -> color_eyre::Result<()> {
        let cue = self.get_cue(i)?;

        cue.play_cue()?;
        cue.set_status(PlayState::Playing);
        Ok(())
    }

    fn pause_cue(&mut self, i: usize) -> color_eyre::Result<()> {
        let cue = self.get_cue(i)?;

        // If pause is implemented
        match cue.pause_cue() {
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
        Ok(())
    }

    fn stop_cue(&mut self, i: usize) -> color_eyre::Result<()> {
        let cue = self.get_cue(i)?;

        if let Some(_i) = cue.stop_cue() {
            cue.set_status(PlayState::Stopped);
        }
        Ok(())
    }

    fn fade_stop_cue(&mut self, i: usize) -> color_eyre::Result<()> {
        let cue = self.get_cue(i)?;

        // If F+S is implemented
        if let Some(_i) = cue.fade_stop_cue(3_f32) {
            cue.set_status(PlayState::Fading);
        }
        Ok(())
    }
}
