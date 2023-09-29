use log::{debug, info};

pub const TUNES: [&'static str; 4] = [
  "average",
  "pythagoras",
  "lydian",
  "just",
];

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
  Ok(())

}
