use std::sync::{Arc, atomic::{AtomicU64, Ordering}};

use ratatui::widgets::Row;
use rodio::Player;

use crate::model::cues::{audio_file::AudioFile, cue::{Cue, CueColumn, CueType, FollowState}};

pub struct AudioCue {
    name: String,
    number: i32,
    cue_type: CueType,
    duration: i32,
    follow: FollowState,
    // colour: Option<String>,
    file: Option<AudioFile>,
    icon: String
}

#[allow(dead_code)]
impl Cue for AudioCue {
    fn get_row(&self) -> ratatui::widgets::Row<'_>  {
        Row::new(vec![
            self.icon.clone(),
            self.number.to_string(),
            self.name.clone(),
            self.duration.to_string()
        ])
    }

    /// Plays a cue. Does nothing if there is no cue stored.
    fn play_cue(&self) -> color_eyre::Result<()> {
        // Arc reference to now_playing. Can be read for parallel changes during a fade
        let now_playing: Arc<AtomicU64> = self.now_playing().unwrap();

        if self.player().empty() && let Some(ref i) = self.file {
            now_playing.store(1, Ordering::Release);
            i.play_file()
        }
        Ok(())
    }


    fn get_type(&self) -> CueType {
        self.cue_type
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&self) -> String {
        todo!()
    }

    fn follow_state(&self) -> String {
        match self.follow {
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

    fn edit_cell_value(&mut self, column: CueColumn, value: String) {
        if column == CueColumn::Playing {
            self.icon = value
        };
    }

    fn get_icon(&self) -> &String {
        todo!()
    }

    fn set_icon(&self) {
        todo!()
    }

    /// Pauses a cue. Does nothing if there is no cue stored or if no cue is playing.
    fn pause_cue(&self) -> Option<bool> {
        // if not currently playing then pause shouldn't do anythin
        if self.now_playing().unwrap().load(Ordering::Acquire) == 0 {
            Some(false)
        }
        else if self.player().is_paused() {
            self.player().play();
            Some(false)
        } else {
            self.player().pause();
            Some(true)
        }
    }

    /// Stops a cue. Does nothing if there is no cue stored or if no cue is playing.
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

    /// Fade + Stops a cue.
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

    fn get_target(&self) -> Option<f32> { None }

    fn set_target(&self) -> Option<f32> { None }
}

impl AudioCue {
    pub fn new(number: i32, name: String, duration: i32,
        follow: FollowState, file: Option<AudioFile>) -> Self {

        Self {
            number,
            name,
            duration,
            follow,
            cue_type: CueType::Audio,
            // colour: None,
            file,
            icon: String::new(),
        }
    }

    /// Absolute reference to player
    pub fn player(&self) -> &Player {
        self.file.as_ref().unwrap().get_player()
    }

    /// Async capable reference to player
    pub fn player_arc(&self) -> Arc<Player> {
        self.file.as_ref().unwrap().get_player_arc()
    }

    /// Async capable reference to status Atomic U64
    pub fn now_playing(&self) -> Option<Arc<std::sync::atomic::AtomicU64>> {
        self.file.as_ref().map(|file| file.get_playing())
    }

}
