use crate::player::{A4, A4HZ, Tuner};

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
    A4HZ * (2.0_f64.powf(1.0 / 12.0)).powi(code as i32 - A4)
  }
}
