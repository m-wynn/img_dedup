#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;
extern crate simplelog;

extern crate image;
extern crate img_hash;

extern crate img_dedup;

use clap::{App, Arg};
use img_dedup::{config::Config, errors::Error};
use simplelog::{LevelFilter, TermLogger};

fn main() {
    let matches = App::new("img-dedup")
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .arg(Arg::with_name("directory")
             .short("d")
             .long("directory")
             .value_name("FOLDER")
             .help("Name of the directory to use")
             .default_value(".")
             .takes_value(true))
        .arg(Arg::with_name("method")
             .short("m")
             .long("method")
             .value_name("METHOD")
             .help("Name of the method to use")
             .default_value("gradient")
             .case_insensitive(true)
             .possible_values(&["mean", "block", "gradient", "doublegradient", "dct"])
             .takes_value(true))
        .arg(Arg::with_name("v")
             .short("v")
             .help("Sets level of verbosity")
             .multiple(true))
        .get_matches();

    let level = match matches.occurrences_of("v") {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        5 | _ => LevelFilter::Trace,
    };

    TermLogger::init(level, simplelog::Config::default()).unwrap();
    info!("Starting Image Deduplicator");

    let config = Config::new(matches);
    debug!("{:?}", config);

    if let Err(ref err) = img_dedup::run(config) {
        handle_error(err);
    }
}

fn handle_error(e: &Error) {
    use std::io::Write;
    let stderr = &mut ::std::io::stderr();
    let errmsg = "Error writing to stderr";
    writeln!(stderr, "error: {}", e).expect(errmsg);

    for e in e.iter().skip(1) {
        writeln!(stderr, "caused by: {}", e).expect(errmsg);
    }

    if let Some(backtrace) = e.backtrace() {
        writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
    }

    std::process::exit(1);
}
