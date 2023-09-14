mod pythagoras;
pub use pythagoras::*;
mod lydian;
pub use lydian::*;
mod japan;
pub use japan::*;
mod just;
pub use just::*;

pub trait Tune {
  fn generate() -> Vec<f64>;
}

pub struct Tuner {

}

impl Tuner {
  pub fn new() -> Self {
    Self {
    }
  }

  pub fn tune<T: Tune>(&self, base: f64) -> Vec<(usize, f64, f64)> {
    let sounds = T::generate();
    let mut sounds = (1..=sounds.len()).zip(sounds).collect::<Vec<_>>();
    sounds.sort_by(|a, b| a.1.total_cmp(&b.1));
    sounds.iter().map(|(cnt, factor)| {
      (*cnt, *factor, *factor * base)
    }).collect::<Vec<_>>()
  }
}
