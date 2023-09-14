mod tune;
mod sound;

fn app() -> clap::Command {
  use clap::{Command, Arg, ArgAction, value_parser};
  Command::new("tune")
    .author("Kaede Fujisaki")
    .about("Tune")
    .arg(Arg::new("verbose")
      .long("verbose")
      .short('v')
      .required(false)
      .action(ArgAction::Count)
      .value_parser(value_parser!(u8))
      .help("Show verbose message"))
    .arg(Arg::new("NAME")
      .help("tuning name")
      .index(1)
      .action(ArgAction::Set)
      .value_parser(["pythagoras", "lydian", "just", "japan"])
      .required(true))
}

fn setup_logger(log_level: log::LevelFilter) -> Result<(), fern::InitError> {
  fern::Dispatch::new()
    .format(|out, message, record| {
      out.finish(format_args!(
        "{}[{}] {}",
        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
        record.level(),
        message
      ))
    })
    .level(log_level)
    .chain(std::io::stdout())
    //.chain(fern::log_file("output.log")?)
    .apply()?;
  Ok(())
}
fn main() -> anyhow::Result<()> {
  let c5hz = 523.2511306011974;
  use log::{info, error};
  let m = app().get_matches();
  let log_level = match m.get_one::<u8>("verbose") {
    None | Some(0) => log::LevelFilter::Info,
    Some(1) => log::LevelFilter::Debug,
    _ => log::LevelFilter::Trace,
  };
  setup_logger(log_level)?;
  let name = m.get_one::<String>("NAME").expect("[BUG] NAME is not set").clone();
  info!("Let's tune: {} tuning", name);
  let tuner = tune::Tuner::new();
  match name.as_str() {
    "pythagoras" => {
      let mut sounds = tuner.tune::<tune::Pythagoras>();
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
      sound::output("pythagoras", &sounds)?;
    },
    "lydian" => {
      let sounds = tuner.tune::<tune::Lydian>();
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
      sound::output("lydian", &sounds)?;
    },
    "just" => {
      let sounds = tuner.tune::<tune::Just>();
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
      let sounds = tuner.tune::<tune::Japan>();
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
      sound::output("japan", &sounds)?;
    },
    name => {
      error!("Unknown name: {}", name)
    },
  }
  Ok(())
}
