use num_traits::ToPrimitive;
use rodio::Player;
use std::sync::{
    Arc,
    atomic::{AtomicU16, Ordering},
};

use crate::audio_file::AudioFile;

/// Playable Cue Specific Functions.
///
/// This can possibly be extended to video in the future?
#[allow(dead_code)]
pub trait PlayableCue {
    /// Plays a Cue
    ///
    /// Plays a cue from the `AudioFile`'s provided file
    /// as bytes, using a Rodio Decoder.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the decoder cannot read the file
    fn play_cue(&self) -> color_eyre::Result<()> {
        Ok(())
    }

    /// Pauses a cue
    ///
    /// - Returns None if there is no file
    /// - Returns `Some(false)` when file is not currently playing
    /// - Returns `Some(true)` when file goes play -> pause
    /// - Returns `Some(false)` when file goes pause -> play
    fn pause_cue(&self) -> Option<bool> {
        None
    }

    /// Stops a cue
    ///
    /// - Returns None if there is no file
    /// - Returns `Some(true)` when file gets stopped
    /// - Returns `Some(false)` when file is not currently playing
    fn stop_cue(&self) -> Option<bool> {
        None
    }

    /// Fade and Stops a cue
    ///
    /// - Returns None if there is no file
    /// - Returns `Some(true)` when file gets stopped
    /// - Returns `Some(false)` when file is not currently playing
    #[allow(unused_variables)]
    fn fade_stop_cue(&self, time: f32) -> Option<bool> {
        None
    }
}

impl PlayableCue for AudioFile {
    fn play_cue(&self) -> color_eyre::Result<()> {
        // Arc reference to now_playing. Can be read for parallel changes during a fade
        self.now_playing.store(1, Ordering::Release);
        self.play_file()?;
        Ok(())
    }

    fn pause_cue(&self) -> Option<bool> {
        // Pauses if currently playing, plays if currently paused, and returns opposite.
        fn toggle_pause(player: &Player, paused: bool) -> bool {
            if paused {
                player.play();
            } else {
                player.pause();
            }
            !paused
        }

        if self.now_playing.load(Ordering::Acquire) == 0 {
            None
        // Else toggle play
        } else {
            let p: &Player = &self.player;
            Some(toggle_pause(p, p.is_paused()))
        }
    }

    fn stop_cue(&self) -> Option<bool> {
        // 0 means not currently playing
        if self.now_playing.load(Ordering::Acquire) == 0 {
            Some(false)
            // Else stop
        } else {
            //Store 0 on now_playing (as operation has finished)
            self.now_playing.store(0, Ordering::Release);
            //Stop player
            self.player.stop();
            //Reset volume
            self.player.set_volume(1.0);

            Some(true)
        }
    }

    fn fade_stop_cue(&self, time: f32) -> Option<bool> {
        // Arc reference to the player (so that open thread can continue after function)
        let player: Arc<Player> = Arc::clone(&self.player);

        // Arc reference to now_playing. Can be read for parallel changes during a fade
        let now_playing: Arc<AtomicU16> = Arc::clone(&self.now_playing);

        // Current volume
        let vol: f32 = player.volume();

        // Either 50 * step count, or 100 whichever is bigger.
        let count: f32 = (time * 50.0).max(100_f32);
        // Keep 100 steps for a small fade time, but ensure enough steps for a longer one
        let count_usize: usize = (time * 50.0)
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
                // Superseded by another call: it owns the player now, so do nothing
                if now_playing.load(Ordering::Acquire) != now_playing_id {
                    return;
                }

                let t_f32 = t.to_f32().unwrap_or(f32::MAX);

                player.set_volume(vol * t_f32 / count);
                std::thread::sleep(dur);
            }
            // Only finish up if we still own the ID (nothing took over during the last sleep)
            if now_playing
                .compare_exchange(now_playing_id, 0, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                player.stop();
                player.set_volume(1.0);
            }
        });

        Some(true)
    }
}
