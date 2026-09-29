#[derive(Debug, Default)]
struct Cue {
    number: i32,
    name: String,
    duration: i32,
    follow: FollowState
}

#[derive(Debug, Default)]
pub struct CueStack {
    cue_count: i16,
    current_cue: i16,
    cues: [Cue; 5],
    pub running_state: RunningState,
}

#[derive(Debug, Default, PartialEq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}


#[derive(Debug, Default, PartialEq, Eq)]
enum FollowState {
    #[default]
    None,
    Follow,
    Continue
}

impl CueStack {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_cue(&self){
        unimplemented!();
    }

    pub fn new_cue(&self){
        unimplemented!();
    }

    pub fn delete_cue(&self){
        unimplemented!();
    }
}
