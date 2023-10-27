mod raw_source;
mod tuner;

use std::collections::HashMap;
use std::path::Path;
use log::{debug, info, warn};
use midly::{MetaMessage, MidiMessage, Track, TrackEvent, TrackEventKind};

pub use raw_source::RawSource;
pub use tuner::Tuner;

/// A4 midi code
pub const A4: u8 = 69;

pub struct Player {
  tuner: Box<dyn Tuner>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Note {
  velocity: u8,
}

#[derive(Debug, Eq, PartialEq)]
struct TrackPlayer<'a> {
  track: &'a Track<'a>,
  consumed_idx: usize,
  next_event_time: usize,
  current_notes: HashMap<u8, Note>
}

impl Player {
  pub fn new(tuner: Box<dyn Tuner>) -> Self {
    Self {
      tuner,
    }
  }

  pub fn play<P: AsRef<Path>>(&self, mid: &midly::Smf, path: P) -> anyhow::Result<()> {
    let mut clock = 0;
    let mut tracks = Vec::from_iter(
      mid.tracks.iter().map(|track| {
        TrackPlayer {
          track,
          consumed_idx: 0,
          next_event_time: 0,
          current_notes: HashMap::new(),
        }
      })
    );
    while !tracks.iter().all(|state| state.done()) {
      for state in &mut tracks {
        state.process(clock);
      }
      clock += 1;
    }
    Ok(())
  }
}

impl <'a> TrackPlayer<'a> {
  fn done(&self) -> bool {
    self.consumed_idx >= self.track.len()
  }
  fn process(&mut self, clock: usize) {
    let track = self.track;
    if self.done() || clock < self.next_event_time {
      return;
    }
    let notes = &mut self.current_notes;
    let e= &track[self.consumed_idx];
    self.consumed_idx += 1;
    self.next_event_time += e.delta.as_int() as usize;
    match e.kind {
      TrackEventKind::Midi { channel, message } => {
        match message {
          MidiMessage::NoteOff { key, vel } => {
            //debug!("Note off: {}, {}", key, vel);
            let r = notes.remove(&key.as_int());
            if r.is_none() {
              warn!("Missing note off: {}, {}", key, vel);
            }
          },
          MidiMessage::NoteOn { key, vel } => {
            //debug!("Note on : {}, {}", key, vel);
            notes.insert(key.as_int(), Note {
              velocity: vel.as_int(),
            });
          },
          MidiMessage::Aftertouch { key, vel } => {
            if let Some(note) = notes.get_mut(&key.as_int()) {
              note.velocity = vel.as_int();
            }
          },
          MidiMessage::Controller { .. } => {},
          MidiMessage::ProgramChange { .. } => {},
          MidiMessage::ChannelAftertouch { .. } => {},
          MidiMessage::PitchBend { .. } => {},
        }
      },
      TrackEventKind::SysEx(_) => {},
      TrackEventKind::Escape(_) => {},
      TrackEventKind::Meta(meta) => {
        match meta {
          MetaMessage::TrackNumber(num) => {
            info!("TrackNumber: {:?}", num);
          }
          MetaMessage::Text(text) => {
            info!("Text: {}", String::from_utf8_lossy(text));
          }
          MetaMessage::Copyright(text) => {
            info!("Copyright: \n```\n{}\n```", String::from_utf8_lossy(text));
          }
          MetaMessage::TrackName(text) => {
            info!("TrackName: {}", String::from_utf8_lossy(text));
          }
          MetaMessage::InstrumentName(_) => {}
          MetaMessage::Lyric(_) => {}
          MetaMessage::Marker(_) => {}
          MetaMessage::CuePoint(_) => {}
          MetaMessage::ProgramName(_) => {}
          MetaMessage::DeviceName(_) => {}
          MetaMessage::MidiChannel(_) => {}
          MetaMessage::MidiPort(_) => {}
          MetaMessage::EndOfTrack => {}
          MetaMessage::Tempo(_) => {}
          MetaMessage::SmpteOffset(_) => {}
          MetaMessage::TimeSignature(_, _, _, _) => {}
          MetaMessage::KeySignature(_, _) => {}
          MetaMessage::SequencerSpecific(_) => {}
          MetaMessage::Unknown(_, _) => {}
        }
      },
    }
  }
}
