use crate::cmd::play;

mod cmd;
mod player;

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
    .subcommand_required(true)
    .subcommand(Command::new("play")
      .arg(Arg::new("tune")
        .help("tune name")
        .action(ArgAction::Set)
        .long("tune")
        .short('t')
        .value_parser(play::TUNES))
      .arg(Arg::new("FILENAME")
        .help("midi file name")
        .index(1)
        .action(ArgAction::Set)
        .default_value("sample/serenade_525_1_(c)ishii.mid")
        .value_parser(value_parser!(String))))
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
  use log::debug;
  let m = app().get_matches();
  let log_level = match m.get_one::<u8>("verbose") {
    None | Some(0) => log::LevelFilter::Info,
    Some(1) => log::LevelFilter::Debug,
    _ => log::LevelFilter::Trace,
  };
  setup_logger(log_level)?;
  debug!("Logging level: {}", log_level);

  let (cmd, m) = m.subcommand().expect("No subcommand!");
  match cmd {
    "play" => {
      let file_name = m.get_one::<String>("FILENAME").expect("[BUG] FILENAME is not set").clone();
      let tune_name = m.get_one::<String>("tune").expect("[BUG] --tune is not set").clone();
      play::run(&tune_name, &file_name)
    },
    sub_cmd => {
      Err(anyhow::Error::msg(format!("Unknown subcommand: {}", sub_cmd)))
    }
  }
}
