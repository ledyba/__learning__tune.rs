pub trait Tune {
  fn generate() -> Vec<f64>;
}

pub struct Tuner<T: Tune> {
  _phantom_data: std::marker::PhantomData<T>,
}

impl <T: Tune> Tuner<T> {
  pub fn new() -> Self {
    Self {
      _phantom_data: std::marker::PhantomData::default(),
    }
  }

  pub fn tune(&self) -> Vec<(usize, f64)> {
    let sounds = T::generate();
    let mut sounds = (1..=sounds.len()).zip(sounds).collect::<Vec<_>>();
    sounds.sort_by(|a, b| a.1.total_cmp(&b.1));
    sounds.iter().map(|(cnt, factor)| {
      (*cnt, *factor)
    }).collect::<Vec<_>>()
  }
}
