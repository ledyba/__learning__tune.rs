pub struct Lydian;

impl super::Tune for Lydian {
  fn generate() -> Vec<f64> {
    let diff = 0.1;
    let mut f = 1.0;
    let mut sounds = Vec::<f64>::new();
    while (f - 2.0_f64).abs() > diff {
      while f >= 2.0 {
        f = f / 2.0;
      }
      sounds.push(f);
      f = f * 3.0 / 2.0;
      while f >= (2.0 + diff) {
        f = f / 2.0;
      }
    }
    sounds.push(f);
    sounds
  }
}
