use log::info;

pub fn display(tune_name: &str) -> anyhow::Result<()> {
  let c5hz = 523.2511306011974;
  use crate::{tune, tune::Tuner};
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
      output(tune_name, &sounds)?;
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
      output(tune_name, &sounds)?;
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
      output("just", &sounds)?;
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
      output(tune_name, &sounds)?;
    },
    name => {
      let msg = format!("Unknown name: {}", name);
      return Err(anyhow::Error::msg(msg));
    },
  }
  Ok(())
}
pub fn output(name: &str, sounds: &Vec<f64>) -> anyhow::Result<()> {
  use std::f64::consts::PI;

  let spec = hound::WavSpec {
    channels: 1,
    sample_rate: 44100,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
  };
  let mut writer = hound::WavWriter::create(format!("{}.wav", name), spec)?;
  let num_samples = spec.sample_rate as usize;
  let num_samples_near_last = num_samples * 99 / 100;
  let num_samples_near_beg = num_samples * 1 / 100;
  let mut t = 0;
  let amplitude = i16::MAX as f64;
  let mut max = 0.0;
  let mut min = 0.0;
  let mut silent = true;
  for hz in sounds {
    for dt in 0..num_samples {
      let x = (t + dt) as f64 / (spec.sample_rate as f64);
      let sample = (x * hz * 2.0 * PI).sin();
      if dt <= num_samples_near_beg {
        silent = true;
      }else if !silent && num_samples_near_last <= dt && sample.abs() < 0.01 {
        silent = true;
      }else if silent && (num_samples_near_beg <= dt && dt <= num_samples_near_last) && sample.abs() >= 0.01 {
        silent = false;
      }
      let sample = if silent {
        0.0
      } else {
        sample
      };
      max = sample.max(max);
      min = sample.min(min);
      writer.write_sample((sample * amplitude) as i16)?;
    }
    t += num_samples;
  }
  Ok(())
}
