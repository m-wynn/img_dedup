// #![feature(unboxed_closures)]
// #![feature(fn_traits)]
// #![feature(proc_macro)]
extern crate clap;
#[macro_use]
extern crate conrod;
extern crate failure;
extern crate image;
extern crate img_hash;
extern crate itertools;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate scoped_threadpool;
extern crate simplelog;
extern crate walkdir;

mod config;
mod scanner;
mod win;

fn main() {
    win::main();
}
