mod tune;

fn main() -> anyhow::Result<()> {
  let tuner = tune::Tuner::new();
  tuner.tune::<tune::Pythagoras>(440.0);
  Ok(())
}
