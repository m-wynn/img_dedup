#![feature(integer_atomics)]
#![feature(uniform_paths)]
#![feature(test)]

mod win;

use clap::{App, Arg};
use failure::Error;
use img_dedup::Config;
use img_dedup::HashType;
use log::{debug, info};
use relm::Widget;
use simplelog::{LevelFilter, TermLogger};
use win::Win;

fn main() -> Result<(), Error> {
    // Some of the CLI stuff is a little silly, but it doesn't hurt
    let matches = App::new("img-dedup")
        .about(clap::crate_description!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .arg(
            Arg::with_name("method")
                .short("m")
                .long("method")
                .value_name("METHOD")
                .help("Name of the method to use. (run --describe-methods for more info)")
                .default_value("gradient")
                .case_insensitive(true)
                .possible_values(&["mean", "block", "gradient", "doublegradient", "dct"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("describe-methods")
                .long("describe-methods")
                .help("Describe possible methods"),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .help("Sets level of verbosity")
                .multiple(true),
        )
        .arg(
            Arg::with_name("directory")
                .value_name("DIRECTORY")
                .help("Name of the directory to use")
                .default_value(".")
                .takes_value(true)
                .index(1),
        )
        .arg(
            Arg::with_name("hash_length")
                .value_name("HASH LENGTH")
                .short("l")
                .help("Length of the hashes to generate and compare")
                .default_value("16")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("run")
                .short("x")
                .help("Run without prompting for GUI changes"),
        )
        .get_matches();

    // User has asked for a description of available methods in the CLI
    if matches.is_present("describe-methods") {
        println!("Available methods:");
        let methods = HashType::available_methods();

        for method in methods {
            println!("{0: <20}: {1}", method.0, method.1);
        }
        return Ok(());
    }

    // Verbosity switcher cause I hate that environment variable
    let level = match matches.occurrences_of("v") {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        5 | _ => LevelFilter::Trace,
    };

    TermLogger::init(level, simplelog::Config::default())?;
    info!("Starting Image Deduplicator");

    let config = Config::new(&matches);
    debug!("{:?}", config);

    Win::run(config).unwrap();
    Ok(())
}
