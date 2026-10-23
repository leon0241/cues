use crate::model::{
    cue::{Cue, CueType, PlayState, audio_cue::PlayableCue},
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

    fn get_cue(&mut self, i: usize) -> Option<&mut Cue> {
        self.cues.get_mut(i)
    }

    fn play_cue(&mut self, i: usize) -> color_eyre::Result<()> {
        let Some(cue) = self.get_cue(i) else {
            return Ok(());
        };

        if let CueType::Audio(audio_cue) = cue.cue_type() {
            audio_cue.play_cue()?;
            cue.set_status(PlayState::Playing);
        }
        Ok(())
    }

    fn pause_cue(&mut self, i: usize) {
        let Some(cue) = self.get_cue(i) else {
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

    fn stop_cue(&mut self, i: usize) {
        let Some(cue) = self.get_cue(i) else {
            return;
        };

        if let CueType::Audio(audio_cue) = cue.cue_type()
            && let Some(_i) = audio_cue.stop_cue()
        {
            cue.set_status(PlayState::Stopped);
        }
    }

    fn fade_stop_cue(&mut self, i: usize) {
        let Some(cue) = self.get_cue(i) else {
            return;
        };

        if let CueType::Audio(audio_cue) = cue.cue_type()
            && let Some(_i) = audio_cue.fade_stop_cue(3_f32)
        {
            cue.set_status(PlayState::Fading);
        }
    }
}
