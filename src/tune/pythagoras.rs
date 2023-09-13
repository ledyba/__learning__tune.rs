pub struct Pythagoras;

impl super::Tune for Pythagoras {
  fn generate() -> Vec<f64> {
    // https://ja.wikipedia.org/wiki/%E3%83%94%E3%82%BF%E3%82%B4%E3%83%A9%E3%82%B9%E9%9F%B3%E5%BE%8B#%E6%96%B9%E6%B3%95
    let factors = [0.0, 1.0, 2.0, 3.0, -1.0, -2.0, -3.0];
    let sounds = factors.iter().map(|f| (1.5_f64).powf(*f)).collect::<Vec<_>>();
    let base = *sounds.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    sounds
      .iter()
      .map(|f| f / base)
      .map(|f| {
        let mut f = f;
        while f > 2.0 {
          f /= 2.0;
        }
        f
      })
      .collect()
  }
}
