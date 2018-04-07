use clap::ArgMatches;
use img_hash::HashType;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    /// The directory to scan
    pub directory: PathBuf,
    pub method: HashType,
}

impl Config {
    pub fn new(matches: ArgMatches) -> Config {
        // Would be a lot easier if we overwrote img_hash::HashType with a macro
        let method = match matches.value_of("method").unwrap_or("gradient") {
            "mean" => HashType::Mean,
            "block" => HashType::Block,
            "doublegradient" => HashType::DoubleGradient,
            "dct" => HashType::DCT,
            "gradient" | _ => HashType::Gradient,
        };

        Config {
            directory: PathBuf::from(matches.value_of("directory").unwrap_or(".")),
            method,
        }
    }
}
