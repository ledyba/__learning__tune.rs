use hound;
use std::f64::consts::PI;
use log::info;

pub fn output(name: &str, sounds: &Vec<f64>) -> anyhow::Result<()> {
  let spec = hound::WavSpec {
    channels: 1,
    sample_rate: 44100,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
  };
  let mut writer = hound::WavWriter::create(format!("{}.wav", name), spec)?;
  let num_samples = spec.sample_rate as usize;
  let num_samples_near_last = spec.sample_rate as usize * 9 / 10;
  let num_samples_near_beg = spec.sample_rate as usize * 10;
  let mut t = 0;
  let amplitude = i16::MAX as f64;
  let mut max = 0.0;
  let mut min = 0.0;
  for hz in sounds {
    for dt in 0..num_samples {
      let x = (t + dt) as f64 / (spec.sample_rate as f64);
      let sample = (x * hz * 2.0 * PI).sin();
      let sample = if (dt <= num_samples_near_beg || num_samples_near_last <= dt) && sample.abs() < 0.01 {
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
