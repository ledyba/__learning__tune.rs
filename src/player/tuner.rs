///
/// Tuner maps midi key into frequency.
///
pub trait Tuner {
  fn freq(&self, code: u8) -> f64;
}
