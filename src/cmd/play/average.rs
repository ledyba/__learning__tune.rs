use crate::player::{A4, Tuner};

pub struct Average {

}

impl Average {
  pub fn new() -> Self {
    Self {
    }
  }
}

impl Tuner for Average {
  fn freq(&self, code: u8) -> f64 {
    440.0 * (2.0_f64.powf(1.0 / 12.0)).powi(code as i32 - A4)
  }
}
