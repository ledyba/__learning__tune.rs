use log::info;
use crate::cmd::display::C4HZ;

fn gen() -> Vec<f64> {
  // https://ja.wikipedia.org/wiki/%E3%83%94%E3%82%BF%E3%82%B4%E3%83%A9%E3%82%B9%E9%9F%B3%E5%BE%8B#%E6%96%B9%E6%B3%95
  let factors = [0.0, 1.0, 2.0, 3.0, -1.0, -2.0, -3.0];
  let sounds = factors.iter().map(|f| (1.5_f64).powf(*f)).collect::<Vec<_>>();
  let base = *sounds.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
  let mut sounds = sounds
    .iter()
    .map(|f| f / base)
    .map(|f| {
      let mut f = f;
      while f > 2.0 {
        f /= 2.0;
      }
      f
    })
    .collect::<Vec<_>>();
  sounds
}

pub fn run() -> anyhow::Result<()> {
  let mut sounds = gen();
  sounds.sort_by(|a, b| a.partial_cmp(b).unwrap());
  let sounds = (0..sounds.len()).zip(sounds).collect::<Vec<_>>();
  for (idx, factor) in &sounds {
    info!("{}, {}", idx, factor * super::C4HZ);
  }
  info!("-- original order --");
  // http://www15.plala.or.jp/gundog/homepage/densi/onkai/onkai.html
  info!("レ(D)を基準として音名と合わせると：");
  let names = ["ファ(F)", "ソ(G)", "ラ(A)", "シ(B)", "ド(C)", "レ(D)", "ミ(E)"];
  let mut sounds = sounds.iter().zip(names).map(|((a,b), c)| (*a, *b, c)).collect::<Vec<_>>();
  for (idx, factor, name) in &sounds {
    info!("{}, {}, {}, {} [Hz]", idx, name, factor, factor * C4HZ);
  }
  sounds[4].1 /= 4.0;
  sounds[5].1 /= 4.0;
  sounds[6].1 /= 4.0;
  info!("-- re-order --");
  sounds.sort_by(|(_idx1, factor1, _name1), (_idx2, factor2, _name2)| factor1.partial_cmp(factor2).unwrap());
  let first = sounds[0].1;
  let sounds = sounds.iter().map(|(idx, factor, name)| {
    let mut factor = *factor / first;
    while  factor < 1.0 {
      factor *= 2.0;
    }
    while factor >= 2.0 {
      factor /= 2.0;
    }
    (*idx, factor, *name)
  }).collect::<Vec<_>>();
  for (idx, factor, name) in &sounds {
    info!("{}, {}, {}, {} [Hz]", idx, name, factor, factor * C4HZ);
  }
  let mut sounds = sounds.iter().map(|(_idx, factor, _name)| *factor * C4HZ).collect::<Vec<_>>();
  sounds.sort_by(|a, b| a.partial_cmp(b).unwrap());
  super::output("pythagoras", &sounds)?;
  Ok(())
}
