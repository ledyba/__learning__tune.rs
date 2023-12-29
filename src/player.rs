mod raw_source;
mod tuner;

use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use log::{info, warn};
use midly::{MetaMessage, MidiMessage, Track, TrackEventKind};

pub use raw_source::RawSource;
pub use tuner::Tuner;

/// A4 midi code
pub const A4: i32 = 69;

struct Sink {
  buffer: Vec<f32>,
}

impl Sink {
  fn new() -> Self {
    Self {
      buffer: Vec::new(),
    }
  }
  fn put(&mut self, time: usize, sample: f32) {
    if self.buffer.len() < time {
      self.buffer.resize(time + 1, 0.0);
    }
    self.buffer[time] += sample;
  }

  fn max(&self) -> f32 {
    let mut m = 0.0_f32;
    for sample in &self.buffer {
      m = m.max(sample.abs());
    }
    m
  }
}

pub struct Player {
  tuner: Rc<dyn Tuner>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Note {
  start_at: usize,
  key: u8,
  velocity: u8,
}

struct TrackPlayer<'a> {
  tuner: Rc<dyn Tuner>,
  track: &'a Track<'a>,
  current_idx: usize,
  next_event_time: usize,
  notes: HashMap<u8, Note>,
}

impl Player {
  pub fn new(tuner: Rc<dyn Tuner>) -> Self {
    Self {
      tuner,
    }
  }

  pub fn play<P: AsRef<Path>>(&self, mid: &midly::Smf, path: P) -> anyhow::Result<()> {
    let mut sink = Sink::new();
    let mut clock = 0;
    let mut track_players = Vec::from_iter(
      mid.tracks.iter().map(|track| {
        TrackPlayer {
          tuner: self.tuner.clone(),
          track,
          current_idx: 0,
          next_event_time: 0,
          notes: HashMap::new(),
        }
      })
    );
    while !track_players.iter().all(|state| state.done()) {
      for player in &mut track_players {
        player.process(clock, &mut sink);
      }
      clock += 1;
    }
    Ok(())
  }
}

impl <'a> TrackPlayer<'a> {
  fn done(&self) -> bool {
    self.current_idx >= self.track.len()
  }
  fn process(&mut self, clock: usize, sink: &mut Sink) {
    let track = self.track;
    if self.done() {
      return;
    }
    let notes = &mut self.notes;
    if clock >= self.next_event_time {
      let event = &track[self.current_idx];
      self.current_idx += 1;
      self.next_event_time += event.delta.as_int() as usize;
      match event.kind {
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
              let note = Note {
                start_at: clock,
                key: 0,
                velocity: vel.as_int(),
              };
              notes.insert(key.as_int(), note);
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
    let tuner = &self.tuner;
    for (key, note) in &self.notes {
      use std::f64::consts::PI;
      let freq = tuner.freq(*key);
      let t = (clock - note.start_at) as f64;
      let vel = note.velocity as f64 / 127.0;
      sink.put(clock, ((t * freq * 2.0 * PI).sin() * vel) as f32);
    }
  }
}
