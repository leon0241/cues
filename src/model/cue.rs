use ratatui::widgets::Row;
use rodio::{Decoder, Player};

use std::{fs::File, io::Cursor, sync::Arc};

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
    player: Player
}

impl AudioFile {
    pub fn new(name: String, file_path: String, mixer: &AudioDevice) -> color_eyre::Result<Self> {
        let bytes = std::fs::read(&file_path)?.into();

        let player = Player::connect_new(mixer.sink.mixer());

        Ok(Self {
            name,
            file_path,
            bytes,
            player
        })
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn play_file(&self) {

        // Decode that sound file into a source
        // let source = Decoder::try_from(self.file.try_clone().expect("file not available")).unwrap();
        let source = Decoder::try_from(Cursor::new(self.bytes.clone())).unwrap();

        self.player.append(source);
        // self.controller.add(source);
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

    pub fn play_cue(&self) -> color_eyre::Result<()> {
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

    pub fn fade_stop(&self) {
        let vol = self.player().volume();

        for t in (0..100).rev() {
            println!("test");
            self.player().set_volume(vol / 100_f32 * t as f32);
            std::thread::sleep(std::time::Duration::from_millis(30));
        }

        self.player().stop();
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
