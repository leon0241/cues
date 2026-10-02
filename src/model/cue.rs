use ratatui::widgets::Row;

use std::fs::File;

#[allow(dead_code)]
#[derive(Debug, Default)]
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
#[derive(Debug)]
pub struct AudioFile {
    name: String,
    file_path: String,
}

impl AudioFile {
    pub fn new(name: String, file_path: String) -> Self{
        Self {
            name,
            file_path
        }
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

    pub fn get_audio_file(&self) -> Option<File> {
        match self.file {
            Some(ref i) => {
                let file_path = i.file_path.clone();
                Some(File::open(file_path).unwrap())
            },
            None => { None }
        }
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
