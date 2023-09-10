use std::process::Command;

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
  use log::{info, error};
  let m = app().get_matches();
  let log_level = match m.get_one::<u8>("verbose") {
    None | Some(0) => log::LevelFilter::Info,
    Some(1) => log::LevelFilter::Debug,
    _ => log::LevelFilter::Trace,
  };
  setup_logger(log_level)?;
  let tuner = tune::Tuner::new();
  match m.get_one::<String>("NAME").map(|it| it.as_str()) {
    None | Some("pythagoras") => {
      tuner.tune::<tune::Japan>(440.0);
    },
    Some("japan") => {
      tuner.tune::<tune::Japan>(440.0);
    },
    Some(name) => {
      error!("Unknown name: {}", name)
    },
  }
  Ok(())
}
