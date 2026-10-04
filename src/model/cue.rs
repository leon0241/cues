// use ratatui::widgets::Row;
// use rodio::{Player};
//
// use std::{sync::{Arc, atomic::{AtomicU64, Ordering}}};
//
// use crate::model::audio_file::AudioFile;
//
// #[allow(dead_code)]
// // #[derive(Debug, Default)]
// pub struct Cue {
//     name: String,
//     number: i32,
//     cue_type: CueType,
//     duration: i32,
//     follow: FollowState,
//     // colour: Option<String>,
//     file: Option<AudioFile>,
//     icon: String
// }
//
// #[allow(dead_code)]
// impl Cue {
//     pub fn new(number: i32, name: String, cue_type: CueType, duration: i32, follow: FollowState, file: Option<AudioFile>) -> Cue {
//         Cue {
//             number,
//             name,
//             duration,
//             follow,
//             cue_type,
//             // colour: None,
//             file,
//             icon: String::new()
//         }
//     }
//
//     /// Returns a table row for TUI
//     pub fn get_row(&self) -> Row<'_> {
//         Row::new(vec![
//             self.icon.clone(),
//             self.number.to_string(),
//             self.name.clone(),
//             self.duration.to_string()
//         ])
//     }
//
//     pub fn get_type(&self) -> CueType {
//         self.cue_type
//     }
//
//     pub fn get_name(&self) -> String {
//         self.name.clone()
//     }
//
//     /// Absolute reference to player
//     pub fn player(&self) -> &Player {
//         self.file.as_ref().unwrap().get_player()
//     }
//
//     /// Async capable reference to player
//     pub fn player_arc(&self) -> Arc<Player> {
//         self.file.as_ref().unwrap().get_player_arc()
//     }
//
//     /// Async capable reference to status Atomic U64
//     pub fn now_playing(&self) -> Option<Arc<AtomicU64>> {
//         self.file.as_ref().map(|file| file.get_playing())
//     }
//
//     /// Plays a cue. Does nothing if there is no cue stored.
//     pub fn play_cue(&self) -> color_eyre::Result<()> {
//         // Arc reference to now_playing. Can be read for parallel changes during a fade
//         let now_playing: Arc<AtomicU64> = self.now_playing().unwrap();
//
//         if self.player().empty() && let Some(ref i) = self.file {
//             now_playing.store(1, Ordering::Release);
//             i.play_file()
//         }
//         Ok(())
//     }
//
//     /// Pauses a cue. Does nothing if there is no cue stored or if no cue is playing.
//     pub fn pause(&self) -> bool {
//         // if not currently playing then pause shouldn't do anythin
//         if self.now_playing().unwrap().load(Ordering::Acquire) == 0 {
//             false
//         }
//         else if self.player().is_paused() {
//             self.player().play();
//             false
//         } else {
//             self.player().pause();
//             true
//         }
//     }
//
//     /// Stops a cue. Does nothing if there is no cue stored or if no cue is playing.
//     pub fn stop(&self) {
//         // if not currently playing then no need to do anything
//         if self.now_playing().unwrap().load(Ordering::Acquire) == 0 {
//             return
//         }
//
//         //Store 0 on now_playing (as operation has finished)
//         self.now_playing().unwrap().store(0, Ordering::Release);
//         //Stop player
//         self.player_arc().stop();
//         //Reset volume
//         self.player_arc().set_volume(1.0);
//     }
//
//     /// Fade + Stops a cue.
//     pub fn fade_stop(&self, time: f32) {
//         // Arc reference to the player (so that open thread can continue after function)
//         let player: Arc<Player> = self.player_arc();
//
//         // Arc reference to now_playing. Can be read for parallel changes during a fade
//         let now_playing: Arc<AtomicU64> = self.now_playing().unwrap();
//
//         // Current volume
//         let vol = player.volume();
//
//         // Either 50 * step count, or 100 whichever is bigger.
//         // Keep 100 steps for a small fade time, but ensure enough steps for a longer one
//         let count = ((time * 50.0) as i32).max(100);
//
//         // Number of steps over the time (time to sleep between steps)
//         let dur = std::time::Duration::from_secs_f32(time / count as f32);
//
//         // Fetch-add one to the current ID and increase by one to match on this thread.
//         // Subsequent threads increases the atomic, therefore losing eq on the original
//         let now_playing_id = now_playing.fetch_add(1, Ordering::AcqRel) + 1;
//
//         // TODO: change this to be increments sub/add so that multiple fades can be done at once
//         // Fade over a new thread
//         std::thread::spawn(move || {
//             // max > 0 for loop
//             for t in (0..count).rev() {
//                 // If ID is the same as atomic bool, nothing has overwritten it.
//                 if now_playing_id == now_playing.load(Ordering::Acquire) {
//                     player.set_volume(vol * t as f32 / count as f32);
//                     std::thread::sleep(dur);
//                 }
//                 // If ID is different from the atomic bool, the function has been called again.
//                 // Stop cue
//                 else {
//                     break;
//                 }
//             }
//             //Store 0 on now_playing (as operation has finished)
//             now_playing.store(0, Ordering::Release);
//             //Stop player
//             player.stop();
//             //Reset volume
//             player.set_volume(1.0);
//         });
//     }
//
//
//     fn follow_state_symbol(&self) -> String {
//         match self.follow {
//             FollowState::Follow => {
//                 String::from("F")
//             }
//             FollowState::Continue => {
//                 String::from("C")
//             }
//             FollowState::None => {
//                 String::from("")
//             }
//         }
//     }
//
//     pub fn edit_cell_value(&mut self, column: CueColumn, value: String) {
//         if column == CueColumn::Playing {
//             self.icon = value
//         };
//     }
//
//     pub fn get_icon(&self) -> &String {
//         &self.icon
//     }
//
//     pub fn set_icon(&mut self, new_icon: String) {
//         self.icon = new_icon
//     }
// }
