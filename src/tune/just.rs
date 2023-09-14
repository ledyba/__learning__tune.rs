pub struct Just;

impl super::Tune for Just {
  fn generate() -> Vec<f64> {
    // https://ja.wikipedia.org/wiki/%E7%B4%94%E6%AD%A3%E5%BE%8B
    let factors = [(1,1), (9, 8), (5, 4), (4,3), (3,2), (5,3), (15,8),];
    factors.iter().map(|(a,b)| (*a as f64) / (*b as f64)).collect::<Vec<_>>()
  }
}
