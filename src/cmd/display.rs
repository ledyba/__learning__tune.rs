use log::info;
use crate::player::RawSource;

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
  let mut wav = Vec::<f32>::new();
  let (_stream, handle) = rodio::OutputStream::try_default()?;
  let sink = rodio::Sink::try_new(&handle)?;
  let sample_rate = (48 * 1000) as usize;
  let samples_per_sound = sample_rate * 1;
  let num_fade = samples_per_sound / 100;
  let num_fade_half = num_fade / 2;
  let amp = 0.7;
  let mut t = 0;
  let mut max_sample: f32 = -1.0;
  let mut min_sample: f32 = 1.0;
  for _ in 0..num_fade {
    wav.push(0.0);
  }
  for hz in sounds {
    for dt in 0..samples_per_sound {
      let x = (t + dt) as f64 / (sample_rate as f64);
      let sample = (x * hz * 2.0 * PI).sin();
      let f = if dt <= num_fade_half {
        0.0
      } else if dt <= num_fade {
        ((dt - num_fade_half) as f64) / (num_fade_half as f64)
      } else if dt <= (samples_per_sound - num_fade) {
        1.0
      } else if dt <= (samples_per_sound - num_fade_half) {
        ((samples_per_sound - num_fade_half - dt) as f64) / (num_fade_half as f64)
      } else {
        0.0
      };
      let sample = (sample * f * amp) as f32;
      max_sample = max_sample.min(sample);
      min_sample = min_sample.max(sample);
      wav.push(sample);
    }
    t += samples_per_sound + num_fade;
  }
  for _ in 0..num_fade {
    wav.push(0.0);
  }
  info!("max: {} min: {}", max_sample, min_sample);
  sink.append(RawSource::new(wav, 1, sample_rate));
  //sink.append(rodio::source::SineWave::new(440.0));
  sink.sleep_until_end();
  Ok(())
}
