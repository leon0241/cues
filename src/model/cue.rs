use ratatui::widgets::Row;
use rodio::{Decoder, Player, Source};

use std::{cmp::max, fs::File, io::Cursor, sync::{Arc, atomic::{AtomicU64, Ordering}}};

use crate::model::audio_config::AudioDevice;

#[allow(dead_code)]
// #[derive(Debug, Default)]
pub struct Cue {
    name: String,
    number: i32,
    cue_type: CueType,
    duration: i32,
    follow: FollowState,
    // colour: Option<String>,
    file: Option<AudioFile>,
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

#[allow(dead_code)]
// #[derive(Debug)]
pub struct AudioFile {
    name: String,
    file_path: String,
    bytes: Arc<[u8]>,
    player: Arc<Player>,
    now_playing: Arc<AtomicU64>,
}

impl AudioFile {
    pub fn new(name: String, file_path: String, mixer: &AudioDevice) -> color_eyre::Result<Self> {
        let bytes = std::fs::read(&file_path)?.into();

        let player: Arc<Player> = Arc::new(Player::connect_new(mixer.sink.mixer()));

        Ok(Self {
            name,
            file_path,
            bytes,
            player,
            now_playing: Arc::new(AtomicU64::new(0))
        })
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn get_player_arc(&self) -> Arc<Player> {
        Arc::clone(&self.player)
    }

    pub fn now_playing(&self) -> Arc<AtomicU64> {
        Arc::clone(&self.now_playing)
    }

    pub fn play_file(&self) {

        // Decode that sound file into a source
        // let source = Decoder::try_from(self.file.try_clone().expect("file not available")).unwrap();
        let source: Decoder<Cursor<Arc<[u8]>>> = Decoder::try_from(Cursor::new(self.bytes.clone())).unwrap();

        self.player.append(source);
    }


}

impl Cue {
    pub fn new(number: i32, name: String, cue_type: CueType, duration: i32, follow: FollowState, file: Option<AudioFile>) -> Cue {
        Cue {
            number,
            name,
            duration,
            follow,
            cue_type,
            // colour: None,
            file
        }
    }

    pub fn get_row(&self) -> Row<'_> {
        Row::new(vec![
            self.number.to_string(),
            self.name.clone(),
            self.duration.to_string()
        ])
    }

    pub fn get_type(&self) -> CueType {
        self.cue_type
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn player(&self) -> &Player {
        self.file.as_ref().unwrap().get_player()
    }

    // Async capable reference
    pub fn player_arc(&self) -> Arc<Player> {
        self.file.as_ref().unwrap().get_player_arc()
    }

    // Async capable reference
    pub fn now_playing(&self) -> Arc<AtomicU64> {
        self.file.as_ref().unwrap().now_playing()
    }

    pub fn play_cue(&self) -> color_eyre::Result<()> {
        // Arc reference to now_playing. Can be read for parallel changes during a fade
        let now_playing: Arc<AtomicU64> = self.now_playing();

        if self.player().empty() && let Some(ref i) = self.file {
            i.play_file()
        }
        Ok(())
    }

    pub fn pause(&self) {
        if self.player().is_paused() {
            self.player().play()
        } else {
            self.player().pause();
        }
    }

    pub fn stop(&self) {
        self.player().stop();
    }

    pub fn fade_stop(&self, time: f32) {
        // Arc reference to the player (so that open thread can continue after function)
        let player: Arc<Player> = self.player_arc();

        // Arc reference to now_playing. Can be read for parallel changes during a fade
        let now_playing: Arc<AtomicU64> = self.now_playing();

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
    }


    fn follow_state_symbol(&self) -> String {
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
}
