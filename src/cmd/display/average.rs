fn gen() -> Vec<f64> {
  let mut sounds = Vec::<f64>::new();
  for i in 0..12 {
    sounds.push((2.0_f64).powf(i as f64/12.0))
  }
  sounds
}


pub fn run() -> anyhow::Result<()> {
  Ok(())
}
