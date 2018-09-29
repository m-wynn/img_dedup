use clap::ArgMatches;
use crate::hash_type::HashType;
use std::path::PathBuf;

const DEFAULT_DIR: &str = ".";
const DEFAULT_HASH_LENGTH: u32 = 16;

/// Configuration of the scanner
#[derive(Clone, Debug)]
pub struct Config {
    /// The directory to scan
    pub directory: PathBuf,
    /// The hashing method to use
    pub method: HashType,
    /// The square root of the length of the hash
    pub hash_size: u32,
}

impl Config {
    /// Creates a new Config item from Clap args.
    /// Expects:
    /// `method`: String
    /// `hash_size`: u32
    /// `directory`: String
    /// # Example
    /// ```
    /// let matches = App::new("img-dedup")
    ///     .arg(Arg::with_name("directory").takes_value(true).index(1))
    ///     .arg(Arg::with_name("hash_size").takes_value(true))
    ///     .arg(Arg::with_name("method").takes_value(true))
    ///     .get_matches_from_safe(vec![
    ///         "img_dedup", "./img",
    ///         "--hash_size", "32",
    ///         "--method", "Block",
    ///     ]);
    /// assert_eq!(
    ///     Config::new(matches),
    ///     Config {
    ///         directory: PathBuf::from("img_dedup"),
    ///         hash_size: 32,
    ///         method: HashType::new(InnerHashType::Mean)
    ///     }
    /// );
    /// ```
    pub fn new(matches: &ArgMatches) -> Config {
        let method: HashType = match matches.value_of("method") {
            Some(method_str) => method_str.parse().unwrap_or_default(),
            _ => HashType::default(),
        };
        let hash_size: u32 = match matches.value_of("hash_size") {
            Some(length_string) => match length_string.parse() {
                Ok(length) => length,
                _ => DEFAULT_HASH_LENGTH,
            },
            _ => DEFAULT_HASH_LENGTH,
        };

        Config {
            directory: PathBuf::from(matches.value_of("directory").unwrap_or(DEFAULT_DIR)),
            method,
            hash_size,
        }
    }

    /// Set the directory on the Config item
    pub fn set_directory(&mut self, dir: &str) {
        self.directory = PathBuf::from(dir);
    }

    /// Set the method on the Config item
    pub fn set_method(&mut self, method: HashType) {
        self.method = method;
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            directory: PathBuf::from(DEFAULT_DIR),
            method: HashType::default(),
            hash_size: DEFAULT_HASH_LENGTH,
        }
    }
}
