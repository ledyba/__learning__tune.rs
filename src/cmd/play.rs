mod average;

use std::rc::Rc;
use log::{debug, info};
use crate::player::{Player, Tuner};

pub const TUNES: [&'static str; 4] = [
  "average",
  "pythagoras",
  "lydian",
  "just",
];

fn make_tuner(tune_name: &str) -> anyhow::Result<Rc<dyn Tuner>> {
  match tune_name {
    "average" => Ok(Rc::new(average::Average::new())),
    name => {
      let msg = format!("Unknown name: {}", name);
      Err(anyhow::Error::msg(msg))
    },
  }
}

pub fn run(tune_name: &str, file_name: &str) -> anyhow::Result<()> {
  let file_bytes = std::fs::read(&file_name)?;
  let mid = {
    info!("Parsing: \"{}\"", &file_name);
    let mid = midly::Smf::parse(&file_bytes)?;
    debug!("  - Timing: {:?}", mid.header.format);
    debug!("  - Format: {:?}", mid.header.timing);
    debug!("  - {} tracks", mid.tracks.len());
    for (idx, track) in (0..mid.tracks.len()).zip(&mid.tracks) {
      debug!("    - Track[{}]: {} events", idx, track.len());
    }
    mid
  };
  info!("Playing \"{}\" using \"{}\" tuning.", file_name, tune_name);
  let tuner: Rc<dyn Tuner> = make_tuner(tune_name)?;
  let player = Player::new(tuner);
  player.play(&mid, file_name)?;
  Ok(())
}
