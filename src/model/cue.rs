use ratatui::widgets::Row;

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
#[derive(Debug, Clone)]
pub struct AudioFile {
    name: String,
    file_path: String,
    duration: i32,
}

impl Cue {
    pub fn new(number: i32, name: String, cue_type: CueType, duration: i32, follow: FollowState) -> Cue {
        Cue {
            number,
            name,
            duration,
            follow,
            cue_type,
            // colour: None,
            file: None
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

    pub fn get_audio_file(&self) -> Option<AudioFile> {
        self.file.clone()
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
