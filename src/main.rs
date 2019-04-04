mod win;

use failure::Error;
use img_dedup::Config;
use log::{debug, info};
use relm::Widget;
use simplelog::{LevelFilter, TermLogger};
use structopt::StructOpt;
use win::Win;

fn main() -> Result<(), Error> {
    let config = Config::from_args();

    let level = match config.verbosity {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        4 | _ => LevelFilter::Trace,
    };

    TermLogger::init(level, simplelog::Config::default())?;
    info!("Starting Image Deduplicator");
    debug!("{:?}", config);

    Win::run(config).unwrap();
    Ok(())
}
