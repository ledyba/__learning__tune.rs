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
const SAMPLE_RATE: usize = 48 * 1000;
const NUM_CHANNELS: usize = 1;

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
  let data = generate_sounds(sounds);
  save_to_file(name, &data)?;
  play(&data)?;
  Ok(())
}

fn generate_sounds(sounds: &Vec<f64>) -> Vec<f32> {
  use std::f64::consts::PI;
  let mut data = Vec::<f32>::new();
  let samples_per_sound = SAMPLE_RATE * NUM_CHANNELS;
  let num_fade = samples_per_sound / 100;
  let num_fade_half = num_fade / 2;
  let amp = 0.7;
  let mut t = 0;
  let mut max_sample: f32 = -1.0;
  let mut min_sample: f32 = 1.0;
  for _ in 0..num_fade {
    data.push(0.0);
  }
  for hz in sounds {
    for dt in 0..samples_per_sound {
      let x = (t + dt) as f64 / (SAMPLE_RATE as f64);
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
      data.push(sample);
    }
    t += samples_per_sound + num_fade;
  }
  for _ in 0..num_fade {
    data.push(0.0);
  }
  info!("max: {} min: {}", max_sample, min_sample);
  data
}

fn play(data: &[f32]) -> anyhow::Result<()> {
  let (_stream, handle) = rodio::OutputStream::try_default()?;
  let sink = rodio::Sink::try_new(&handle)?;
  sink.append(RawSource::new(Vec::from(data), 1, SAMPLE_RATE).into_iter());
  sink.sleep_until_end();
  Ok(())
}

fn save_to_file(name: &str, data: &[f32]) -> anyhow::Result<()> {
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
  for item in data {
    writer.write_sample(item * i16::MAX as f32)?;
  }
  writer.finalize()?;
  Ok(())
}
