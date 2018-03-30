#[macro_use]
extern crate error_chain;

extern crate clap;

extern crate img_hash;

pub mod config;

#[allow(unknown_lints)]
#[allow(unused_doc_comment)]
pub mod errors {
    error_chain!{}
}

pub use errors::*;

pub use config::Config;

/// Runs the program
pub fn run(config: Config) -> Result<()> {
    println!("Hi");
    Ok(())
}
