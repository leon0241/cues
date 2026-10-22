//! # Audio Cue
//!
//! `CueType` structure for audio cues
//! Uses Rodio as a backend

use std::sync::{Arc, atomic::{AtomicU16, Ordering}};

use num_traits::ToPrimitive;

use rodio::Player;

use crate::model::cue::{
    audio_file::AudioFile
};

/// Audio Cue
///
/// Contains an `AudioFile` field which is what
/// the Rodio Player for the cue is implemented on.
#[derive(Debug, Default, Clone)]
pub struct AudioCue {
    file: Option<AudioFile>,
}


/// Audio Cue Specific Functions.
///
/// This can possibly be extended to video in the future?
#[allow(dead_code)]
impl AudioCue {
    /// Absolute reference to player
    ///
    /// This is a reference to the Rodio Player.
    fn player(&self) -> Option<&Player> {
        self.file
            .as_ref()
            .map_or_default(
                |i| Some(i.get_player())
            )
    }

    /// Async capable reference to player
    ///
    /// This returns an Arc Clone of the Player. Useful for
    /// when the reference needs to outlive the function
    /// such as a play cue function.
    ///
    /// Returns None if there is no audio file
    fn player_arc(&self) -> Option<Arc<Player>> {
        self.file
            .as_ref()
            .map_or_default(
                |i| Some(i.get_player_arc())
            )
    }

    /// Async capable reference to status Atomic U16
    ///
    /// This returns an Arc clone of the now playing variable
    /// of the file (an `AtomicU16`) which tracks ownership of
    /// a modifier. Useful for cues like fades which can override
    /// each other.
    ///
    /// Returns None if there is no audio file
    fn now_playing(&self) -> Option<Arc<std::sync::atomic::AtomicU16>> {
        self.file
            .as_ref()
            .map_or_default(
                |i| Some(i.get_playing())
            )
    }
}

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
    fn play_cue(&self) -> color_eyre::Result<()>;

    /// Pauses a cue
    ///
    /// - Returns None if there is no file
    /// - Returns `Some(false)` when file is not currently playing
    /// - Returns `Some(true)` when file goes play -> pause
    /// - Returns `Some(false)` when file goes pause -> play
    fn pause_cue(&self) -> Option<bool>;

    /// Stops a cue
    ///
    /// - Returns None if there is no file
    /// - Returns `Some(true)` when file gets stopped
    /// - Returns `Some(false)` when file is not currently playing
    fn stop_cue(&self) -> Option<bool>;

    /// Fade and Stops a cue
    ///
    /// - Returns None if there is no file
    /// - Returns `Some(true)` when file gets stopped
    /// - Returns `Some(false)` when file is not currently playing
    #[allow(unused_variables)]
    fn fade_stop_cue(&self, time: f32) -> Option<bool>;
}

impl PlayableCue for AudioCue {
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
            |i: Arc<AtomicU16> | 
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
            |i: Arc<AtomicU16>|
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
        let now_playing: Arc<AtomicU16> = self.now_playing()?;

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
                // Superseded by another call: it owns the player now, so do nothing
                if now_playing.load(Ordering::Acquire) != now_playing_id {
                    return;
                }

                let t_f32 = t.to_f32()
                    .unwrap_or(f32::MAX);

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

/// Other Methods
impl AudioCue {
    /// Creates an `AudioCue`
    ///
    /// To create an audio cue without a file use `AudioCue::default()`
    #[must_use]
    pub const fn new(file: Option<AudioFile>) -> Self {
        Self {
            file,
        }
    }

    /// Gets a reference to the `AudioFile`
    ///
    /// To get a mutable reference, use `file_mut()`
    #[must_use]
    pub const fn file(&self) -> &Option<AudioFile> {
        &self.file
    }

    /// Gets a mutable reference to the `AudioFile`
    ///
    /// To get an absolute reference, use `file()`
    #[must_use]
    pub const fn file_mut(&mut self) -> &mut Option<AudioFile> {
        &mut self.file
    }

    /// Returns a file's icon.
    ///
    /// This is solely for use in the Target field, where
    /// a populated audio file will have the "" icon while
    /// an unpopulated one will be blank.
    #[must_use]
    pub fn as_icon(&self) -> &'static str {
        self.file.as_ref().map_or("", |_| "")
    }
}
