use crate::model::{
    editor::{Mode},
    cuestack::{CueStack},
    cue::{Cue, FollowState, CueType}
};

use std::time::Duration;
use ratatui::crossterm::event::{self, Event, KeyCode};

use ringbuf::{
    traits::{Consumer, Producer, Split},
    HeapRb
};

use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::formats::TrackType;
use symphonia::core::formats::probe::Hint;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::audio::GenericAudioBufferRef;
use symphonia::core::audio::Audio;


use crate::model::cue::AudioFile;

#[derive(Debug)]
pub enum Message {
    Quit,
    PrevCue,
    NextCue,
    ModifyCue,
    NewCue,
    DeleteCue,
    PlayCue,
}

pub fn init_cues(model: &mut CueStack) {
    model.set_items(
        vec![
            Cue::new(1, String::from("test"), CueType::Audio, 1, FollowState::None),
            Cue::new(2, String::from("test2"), CueType::Audio, 1, FollowState::None),
        ]
    )
}

pub fn update(model: &mut CueStack, msg: Message) -> color_eyre::Result<(Mode, Option<Message>)> {
    // TODO: check all these guys for errors
    match msg {
        Message::PrevCue => {
            model.previous();
        }
        Message::NextCue => {
            model.next();
        }
        Message::ModifyCue => {
            model.update_cue();
        }
        Message::NewCue => {
            model.new_cue(Cue::new(
                    1,
                    String::from("test"),
                    CueType::Audio,
                    1,
                    FollowState::None
            ));
        }
        Message::DeleteCue => {
            model.delete_cue()?;
        }
        Message::Quit => {
            model.running_state = crate::model::editor::RunningState::Done
        }

        Message::PlayCue => {
            let cue_type = model.get_current_type()?;

            match cue_type {
                CueType::Audio => {
                    let audiofile = model.get_audio_file()?;
                    if let Some(i) = audiofile {
                        // play_audio(i);
                        play_audio();
                    }
                },
                CueType::Stop => {

                },
                // TODO: the rest of these cases
                _ => {  }
            };
        }
    }

    Ok((Mode::Insert, None))
}


// pub fn play_audio(audio_file: AudioFile) {
pub fn play_audio() {
    // Get the first command line argument.
    // let args: Vec<String> = std::env::args().collect();
    // let path = args.get(1).expect("file path not provided");

    // Open the media source.
    let src = std::fs::File::open("/mnt/data/Documents/git/cues/project/audio/test.wav").expect("failed to open media");

    println!("did it!");

    //Buffer limit can be detected automatically
    let (mut producer, mut receiver) = HeapRb::<f32>::new(512).split();
    let mut counter = 0;
    let mut audio_decoded_left = vec![];
    let mut audio_decoded_right = vec![];

    // Create the media source stream.
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    // Create a probe hint using the file's extension. [Optional]
    let mut hint = Hint::new();
    hint.with_extension("mp3");

    // // Use the default options for metadata and format readers.
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    // Probe the media source.
    let mut format = symphonia::default::get_probe()
        .probe(&hint, mss, fmt_opts, meta_opts)
        .expect("unsupported format");

    // Find the first audio track with a known (decodeable) codec.
    let track = format.default_track(TrackType::Audio).expect("no audio track");

    // Use the default options for the decoder.
    let dec_opts: AudioDecoderOptions = Default::default();

    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(
            track.codec_params.as_ref().expect("codec parameters missing").audio().unwrap(),
            &dec_opts,
        )
        .expect("unsupported codec");

    // Store the track identifier, it will be used to filter packets.
    let track_id = track.id;

    // The decode loop.
    loop {
        // Get the next packet from the media format.
        let packet = match format.next_packet() {
            Ok(Some(packet)) => packet,
            Ok(None) => {
                // Reached the end of the stream.
                break;
            }
            Err(Error::ResetRequired) => {
                // The track list has been changed. Re-examine it and create a new set of decoders,
                // then restart the decode loop. This is an advanced feature and it is not
                // unreasonable to consider this "the end." As of v0.5.0, the only usage of this is
                // for chained OGG physical streams.
                unimplemented!();
            }
            Err(err) => {
                // A unrecoverable error occurred, halt decoding.
                panic!("{}", err);
            }
        };

        // Consume any new metadata that has been read since the last packet.
        while !format.metadata().is_latest() {
            // Pop the old head of the metadata queue.
            format.metadata().pop();

            // Consume the new metadata at the head of the metadata queue.
        }

        // If the packet does not belong to the selected track, skip over it.
        if packet.track_id != track_id {
            continue;
        }

        // Decode the packet into audio samples.
        match decoder.decode(&packet) {
            Ok(decoded) => match decoded {
                GenericAudioBufferRef::F32(buf) => {
                    counter += 1;
                    println!("Size = {:#?}", buf.capacity());
                    if let (Some(l), Some(r)) = (buf.plane(0), buf.plane(1)) {
                        for (left, right) in l.iter().zip(r.iter()) {
 
                        audio_decoded_left.push(*left as f64);
                        audio_decoded_right.push(*right as f64);
                        }
                    }
                },
                _ => {
                    continue;
                }
            }
            Err(Error::IoError(_)) => {
                // The packet failed to decode due to an IO error, skip the packet.
                continue;
            }
            Err(Error::DecodeError(_)) => {
                // The packet failed to decode due to invalid data, skip the packet.
                continue;
            }
            Err(err) => {
                // An unrecoverable error occurred, halt decoding.
                panic!("{}", err);
            }
        }
    }

}


pub fn handle_event(_: &CueStack) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))?
        && let Event::Key(key) = event::read()?
        && key.kind == event::KeyEventKind::Press
    {
        return Ok(handle_key(key));
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('k') => Some(Message::PrevCue),
        KeyCode::Char('j') => Some(Message::NextCue),
        KeyCode::Char('i') => Some(Message::ModifyCue),
        KeyCode::Char('o') => Some(Message::NewCue),
        KeyCode::Char('d') => Some(Message::DeleteCue),
        KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}
