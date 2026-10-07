use std::sync::{Arc, atomic::{AtomicU64, Ordering}};

use num_traits::ToPrimitive;

use rodio::Player;

use crate::model::cues::{
    audio_file::AudioFile, cue::Cue, cue_base::{CueBase, CueType, FollowState, HasCueBase}
};

/// Audio Cue
#[derive(Default)]
pub struct AudioCue {
    /// Common Base
    base: CueBase,
    file: Option<AudioFile>,
}


#[allow(dead_code)]
/// Audio Cue Specific Functions
trait AudioCueFn: Cue {
    fn player(&self) -> Option<&Player>;

    fn player_arc(&self) -> Option<Arc<Player>>;

    fn now_playing(&self) -> Option<Arc<std::sync::atomic::AtomicU64>>;
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
    fn player(&self) -> Option<&Player> {
        self.file
            .as_ref()
            .map_or_default(
                |i| Some(i.get_player())
            )
    }

    /// Async capable reference to player
    fn player_arc(&self) -> Option<Arc<Player>> {
        self.file
            .as_ref()
            .map_or_default(
                |i| Some(i.get_player_arc())
            )
    }

    /// Async capable reference to status Atomic U64
    /// Returns None if there is no audio file
    fn now_playing(&self) -> Option<Arc<std::sync::atomic::AtomicU64>> {
        self.file
            .as_ref()
            .map_or_default(
                |i| Some(i.get_playing())
            )
    }
}

impl Cue for AudioCue {
    fn play_cue(&self) -> color_eyre::Result<()> {
        // Arc reference to now_playing. Can be read for parallel changes during a fade
        if 
            let Some(ref j) = self.file && // If there is a file
            let Some(i) = self.now_playing() {// If file is currently playing
            // self.player().empty() { this might not be redundant ngl
                i.store(1, Ordering::Release);
                j.play_file()?;
        }
        Ok(())
    }

    fn pause_cue(&self) -> Option<bool> {
        // Pauses if currently playing, plays if currently paused, and returns opposite.
        fn toggle_pause(player: &Player, paused: bool) -> bool {
            if paused { player.play() } else {player.pause()} !paused
        }

        // check If there is a file
        self.now_playing().map_or_default( 
            // there is a file (and therefore a AU64)
            |i: Arc<AtomicU64> | 
            // File is not playing -> None
            if i.load(Ordering::Acquire) == 0 {
                None
            // Else toggle play
            } else {
                let p = self.player()?;
                Some(toggle_pause(p, p.is_paused()))
            },
        )
    }

    fn stop_cue(&self) -> Option<bool> {
        // if not currently playing then no need to do anything
        self.now_playing().map_or_else(
            // default: there is no file, return true
            || Some(true),

            // there is a file (and therefore a AU64)
            |i: Arc<AtomicU64>|
            // 0 means not currently playing
            if i.load(Ordering::Acquire) == 0 {
                Some(false)
                    // Else stop
            } else {
                //Store 0 on now_playing (as operation has finished)
                i.store(0, Ordering::Release);
                //Stop player
                self.player_arc()?.stop();
                //Reset volume
                self.player_arc()?.set_volume(1.0);

                Some(true)
            }
        )
    }

    fn fade_stop_cue(&self, time: f32) -> Option<bool> {
        // Arc reference to the player (so that open thread can continue after function)
        let player: Arc<Player> = self.player_arc()?;

        // Arc reference to now_playing. Can be read for parallel changes during a fade
        let now_playing: Arc<AtomicU64> = self.now_playing()?;

        // Current volume
        let vol: f32 = player.volume();

        // Either 50 * step count, or 100 whichever is bigger.
        let count: f32 = (time * 50.0).max(100_f32);
        // Keep 100 steps for a small fade time, but ensure enough steps for a longer one
        let count_usize = (time * 50.0)
        .clamp(100.0, 10_000.0)
        .to_usize()
        .unwrap_or(100);


        // Number of steps over the time (time to sleep between steps)
        let dur = std::time::Duration::from_secs_f32(time / count);

        // Fetch-add one to the current ID and increase by one to match on this thread.
        // Subsequent threads increases the atomic, therefore losing eq on the original
        let now_playing_id = now_playing.fetch_add(1, Ordering::AcqRel).checked_add(1)?;

        // TODO: change this to be increments sub/add so that multiple fades can be done at once
        // Fade over a new thread
        std::thread::spawn(move || {
            // max > 0 for loop
            for t in (0..count_usize).rev() {
                // If ID is the same as atomic bool, nothing has overwritten it.
                if now_playing_id == now_playing.load(Ordering::Acquire) {
                    let t_f32 = t.to_f32()
                        .unwrap_or(f32::MAX);
                    player.set_volume(vol * t_f32 / count);
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
    #[must_use]
    pub const fn new(number: f32, name: String, duration: f32,
        follow: FollowState, file: Option<AudioFile>) -> Self {

        Self {
            base: CueBase::new(number, name, duration, follow,CueType::Audio),
            // colour: None,
            file,
        }
    }
}
