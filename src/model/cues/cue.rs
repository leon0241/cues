use std::sync::{Arc, atomic::{AtomicU64, Ordering}};

use ratatui::widgets::Row;
use rodio::Player;

#[derive(Debug, PartialEq, Eq)]
pub enum CueColumn {
    Playing,
    Name,
    Number,
    Duration,
    Follow
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum FollowState {
    #[default]
    None,
    Follow,
    Continue
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum CueType {
    #[default]
    Audio,
    Start,
    Stop,
    Pause,
    Fade,
    Network,
    Group
}
pub trait Cue {
    /// Returns cue in table row format for Ratatui
    fn get_row(&self) -> Row<'_> ;

    /// Play functionality for a cue. Note this is not just
    /// "Play" but rather what happens when Space is pressed
    fn play_cue(&self) -> color_eyre::Result<()>;

    /// Get Cue Type
    fn get_type(&self) -> CueType;

    /// Get Name (Label) of a cue
    fn get_name(&self) -> String;

    /// Set Name (Label) of a cue
    fn set_name(&mut self, name: String);

    /// Get Number of a cue
    fn get_number(&self) -> f32;

    /// Set Number of a cue
    fn set_number(&mut self, number: f32);

    /// Get Follow State of a cue
    fn get_follow_state(&self) -> String;

    /// Set Follow State of a cue
    fn set_follow_state(&self) -> String;

    /// Get (Playing Status) icon of a cue
    fn get_icon(&self) -> &String;

    /// Set (Playing Status) icon of a cue
    fn set_icon(&mut self);

    /// Edit value of a cell
    fn edit_cell_value(&mut self, column: CueColumn, value: String);


    // Target Methods

    /// Pause currently playing Cue
    /// Pauses a cue. Does nothing if there is no cue stored or if no cue is playing.
    fn pause_cue(&self) -> Option<bool>;
    /// Stop currently playing Cue
    fn stop_cue(&self) -> Option<bool>;

    /// Fade and stop currently playing Cue.
    fn fade_stop_cue(&self, time: f32) -> Option<bool>;

    // Modifier Methods

    /// Get Cue Target
    fn get_target(&self) -> Option<f32>;

    /// Set Cue Target
    fn set_target(&mut self, target: f32);
}

// Implement for all types where T ==(has the trait)=> Cue
impl <T: Cue> Cue for T {
    fn get_row(&self) -> Row<'_>  {
        Row::new(vec![
            self.icon.clone(),
            self.number.to_string(),
            self.name.clone(),
            self.duration.to_string()
        ])
    }

    fn play_cue(&self) -> color_eyre::Result<()> {
        // Arc reference to now_playing. Can be read for parallel changes during a fade
        let now_playing: Arc<AtomicU64> = self.now_playing().unwrap();

        if self.player().empty() && let Some(ref i) = self.file {
            now_playing.store(1, Ordering::Release);
            i.play_file()
        }
        Ok(())
    }

    fn get_type(&self) -> CueType { self.cue_type }

    fn get_name(&self) -> String { self.name.clone() }

    fn set_name(&mut self, name: String) { self.name = name }

    fn get_number(&self) -> f32 { self.number }

    fn set_number(&mut self, number: f32) { self.number = number }

    fn get_follow_state(&self) -> String {
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

    fn set_follow_state(&self) -> String {
        todo!()
    }

    fn get_icon(&self) -> &String {
        todo!()
    }

    fn set_icon(&mut self) {
        todo!()
    }

    fn edit_cell_value(&mut self, column: CueColumn, value: String) {
        if column == CueColumn::Playing {
            self.icon = value
        };
    }

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

    fn get_target(&self) -> Option<f32> {
        todo!()
    }

    fn set_target(&mut self, target: f32) {
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
