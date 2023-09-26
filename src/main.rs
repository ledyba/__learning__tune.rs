mod tune;
mod cmd;

fn app() -> clap::Command {
  use clap::{Command, Arg, ArgAction, value_parser};
  let tunes = ["average", "pythagoras", "lydian", "just", "japan"];
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
    .subcommand(Command::new("display")
      .about("Display about tune")
      .arg(Arg::new("NAME")
        .help("tuning name")
        .action(ArgAction::Set)
        .index(1)
        .value_parser(tunes)
        .required(true)))
    .subcommand(Command::new("play")
      .arg(Arg::new("tune")
        .help("tune name")
        .action(ArgAction::Set)
        .long("tune")
        .short('t')
        .value_parser(tunes))
      .arg(Arg::new("FILENAME")
        .help("midi file name")
        .index(1)
        .action(ArgAction::Set)
        .default_value("sample/serenade_525_1_(c)ishii.mid")
        .value_parser(value_parser!(String))))
}

fn setup_logger(log_level: log::LevelFilter) -> Result<(), fern::InitError> {
  use fern::Dispatch;
  Dispatch::new()
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
  use log::{info, debug};
  let m = app().get_matches();
  let log_level = match m.get_one::<u8>("verbose") {
    None | Some(0) => log::LevelFilter::Info,
    Some(1) => log::LevelFilter::Debug,
    _ => log::LevelFilter::Trace,
  };
  setup_logger(log_level)?;

  let (cmd, m) = m.subcommand().expect("No subcommand!");
  match cmd {
    "display" => {
      let tune_name = m.get_one::<String>("NAME").expect("[BUG] NAME is not set").clone();
      cmd::display::display(&tune_name)
    },
    "play" => {
      let file_name = m.get_one::<String>("FILENAME").expect("[BUG] FILENAME is not set").clone();
      let tune_name = m.get_one::<String>("tune").expect("[BUG] --tune is not set").clone();
      let file_bytes = std::fs::read(&file_name)?;
      let mid = {
        info!("Parsing: \"{}\"", &file_name);
        let mid = midly::Smf::parse(&file_bytes)?;
        debug!("  - Timing: {:?}", mid.header.format);
        debug!("  - Format: {:?}", mid.header.timing);
        debug!("  - {} tracks", mid.tracks.len());
        for (idx, track) in (0..mid.tracks.len()).zip(&mid.tracks) {
          debug!("    - Track[{}]: {} events", idx, track.len());
        }
        mid
      };
      Ok(())
    },
    sub_cmd => {
      Err(anyhow::Error::msg(format!("Unknown subcommand: {}", sub_cmd)))
    }
  }
}
