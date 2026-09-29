use ratatui::widgets::Row;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Cue {
    pub name: String,
    number: i32,
    duration: i32,
    follow: FollowState,
    // colour: Option<String>,
    file: Option<AudioFile>
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum FollowState {
    #[default]
    None,
    Follow,
    Continue
}

#[derive(Debug, Default, PartialEq, Eq)]
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
    duration: i32,
}

impl Cue {
    pub fn new(number: i32, name: String, duration: i32, follow: FollowState) -> Cue {
        Cue {
            number,
            name,
            duration,
            follow,
            // colour: None,
            file: None
        }
    }

    pub fn get_row(&self) -> Row {
        Row::new(vec![
            self.number.to_string(),
            self.name.clone(),
            self.duration.to_string()
        ])
    }
}
