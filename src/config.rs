use crate::hash_type::HashType;
use std::path::PathBuf;
use structopt::StructOpt;

const DEFAULT_DIR: &str = ".";
const DEFAULT_HASH_LENGTH: u32 = 16;

/// Configuration of the scanner
#[derive(Clone, Debug, Eq, PartialEq, StructOpt)]
#[structopt(name = "img_dedup", about = "Deduplicate images in a directory")]
pub struct Config {
    /// The directory to scan
    #[structopt(
        parse(from_os_str),
        short = "d",
        long = "directory",
        default_value = "."
    )]
    pub directory: PathBuf,
    /// The hashing method to use
    #[structopt(
        parse(from_str),
        short = "h",
        long = "hash_type",
        default_value = "Gradient"
    )]
    pub method: HashType,
    /// The square root of the length of the hash
    #[structopt(short = "l", long = "hash_length", default_value = "16")]
    pub hash_size: u32,
    /// The square root of the length of the hash
    #[structopt(short = "v", parse(from_occurrences))]
    pub verbosity: u8,
}

impl Config {
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
            verbosity: 0,
        }
    }
}
