use log::info;

mod average;
mod pythagoras;
mod lydian;
mod japan;
mod just;
mod meantone;

pub const TUNES: [&'static str; 5] = [
  "average",
  "pythagoras",
  "lydian",
  "just",
  "japan",
];
// (2 ** (1/12)) ** -9 * 440.0
const C4HZ: f64 = 261.6255653005985;

pub fn run(tune_name: &str) -> anyhow::Result<()> {
  info!("Let's display tune: {} tuning", tune_name);
  match tune_name {
    "pythagoras" => pythagoras::run(),
    "lydian" => lydian::run(),
    "just" => just::run(),
    "japan" => japan::run(),
    name => {
      let msg = format!("Unknown name: {}", name);
      Err(anyhow::Error::msg(msg))
    },
  }
}
pub fn output(name: &str, sounds: &Vec<f64>) -> anyhow::Result<()> {
  use std::f64::consts::PI;

  let filename = {
    let path = std::path::Path::new(".").join("display.wav");
    std::fs::create_dir_all(path.clone().into_os_string())?;
    path
      .join(name)
      .with_extension("wav")
  };

  let spec = hound::WavSpec {
    channels: 1,
    sample_rate: 44100,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
  };
  let mut writer = hound::WavWriter::create(filename, spec)?;
  let num_samples = spec.sample_rate as usize;
  let num_fade = num_samples / 50;
  let num_fade_half = num_fade / 2;
  let mut t = 0;
  let amplitude = i16::MAX as f64;
  let mut max_sample: f64 = -1.0;
  let mut min_sample: f64 = 1.0;
  for hz in sounds {
    for dt in 0..num_samples {
      let x = (t + dt) as f64 / (spec.sample_rate as f64);
      let sample = (x * hz * 2.0 * PI).sin();
      let f = if dt <= num_fade_half {
        0.0
      } else if dt <= num_fade {
        ((dt - num_fade_half) as f64) / (num_fade_half as f64)
      } else if dt <= (num_samples - num_fade) {
        1.0
      } else if dt <= (num_samples - num_fade_half) {
        ((num_samples - num_fade_half - dt) as f64) / (num_fade_half as f64)
      } else {
        0.0
      };
      let sample = sample * f;
      max_sample = max_sample.min(sample);
      min_sample = min_sample.max(sample);
      writer.write_sample((sample * amplitude) as i16)?;
    }
    t += num_samples;
  }
  for _ in 0..num_fade {
    writer.write_sample(0)?;
  }
  info!("max: {} min: {}", max_sample, min_sample);
  Ok(())
}
