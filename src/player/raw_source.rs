use std::time::Duration;

pub struct RawSource {
  data: Vec<f32>,
  num_channels: u16,
  sample_rate: u32,
  current_pos: usize,
}

impl RawSource {
  pub fn new(data: Vec<f32>, num_channels: usize, sample_rate: usize) -> Self {
    Self {
      data,
      num_channels: num_channels as u16,
      sample_rate: sample_rate as u32,
      current_pos: 0,
    }
  }
}

impl Iterator for RawSource {
  type Item = f32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.current_pos < self.data.len() {
      let pos = self.current_pos;
      self.current_pos += 1;
      Some(self.data[pos])
    } else {
      None
    }
  }
}

impl rodio::source::Source for RawSource {
  fn current_frame_len(&self) -> Option<usize> {
    Some(self.data.len())
  }

  fn channels(&self) -> u16 {
    self.num_channels
  }

  fn sample_rate(&self) -> u32 {
    self.sample_rate
  }

  fn total_duration(&self) -> Option<Duration> {
    let num_samples = self.sample_rate as f64 * self.num_channels as f64;
    Some(Duration::from_secs_f64(self.data.len() as f64 / num_samples))
  }
}
