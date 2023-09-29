use log::info;
use super::C4HZ;

fn gen() -> Vec<f64> {
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

pub fn run() -> anyhow::Result<()> {
  let sounds = gen();
  let sounds = (0..sounds.len()).zip(sounds).collect::<Vec<_>>();
  for (idx, factor) in &sounds {
    info!("{}, {}", idx, factor * C4HZ);
  }
  // Adjust
  info!("ドを基準として音名と合わせると：");
  let names = ["ド(C)", "ド#(C#)", "レ(D)", "レ#(D#)", "ミ(E)", "ファ(F)", "ファ#(F#)", "ソ(G)", "ソ#(G#)", "ラ(A)", "ラ#(A#)", "シ(B)", "ド(C)"];
  let mut sounds = sounds.iter().zip(names).map(|((a, b), d)| (*a, *b, d)).collect::<Vec<_>>();
  for (idx, factor, name) in &sounds {
    info!("{}, {}, {}, {} [Hz]", idx, name, factor, factor * C4HZ);
  }
  info!("-- order by idx --");
  sounds.sort_by_key(|(idx, _factor, _name)| *idx);
  for (idx, factor, name) in &sounds {
    info!("{}, {}, {}", idx, name, factor);
  }
  // write lydian.wav
  let mut sounds = sounds.iter().take(7).map(|(_idx, factor, _name)| *factor * C4HZ).collect::<Vec<_>>();
  sounds.sort_by(|a, b| a.partial_cmp(b).unwrap());
  super::output("lydian", &sounds)?;
  Ok(())
}
