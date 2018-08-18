use clap::ArgMatches;
use crate::hash_type::HashType;
use std::path::PathBuf;

const DEFAULT_DIR: &str = ".";
const DEFAULT_HASH_LENGTH: u32 = 16;

#[derive(Clone, Debug)]
pub struct Config {
    /// The directory to scan
    pub directory: PathBuf,
    pub method: HashType,
    pub hash_length: u32,
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Config {
        let method: HashType = match matches.value_of("method") {
            Some(method_str) => method_str.parse().unwrap_or_default(),
            _ => HashType::default(),
        };
        let hash_length: u32 = match matches.value_of("hash_length") {
            Some(length_string) => match length_string.parse() {
                Ok(length) => length,
                _ => DEFAULT_HASH_LENGTH,
            },
            _ => DEFAULT_HASH_LENGTH,
        };

        Config {
            directory: PathBuf::from(matches.value_of("directory").unwrap_or(DEFAULT_DIR)),
            method,
            hash_length,
        }
    }

    pub fn set_directory(&mut self, dir: &str) {
        self.directory = PathBuf::from(dir);
    }

    pub fn set_method(&mut self, method: HashType) {
        self.method = method;
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            directory: PathBuf::from(DEFAULT_DIR),
            method: HashType::default(),
            hash_length: DEFAULT_HASH_LENGTH,
        }
    }
}
