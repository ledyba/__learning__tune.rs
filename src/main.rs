mod tune;
mod sound;
mod display;

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
    .arg(Arg::new("tune")
      .help("tuning name")
      .long("tune")
      .short('t')
      .action(ArgAction::Set)
      .default_value("average")
      .value_parser(["average", "pythagoras", "lydian", "just", "japan"])
      .required(true))
    .arg(Arg::new("display")
      .help("display tune info")
      .long("display")
      .short('d')
      .action(ArgAction::SetTrue))
    .arg(Arg::new("NAME")
      .help("tuning name")
      .index(1)
      .action(ArgAction::Set)
      .default_value("sample/serenade_525_1_(c)ishii.mid")
      .value_parser(value_parser!(String)))
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
  use log::{info, debug};
  let m = app().get_matches();
  let log_level = match m.get_one::<u8>("verbose") {
    None | Some(0) => log::LevelFilter::Info,
    Some(1) => log::LevelFilter::Debug,
    _ => log::LevelFilter::Trace,
  };
  setup_logger(log_level)?;

  let tune_name = m.get_one::<String>("tune").expect("[BUG] --tune is not set").clone();
  if m.get_flag("display") {
    return display::display(&tune_name);
  }

  let file_name = m.get_one::<String>("NAME").expect("[BUG] NAME is not set").clone();
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

  let tune_name = m.get_one::<String>("tune").expect("[BUG] --tune is not set").clone();
  Ok(())
}
