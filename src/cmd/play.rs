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

pub fn run(tune_name: &str, input: &str, output: &str) -> anyhow::Result<()> {
  let bytes = std::fs::read(input)?;
  let mid = {
    info!("Parsing: \"{}\"", input);
    let mid = midly::Smf::parse(&bytes)?;
    info!("  - Format: {:?}", mid.header.format);
    // https://amei.or.jp/midistandardcommittee/MIDI1.0.pdf
    // p.137
    info!("  - Timing: {:?}", mid.header.timing);
    info!("  - {} tracks", mid.tracks.len());
    for (idx, track) in (0..mid.tracks.len()).zip(&mid.tracks) {
      info!("    - Track[{}]: {} midi events", idx, track.len());
    }
    mid
  };
  info!("Playing \"{}\" -> \"{}\" using \"{}\" tuning.", input, output, tune_name);
  let tuner: Rc<dyn Tuner> = make_tuner(tune_name)?;
  let player = Player::new(tuner);
  player.play(&mid, output)?;
  Ok(())
}
