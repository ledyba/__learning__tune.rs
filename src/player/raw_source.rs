use std::time::Duration;

pub struct RawSource {
  data: Vec<f32>,
  num_channels: u16,
  sample_rate: u32,
}

impl RawSource {
  pub fn new(data: Vec<f32>, num_channels: usize, sample_rate: usize) -> Self {
    Self {
      data,
      num_channels: num_channels as u16,
      sample_rate: sample_rate as u32,
    }
  }

  pub fn into_iter(self) -> ImplSourceIterator {
    ImplSourceIterator {
      source: self,
      current_pos: 0,
    }
  }
}

pub struct ImplSourceIterator {
  source: RawSource,
  current_pos: usize,
}

impl Iterator for ImplSourceIterator {
  type Item = f32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.current_pos < self.source.data.len() {
      let pos = self.current_pos;
      self.current_pos += 1;
      Some(self.source.data[pos])
    } else {
      None
    }
  }
}

impl rodio::source::Source for ImplSourceIterator {
  fn current_frame_len(&self) -> Option<usize> {
    Some(self.source.data.len())
  }

  fn channels(&self) -> u16 {
    self.source.num_channels
  }

  fn sample_rate(&self) -> u32 {
    self.source.sample_rate
  }

  fn total_duration(&self) -> Option<Duration> {
    let num_samples = self.source.sample_rate as f64 * self.source.num_channels as f64;
    Some(Duration::from_secs_f64(self.source.data.len() as f64 / num_samples))
  }
}
