mod tune;

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
      .value_parser(["pythagoras", "japan"])
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
  use log::{info, warn, error};
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
      let sounds = tuner.tune::<tune::Pythagoras>(440.0);
      for (idx, _factor, hz) in &sounds {
        info!("{}, {}", idx, hz);
      }
      info!("ドを基準として音名と合わせると：");
      let names = ["ド", "ド#", "レ", "レ#", "ミ", "ファ", "ファ#", "ソ", "ソ#", "ラ", "ラ#", "シ", "ド"];
      for ((idx, factor, _hz), name) in sounds.iter().zip(names) {
        info!("{}, {}, {}", idx, name, factor);
      }
    },
    "japan" => {
      let sounds = tuner.tune::<tune::Japan>(440.0);
      for (idx, _factor, hz) in &sounds {
        info!("{}, {}", idx, hz);
      }
      info!("ドを基準として音名と合わせると：");
      let names = ["ド", "レ", "ミ", "ソ", "ラ"];
      for ((idx, factor, _hz), name) in sounds.iter().zip(names) {
        info!("{}, {}, {}", idx, name, factor);
      }
    },
    name => {
      error!("Unknown name: {}", name)
    },
  }
  Ok(())
}
