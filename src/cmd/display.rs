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

  let spec = hound::WavSpec {
    channels: 1,
    sample_rate: 44100,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
  };
  let mut writer = hound::WavWriter::create(format!("{}.wav", name), spec)?;
  let num_samples = spec.sample_rate as usize;
  let num_samples_near_last = num_samples * 99 / 100;
  let num_samples_near_beg = num_samples * 1 / 100;
  let mut t = 0;
  let amplitude = i16::MAX as f64;
  let mut max = 0.0;
  let mut min = 0.0;
  let mut silent = true;
  for hz in sounds {
    for dt in 0..num_samples {
      let x = (t + dt) as f64 / (spec.sample_rate as f64);
      let sample = (x * hz * 2.0 * PI).sin();
      if dt <= num_samples_near_beg {
        silent = true;
      }else if !silent && num_samples_near_last <= dt && sample.abs() < 0.01 {
        silent = true;
      }else if silent && (num_samples_near_beg <= dt && dt <= num_samples_near_last) && sample.abs() >= 0.01 {
        silent = false;
      }
      let sample = if silent {
        0.0
      } else {
        sample
      };
      max = sample.max(max);
      min = sample.min(min);
      writer.write_sample((sample * amplitude) as i16)?;
    }
    t += num_samples;
  }
  Ok(())
}
