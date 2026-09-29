use ratatui::{
    widgets::{ListState}
};

#[derive(Debug, Default)]
pub struct Cue {
    number: i32,
    pub name: String,
    duration: i32,
    follow: FollowState
}

#[derive(Debug, Default)]
pub struct CueStack {
    // cue_count: i16,
    pub cues: Vec<Cue>,
    pub current_cue: ListState,
    pub running_state: RunningState,
}

#[derive(Debug, Default, PartialEq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}


#[derive(Debug, Default, PartialEq, Eq)]
pub enum FollowState {
    #[default]
    None,
    Follow,
    Continue
}

impl Cue {
    pub fn new(number: i32, name: String, duration: i32, follow: FollowState) -> Cue {
        Cue {
            number,
            name,
            duration,
            follow
        }
    }
}

impl CueStack {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_items(&mut self, items: Vec<Cue>) {
        self.cues = items;
        // We reset the state as the associated items have changed. This effectively reset
        // the selection as well as the stored offset.
        self.current_cue = ListState::default();
    }

    // Select the next item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn next(&mut self) {
        let i = match self.current_cue.selected() {
            Some(i) => {
                if i >= self.cues.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.current_cue.select(Some(i));
    }

    // Select the previous item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn previous(&mut self) {
        let i = match self.current_cue.selected() {
            Some(i) => {
                if i == 0 {
                    self.cues.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.current_cue.select(Some(i));
    }

    // Unselect the currently selected item if any. The implementation of `ListState` makes
    // sure that the stored offset is also reset.
    pub fn unselect(&mut self) {
        self.current_cue.select(None);
    }

    pub fn update_cue(&self){
        unimplemented!();
    }

    pub fn new_cue(&mut self, new_cue: Cue){
        self.cues.push(new_cue)
    }

    pub fn delete_cue(&self){
        unimplemented!();
    }
}
