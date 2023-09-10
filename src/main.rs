mod tune;

fn main() -> anyhow::Result<()> {
  let tuner = tune::Tuner::new();
  tuner.tune::<tune::Pythagoras>();
  Ok(())
}
