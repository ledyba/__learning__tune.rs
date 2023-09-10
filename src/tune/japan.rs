pub struct Japan;

impl super::Tune for Japan {
  fn generate() -> Vec<f64> {
    let mut sounds = Vec::<f64>::new();
    for i in 0..5 {
      let mut f = 3.0_f64.powi(i);
      while f > 2.0 {
        f /= 2.0;
      }
      sounds.push(f);
    }
    sounds
  }
}
