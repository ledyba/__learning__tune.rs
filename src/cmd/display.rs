use anyhow::anyhow;
use log::info;
use crate::tune::Tuner;

pub fn display(tune_name: &str) -> anyhow::Result<()> {
  let c5hz = 523.2511306011974;
  use crate::{tune, sound, tune::Tuner};
  info!("Let's display tune: {} tuning", tune_name);
  match tune_name {
    "pythagoras" => {
      let mut sounds = Tuner::<tune::Pythagoras>::new().tune();
      for (idx, factor) in &sounds {
        info!("{}, {}", idx, factor * c5hz);
      }
      info!("-- original order --");
      // http://www15.plala.or.jp/gundog/homepage/densi/onkai/onkai.html
      info!("レ(D)を基準として音名と合わせると：");
      sounds.sort_by_key(|(idx, _factor)| *idx);
      let names = ["ファ(F)", "ソ(G)", "ラ(A)", "シ(B)", "ド(C)", "レ(D)", "ミ(E)"];
      let mut sounds = sounds.iter().zip(names).map(|((a,b), c)| (*a, *b, c)).collect::<Vec<_>>();
      for (idx, factor, name) in &sounds {
        info!("{}, {}, {}, {} [Hz]", idx, name, factor, factor * c5hz);
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
        info!("{}, {}, {}, {} [Hz]", idx, name, factor, factor * c5hz);
      }
      let mut sounds = sounds.iter().map(|(_idx, factor, _name)| *factor * c5hz).collect::<Vec<_>>();
      sounds.sort_by(|a, b| a.partial_cmp(b).unwrap());
      sound::output(tune_name, &sounds)?;
    },
    "lydian" => {
      let sounds = Tuner::<tune::Lydian>::new().tune();
      for (idx, factor) in &sounds {
        info!("{}, {}", idx, factor * c5hz);
      }
      // Adjust
      info!("ドを基準として音名と合わせると：");
      let names = ["ド(C)", "ド#(C#)", "レ(D)", "レ#(D#)", "ミ(E)", "ファ(F)", "ファ#(F#)", "ソ(G)", "ソ#(G#)", "ラ(A)", "ラ#(A#)", "シ(B)", "ド(C)"];
      let mut sounds = sounds.iter().zip(names).map(|((a, b), d)| (*a, *b, d)).collect::<Vec<_>>();
      for (idx, factor, name) in &sounds {
        info!("{}, {}, {}, {} [Hz]", idx, name, factor, factor * c5hz);
      }
      info!("-- order by idx --");
      sounds.sort_by_key(|(idx, _factor, _name)| *idx);
      for (idx, factor, name) in &sounds {
        info!("{}, {}, {}", idx, name, factor);
      }
      // write lydian.wav
      let mut sounds = sounds.iter().take(7).map(|(_idx, factor, _name)| *factor * c5hz).collect::<Vec<_>>();
      sounds.sort_by(|a, b| a.partial_cmp(b).unwrap());
      sound::output(tune_name, &sounds)?;
    },
    "just" => {
      let sounds = Tuner::<tune::Just>::new().tune();
      for (idx, factor) in &sounds {
        info!("{}, {}", idx, factor * c5hz);
      }
      info!("ドを基準として音名と合わせると：");
      let names = ["ド(A)", "レ(D)", "ミ(E)", "ファ(F)", "ソ(G)", "ラ(A)", "シ(B)"];
      for ((idx, factor), name) in sounds.iter().zip(names) {
        info!("{}, {}, {}", idx, name, factor);
      }
      let sounds = sounds.iter().map(|(_idx, factor)| *factor * c5hz).collect::<Vec<_>>();
      sound::output("just", &sounds)?;
    },
    "japan" => {
      let sounds = Tuner::<tune::Japan>::new().tune();
      for (idx, factor) in &sounds {
        info!("{}, {}", idx, factor * c5hz);
      }
      info!("ドを基準として音名と合わせると：");
      let names = ["ド(A)", "レ(D)", "ミ(E)", "ソ(G)", "ラ(A)"];
      for ((idx, factor), name) in sounds.iter().zip(names) {
        info!("{}, {}, {}", idx, name, factor);
      }
      // write lydian.wav
      let mut sounds = sounds.iter().map(|(_idx, factor)| *factor * c5hz).collect::<Vec<_>>();
      sounds.sort_by(|a, b| a.partial_cmp(b).unwrap());
      sound::output(tune_name, &sounds)?;
    },
    name => {
      let msg = format!("Unknown name: {}", name);
      return Err(anyhow::Error::msg(msg));
    },
  }
  Ok(())
}
