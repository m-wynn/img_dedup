extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate image;
extern crate img_hash;
extern crate itertools;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate scoped_threadpool;
extern crate walkdir;
pub mod config;

#[allow(unknown_lints)]
#[allow(unused_doc_comment)]
pub mod errors {
    error_chain!{}
}

pub use errors::*;

pub use config::Config;

use img_hash::{HashType, ImageHash};
use itertools::Itertools;
use std::{collections::{HashMap, HashSet}, path::PathBuf, sync::{Arc, Mutex}};
use scoped_threadpool::Pool;
use walkdir::WalkDir;

/// Runs the program
pub fn run(config: Config) -> Result<()> {
    let files = scan_files(&config.directory, config.method).chain_err(|| "Could not scan files")?;
    display_matches(files);
    Ok(())
}

fn scan_files(dir: &PathBuf, method: HashType) -> Result<HashMap<ImageHash, HashSet<PathBuf>>> {
    info!("Scanning {:?}", dir);
    let map: HashMap<ImageHash, HashSet<PathBuf>> = HashMap::new();
    let mut pool = Pool::new(num_cpus::get() as u32);
    let map = Arc::new(Mutex::new(map));
    pool.scoped(|scope| {
        for file in WalkDir::new(dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_path_buf())
            .filter(|e| !e.is_dir())
        {
            let img = image::open(&file);
            if let Ok(img) = img {
                info!("Scanning {:?}", file);
                let safe_map = map.clone();
                scope.execute(move || {
                    let hash = ImageHash::hash(&img, 16, method);
                    safe_map
                        .lock()
                        .unwrap()
                        .entry(hash.clone())
                        .or_insert(HashSet::new())
                        .insert(file.to_path_buf());
                    debug!("Done Scanning {:?} with hash {:?}", file, hash);
                });
            }
        }
    });
    Ok(Arc::try_unwrap(map).unwrap().into_inner().unwrap())
}

fn display_matches(hashes: HashMap<ImageHash, HashSet<PathBuf>>) {
    println!("Exact Matches");
    for (_, files) in hashes.clone() {
        if files.len() > 1 {
            println!("[");
            for file in files {
                println!("\t{:?}", file);
            }
            println!("]");
        }
    }
    println!("Partial Matches");
    let mut distances: Vec<(f32, ImageHash, ImageHash)> = hashes
        .keys()
        .tuple_combinations()
        .map(|(a, b)| (a.dist_ratio(b), a.clone(), b.clone()))
        .collect();

    distances.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut prev: f32 = 0.0;
    for distance in distances {
        if distance.0 > 0.3 {
            break;
        }
        if distance.0 != prev {
            print!("{:.1}%", 100. * (1. - distance.0))
        }
        println!("[");
        for files in hashes.get(&distance.1) {
            for file in files {
                println!("\t{:?}", file);
            }
        }
        for files in hashes.get(&distance.2) {
            for file in files {
                println!("\t{:?}", file);
            }
        }
        println!("]");
        prev = distance.0;
    }
}
