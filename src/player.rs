mod raw_source;
mod tuner;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::rc::Rc;
use hound::WavWriter;
use log::{info, warn};
use midly::{MetaMessage, MidiMessage, Timing, Track, TrackEventKind};

pub use tuner::Tuner;

/// A4 midi code
pub const A4: i32 = 69;
pub const A4HZ: f64 = 440.0;

struct Sink {
  wav: WavWriter<BufWriter<File>>,
  current_sample: usize,
  ticks_per_note: usize,
  sample_per_tick: f64,
  next_tick_sample: f64,
}

impl Sink {
  fn new(
    wav: WavWriter<BufWriter<File>>,
    ticks_per_note: usize,
  ) -> Self {
    let sample_rate = wav.spec().sample_rate as f64;
    Self {
      wav,
      current_sample: 0,
      ticks_per_note,
      sample_per_tick: sample_rate / (ticks_per_note as f64),
      next_tick_sample: 0.0,
    }
  }

  fn put(&mut self, mut f: impl FnMut(&mut WavWriter<BufWriter<File>>, usize) -> anyhow::Result<()>) -> anyhow::Result<()> {
    while (self.current_sample as f64) < self.next_tick_sample {
      f(&mut self.wav, self.current_sample)?;
      self.current_sample += 1;
    }
    self.next_tick_sample += self.sample_per_tick;
    Ok(())
  }

  fn finalize(mut self) -> anyhow::Result<()> {
    self.wav.finalize()?;
    Ok(())
  }
}

pub struct Player {
  tuner: Rc<dyn Tuner>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct NoteState {
  start_at: usize,
  velocity: u8,
}

struct TrackPlayer<'a> {
  tuner: Rc<dyn Tuner>,
  track: &'a Track<'a>,
  current_idx: usize,
  next_event_tick: usize,
  current_notes: HashMap<u8, NoteState>,
  ticks_per_note: usize,
}

impl Player {
  pub fn new(tuner: Rc<dyn Tuner>) -> Self {
    Self {
      tuner,
    }
  }

  pub fn play<P: AsRef<Path>>(&self, mid: &midly::Smf, path: P) -> anyhow::Result<()> {
    let spec = hound::WavSpec {
      channels: 1,
      sample_rate: 44100,
      bits_per_sample: 16,
      sample_format: hound::SampleFormat::Int,
    };
    let wav = hound::WavWriter::create(path, spec)?;
    let mut ticks = 0;
    let ticks_per_note = match mid.header.timing {
      Timing::Metrical(ticks) => {
        ticks.as_int() as usize
      }
      Timing::Timecode(_, _) => todo!(),
    };
    let mut sink = Sink::new(wav, ticks_per_note);
    let mut track_players = Vec::from_iter(
      mid.tracks.iter().map(|track| {
        TrackPlayer {
          tuner: self.tuner.clone(),
          track,
          current_idx: 0,
          next_event_tick: 0,
          current_notes: HashMap::new(),
          ticks_per_note,
        }
      })
    );
    while !track_players.iter().all(|state| state.done()) {
      for player in &mut track_players {
        player.process(ticks, &mut sink)?;
      }
      ticks += 1;
    }
    sink.finalize()?;
    Ok(())
  }
}

impl <'a> TrackPlayer<'a> {
  fn done(&self) -> bool {
    self.current_idx >= self.track.len()
  }
  fn process(&mut self, ticks: usize, sink: &mut Sink) -> anyhow::Result<()> {
    let track = self.track;
    if self.done() {
      return Ok(());
    }
    let current_notes = &mut self.current_notes;
    if ticks >= self.next_event_tick {
      let event = &track[self.current_idx];
      self.current_idx += 1;
      self.next_event_tick += event.delta.as_int() as usize;
      match event.kind {
        TrackEventKind::Midi { channel, message } => {
          let channel = channel.as_int();
          match message {
            MidiMessage::NoteOff { key, vel } => {
              //debug!("Note off: {}, {}", key, vel);
              let r = current_notes.remove(&key.as_int());
              if r.is_none() {
                warn!("Missing note off: key={}, vel={}", key.as_int(), vel.as_int());
              }
            },
            MidiMessage::NoteOn { key, vel } => {
              //debug!("Note on : {}, {}", key, vel);
              let note = NoteState {
                start_at: ticks,
                velocity: vel.as_int(),
              };
              current_notes.insert(key.as_int(), note);
            },
            MidiMessage::Aftertouch { key, vel } => {
              if let Some(note) = current_notes.get_mut(&key.as_int()) {
                note.velocity = vel.as_int();
              } else {
                warn!("Note (vel={}) is not on!", key.as_int())
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
              let text = String::from_utf8_lossy(text)
                .lines()
                .map(|it| format!("  ##  {it}\n"))
                .fold(String::new(), |body, line| body + &line);
              info!("Copyright: \n{}", text);
            }
            MetaMessage::TrackName(text) => {
              info!("TrackName: {}", String::from_utf8_lossy(text));
            }
            MetaMessage::InstrumentName(name) => {
              info!("InstrumentName: {}", String::from_utf8_lossy(name));
            }
            MetaMessage::Lyric(lyric) => {
              info!("Lyric: {}", String::from_utf8_lossy(lyric));
            }
            MetaMessage::Marker(marker) => {
              info!("Marker: {}", String::from_utf8_lossy(marker));
            }
            MetaMessage::CuePoint(_) => {}
            MetaMessage::ProgramName(_) => {}
            MetaMessage::DeviceName(device_name) => {
              info!("DeviceName: {}", String::from_utf8_lossy(device_name));
            }
            MetaMessage::MidiChannel(_) => {}
            MetaMessage::MidiPort(_) => {}
            MetaMessage::EndOfTrack => {
              info!("End of Tradck.");
            }
            MetaMessage::Tempo(tempo) => {
              info!("Tempo: {}", tempo);
            }
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
    for (key, note) in &self.current_notes {
      use std::f64::consts::PI;
      let freq = tuner.freq(*key);
      let start_at = note.start_at as f64 * sink.sample_per_tick;
      let vel = note.velocity as f64 / 127.0;
      sink.put(|wav, current_sample| {
        let t = current_sample as f64 - start_at;
        wav.write_sample(((t * freq * 2.0 * PI).sin() * vel) as f32)?;
        Ok(())
      })?;
    }
    Ok(())
  }
}
