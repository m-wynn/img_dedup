use config::Config;
use scanner;

pub fn main(config: &Config) {
    let files = scanner::scan_files(&config.directory, config.method, config.hash_length).unwrap();
    scanner::display_matches(&files);
}
