// #![feature(unboxed_closures)]
// #![feature(fn_traits)]
// #![feature(proc_macro)]
extern crate bit_vec;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate conrod;
#[macro_use]
extern crate failure;
extern crate image;
extern crate img_hash;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate scoped_threadpool;
extern crate simplelog;
extern crate walkdir;

mod cli;
mod config;
mod hash_type;
mod scanner;
mod win;

use clap::{App, Arg};
use config::Config;
use simplelog::{LevelFilter, TermLogger};

fn main() {
    let matches = App::new("img-dedup")
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("method")
                .short("m")
                .long("method")
                .value_name("METHOD")
                .help(
                    "Name of the method to use. (run --describe-methods for more info)",
                )
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
        .arg(Arg::with_name("cli").short("x").help("Run in a CLI"))
        .get_matches();

    if matches.is_present("describe-methods") {
        println!("Available methods:");
        let methods = hash_type::HashType::available_methods();

        for method in methods {
            println!("{0: <20}: {1}", method.0, method.1);
        }
        return;
    }

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

    let config = Config::new(&matches);
    debug!("{:?}", config);

    if matches.is_present("cli") {
        cli::main(config);
    } else {
        win::main(config);
    }
}
