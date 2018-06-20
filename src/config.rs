use clap::ArgMatches;
use hash_type::HashType;
use std::path::PathBuf;

const DEFAULT_DIR: &str = ".";
const DEFAULT_METHOD_STR: &str = "gradient";
const DEFAULT_HASH_LENGTH: u32 = 16;

#[derive(Clone, Debug)]
pub struct Config {
    /// The directory to scan
    pub directory: PathBuf,
    pub method: HashType,
    pub method_str: String,
    pub hash_length: u32,
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Config {
        let method_str = matches.value_of("method").unwrap_or(DEFAULT_METHOD_STR);
        let method: HashType = method_str.parse().unwrap();
        let hash_length: u32 = if let Some(length_string) = matches.value_of("hash_length") {
            if let Ok(length) = length_string.parse() {
                length
            } else {
                DEFAULT_HASH_LENGTH
            }
        } else {
            DEFAULT_HASH_LENGTH
        };

        Config {
            directory: PathBuf::from(matches.value_of("directory").unwrap_or(DEFAULT_DIR)),
            method_str: method_str.to_owned(),
            method,
            hash_length,
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            directory: PathBuf::from(DEFAULT_DIR),
            method_str: DEFAULT_METHOD_STR.to_owned(),
            method: DEFAULT_METHOD_STR.parse().unwrap(),
            hash_length: DEFAULT_HASH_LENGTH,
        }
    }
}
