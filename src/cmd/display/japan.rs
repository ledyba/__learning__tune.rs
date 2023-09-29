use log::info;
use super::{C4HZ, output};

fn gen() -> Vec<f64> {
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

pub fn run() -> anyhow::Result<()> {
  let sounds = gen();
  let sounds = (0..sounds.len()).zip(sounds).collect::<Vec<_>>();
  for (idx, factor) in &sounds {
    info!("{}, {}", idx, factor * C4HZ);
  }
  info!("ドを基準として音名と合わせると：");
  let names = ["ド(A)", "レ(D)", "ミ(E)", "ソ(G)", "ラ(A)"];
  for ((idx, factor), name) in sounds.iter().zip(names) {
    info!("{}, {}, {}", idx, name, factor);
  }
  // write lydian.wav
  let mut sounds = sounds.iter().map(|(_idx, factor)| *factor * C4HZ).collect::<Vec<_>>();
  sounds.sort_by(|a, b| a.partial_cmp(b).unwrap());
  output("japan", &sounds)?;
  Ok(())
}
