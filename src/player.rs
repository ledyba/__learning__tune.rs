use std::collections::HashSet;
use std::path::Path;
use log::{debug, info};
use midly::{MetaMessage, Track, TrackEvent, TrackEventKind};

///
/// Tuner maps midi key into frequency.
///
pub trait Tuner {
  fn freq(&self, code: u8) -> f64;
}
/// A4 midi code
pub const A4: u8 = 69;

pub struct Player {
  tuner: Box<dyn Tuner>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct EventState<'a> {
  event: &'a TrackEvent<'a>,
  from: usize,
  /// Exclusive
  until: usize,
}
#[derive(Debug, Eq, PartialEq)]
struct TrackState<'a> {
  track: &'a Track<'a>,
  done: bool,
  consumed: usize,
  current_events: HashSet<EventState<'a>>
}

impl Player {
  pub fn new(tuner: Box<dyn Tuner>) -> Self {
    Self {
      tuner,
    }
  }

  pub fn play<P: AsRef<Path>>(&self, mid: &midly::Smf, path: P) -> anyhow::Result<()> {
    let mut clock = 0;
    for i in 0..mid.tracks.len() {
      for e in &mid.tracks[i] {
        info!("[{}] {:?}", i, e);
      }
    }
    let mut tracks = Vec::from_iter(
      mid.tracks.iter().map(|track| {
        TrackState {
          track,
          done: false,
          consumed: 0,
          current_events: HashSet::new(),
        }
      })
    );
    for (track_idx, state) in (0..tracks.len()).zip(&mut tracks) {
      let track = state.track;
      let consumed = &mut state.consumed;
      if state.done || *consumed >= track.len() {
        continue;
      }
      let e= &track[*consumed];
    }
    Ok(())
  }
}
