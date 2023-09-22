pub struct Average;

impl super::Tune for Average {
  fn generate() -> Vec<f64> {
    (0..12).map(|idx| (2.0_f64).powf(idx as f64/12.0)).collect()
  }
}
