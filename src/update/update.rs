use crate::model::{
    cues::{
        audio_cue::AudioCue, cue::CueColumn, cue_base::{CueType, FollowState}, fade_cue::FadeCue
    }, cuestack::CueStack, editor::{Editor, Mode}
};

use crate::model::cues::audio_file::AudioFile;


/// # Errors
///
/// Will return `Err` if something went wrong in the files
pub fn init_cues(model: &mut CueStack) -> color_eyre::Result<()> {
    let file1: AudioFile = AudioFile::new(
        String::from("funkytown"),
        String::from("project/audio/test.wav"),
        &model.handler
    )?;
    let file2: AudioFile = AudioFile::new(
        String::from("boom"),
        String::from("project/audio/boom.wav"),
        &model.handler
    )?;

    model.set_items(
        vec![
            Box::new(AudioCue::new(1_f32, String::from("test"), 1_f32, FollowState::None, None)),
            Box::new(AudioCue::new(2_f32, String::from("file"), 1_f32, FollowState::None, Some(file1))),
            Box::new(AudioCue::new(2.5_f32, String::from("boom"), 1_f32, FollowState::None, Some(file2))),
            Box::new(FadeCue::new(3_f32, String::from("fade"), 1_f32, FollowState::None, Some(2_f32))),
            Box::new(AudioCue::new(3_f32, String::from("test2"), 1_f32, FollowState::None, None)),
        ]
    );

    Ok(())
}
