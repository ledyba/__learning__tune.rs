use log::info;
use super::{C4HZ, output};

fn gen() -> Vec<f64> {
  // https://ja.wikipedia.org/wiki/%E7%B4%94%E6%AD%A3%E5%BE%8B
  let factors = [(1,1), (9, 8), (5, 4), (4,3), (3,2), (5,3), (15,8),];
  factors.iter().map(|(a,b)| (*a as f64) / (*b as f64)).collect::<Vec<_>>()
}

pub fn run() -> anyhow::Result<()> {
  let sounds = gen();
  let sounds = (0..sounds.len()).zip(sounds).collect::<Vec<_>>();
  for (idx, factor) in &sounds {
    info!("{}, {}", idx, factor * C4HZ);
  }
  info!("ドを基準として音名と合わせると：");
  let names = ["ド(A)", "レ(D)", "ミ(E)", "ファ(F)", "ソ(G)", "ラ(A)", "シ(B)"];
  for ((idx, factor), name) in sounds.iter().zip(names) {
    info!("{}, {}, {}", idx, name, factor);
  }
  let sounds = sounds.iter().map(|(_idx, factor)| *factor * C4HZ).collect::<Vec<_>>();
  output("just", &sounds)?;
  Ok(())
}
