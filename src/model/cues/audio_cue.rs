use std::sync::{Arc, atomic::{AtomicU64, Ordering}};

use rodio::Player;

use crate::model::cues::{
    audio_file::AudioFile, cue::Cue, cue_base::{CueBase, CueType, FollowState, HasCueBase}
};

pub struct AudioCue {
    base: CueBase,
    file: Option<AudioFile>,
}

#[allow(dead_code)]
trait AudioCueFn: Cue {
    fn player(&self) -> &Player;

    fn player_arc(&self) -> Arc<Player>;

    fn now_playing(&self) -> Option<Arc<std::sync::atomic::AtomicU64>>;

    fn get_player(&self) -> String;
}


impl HasCueBase for AudioCue {
    fn get_base(&self) -> &CueBase {
        &self.base
    }

    fn set_base(&mut self) -> &mut CueBase {
        &mut self.base
    }
}

impl AudioCueFn for AudioCue {
    /// Absolute reference to player
    fn player(&self) -> &Player {
        self.file.as_ref().unwrap().get_player()
    }

    /// Async capable reference to player
    fn player_arc(&self) -> Arc<Player> {
        self.file.as_ref().unwrap().get_player_arc()
    }

    /// Async capable reference to status Atomic U64
    fn now_playing(&self) -> Option<Arc<std::sync::atomic::AtomicU64>> {
        self.file.as_ref().map(|file| file.get_playing())
    }

    fn get_player(&self) -> String { String::from("test") }
}

impl Cue for AudioCue {
    fn play_cue(&self) -> color_eyre::Result<()> {
        // Arc reference to now_playing. Can be read for parallel changes during a fade
        let now_playing: Arc<AtomicU64> = self.now_playing().unwrap();

        if self.player().empty() && let Some(ref i) = self.file {
            now_playing.store(1, Ordering::Release);
            i.play_file()
        }
        Ok(())
    }

    fn pause_cue(&self) -> Option<bool> {
        todo!()
    }

    fn stop_cue(&self) -> Option<bool> {
        // if not currently playing then no need to do anything
        if self.now_playing().unwrap().load(Ordering::Acquire) == 0 {
            return Some(false)
        }

        //Store 0 on now_playing (as operation has finished)
        self.now_playing().unwrap().store(0, Ordering::Release);
        //Stop player
        self.player_arc().stop();
        //Reset volume
        self.player_arc().set_volume(1.0);

        Some(true)
    }

    fn fade_stop_cue(&self, time: f32) -> Option<bool> {
        // Arc reference to the player (so that open thread can continue after function)
        let player: Arc<Player> = self.player_arc();

        // Arc reference to now_playing. Can be read for parallel changes during a fade
        let now_playing: Arc<AtomicU64> = self.now_playing().unwrap();

        // Current volume
        let vol = player.volume();

        // Either 50 * step count, or 100 whichever is bigger.
        // Keep 100 steps for a small fade time, but ensure enough steps for a longer one
        let count = ((time * 50.0) as i32).max(100);

        // Number of steps over the time (time to sleep between steps)
        let dur = std::time::Duration::from_secs_f32(time / count as f32);

        // Fetch-add one to the current ID and increase by one to match on this thread.
        // Subsequent threads increases the atomic, therefore losing eq on the original
        let now_playing_id = now_playing.fetch_add(1, Ordering::AcqRel) + 1;

        // TODO: change this to be increments sub/add so that multiple fades can be done at once
        // Fade over a new thread
        std::thread::spawn(move || {
            // max > 0 for loop
            for t in (0..count).rev() {
                // If ID is the same as atomic bool, nothing has overwritten it.
                if now_playing_id == now_playing.load(Ordering::Acquire) {
                    player.set_volume(vol * t as f32 / count as f32);
                    std::thread::sleep(dur);
                }
                // If ID is different from the atomic bool, the function has been called again.
                // Stop cue
                else {
                    break;
                }
            }
            //Store 0 on now_playing (as operation has finished)
            now_playing.store(0, Ordering::Release);
            //Stop player
            player.stop();
            //Reset volume
            player.set_volume(1.0);
        });

        Some(true)
    }

}

impl AudioCue {
    pub fn new(number: f32, name: String, duration: f32,
        follow: FollowState, file: Option<AudioFile>) -> Self {

        Self {
            base: CueBase::new(number, name, duration, follow,CueType::Audio),
            // colour: None,
            file,
        }
    }
}
