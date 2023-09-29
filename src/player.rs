///
/// Tuner maps midi key into frequency.
///
pub trait Tuner {
  fn freq(&self, code: u8) -> f64;
}
/// A4 midi code
pub const A4: u8 = 69;

pub struct Player {
  tuner: Box<dyn Tuner>,
}

impl Player {
  pub fn new(tuner: Box<dyn Tuner>) -> Self {
    Self {
      tuner,
    }
  }
}
