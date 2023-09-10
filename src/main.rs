fn main() -> anyhow::Result<()> {
  let diff = 0.1;
  //let diff = 0.0001;
  let mut f = 1.0;
  let mut sounds = Vec::<(i32, f64)>::new();
  let mut idx = 0;
  while (f - 2.0_f64).abs() > diff && idx < 1000 {
    idx += 1;
    while f >= 2.0 {
      f = f / 2.0;
    }
    println!("{}, {}", idx, f);
    sounds.push((idx, f));
    f = f * 3.0;
    while f >= (2.0 + diff) {
      f = f / 2.0;
    }
  }
  println!("{}, {}", idx + 1, f);
  sounds.push((idx + 1, f));

  println!("---- sorted ----");
  sounds.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
  for (idx, f) in sounds {
    println!("{}, {}", idx, f);
  }
  Ok(())
}
