#![feature(test)]
#![feature(uniform_paths)]
#![feature(integer_atomics)]

extern crate test;
mod config;
mod hash_type;

use bit_vec::BitVec;
use failure::Error;
use img_hash::ImageHash;
use itertools::Itertools;
use log::{debug, info};
use rayon::prelude::*;
use std::{
    collections::BinaryHeap,
    path::PathBuf,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};
use walkdir::WalkDir;

pub use self::config::Config;
pub use self::hash_type::HashType;

/// A set of similar images
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct SimilarPair {
    // TODO: Normalize this as a ratio so we can say, e.g. 93% similar
    // But keep Ord
    similarity: usize,
    // TODO: Update this so one (left or right) is the path we most
    // expect the user to delete, based on size of the file, length of
    // path, etc.
    // However, we need to be careful with opening and keeping open files
    left: PathBuf,
    right: PathBuf,
}

impl SimilarPair {
    pub fn new(similarity: usize, left: PathBuf, right: PathBuf) -> SimilarPair {
        SimilarPair {
            similarity,
            left,
            right,
        }
    }
}

/// Taken from the image crate's list of valid images
const VALID_IMAGES: [&str; 15] = [
    "jpg", "jpeg", "png", "gif", "webp", "tif", "tiff", "tga", "bmp", "ico", "hdr", "pbm", "pam",
    "pgm", "ppm",
];

/// Scan image files in a directory
/// Outputs an priority queue of close matches
/// starting with exact duplicates
pub fn scan_files(
    dir: PathBuf,
    method: HashType,
    hash_length: u32,
    total: &Arc<AtomicU32>,
    processed: Arc<AtomicU32>,
) -> Result<BinaryHeap<SimilarPair>, Error> {
    let files_to_process = discover_files(dir);
    debug!("List of files found: {:#?}", files_to_process);

    // Alert the GUI how many need to be processed
    total.store(files_to_process.len() as u32, Ordering::Release);

    let hashes = hash_files(files_to_process, hash_length, method, processed);

    Ok(sort_ham(hashes))
}

fn discover_files(dir: PathBuf) -> Vec<PathBuf> {
    info!("Scanning {:?}", dir);
    WalkDir::new(dir)
        .follow_links(false) // no symlinks (TODO: Allow via config?)
        .into_iter()
        .filter_map(|e| e.ok()) // only files that can be accessed
        .filter(|e| !e.file_type().is_dir()) // no directories, only images
        .filter(|e| VALID_IMAGES.contains(
            &e.path().extension()
            .and_then(|s| s.to_str())
            .map_or("".to_string(), |s| s.to_ascii_lowercase()).as_str()))
        .map(|e| e.path().to_path_buf()) // convert to pathbufs
        .collect()
}

fn hash_files(
    files_to_process: Vec<PathBuf>,
    hash_length: u32,
    method: HashType,
    processed: Arc<AtomicU32>,
) -> Vec<(ImageHash, PathBuf)> {
    let inner_method = method.into();
    files_to_process
        .into_par_iter()
        .filter_map(|f| match image::open(&f) {
            Ok(i) => Some((i, f)),
            _ => None,
        }).map_with(processed, |p, (i, f)| {
            let i = ImageHash::hash(&i, hash_length, inner_method);
            p.fetch_add(1, Ordering::SeqCst);
            (i, f)
        }).collect()
}

fn sort_ham(hashes: Vec<(ImageHash, PathBuf)>) -> BinaryHeap<SimilarPair> {
    hashes
        .into_iter()
        .map(|(hash, path)| (hash.bitv, path))
        .tuple_combinations()  // yikes C(n, 2)
        .collect::<Vec<_>>()  // Needless if we don't `into_par_iter()`
        .into_par_iter()  // At what point is it worth parallelizing this?
        .map(|((hash_a, path_a), (hash_b, path_b))| {
            SimilarPair::new(dist(&hash_a, &hash_b), path_a, path_b)
        }).collect()
}

// Todo: Make fast
// For 100 images, this will be called 5000 times
// For 1000, this will be called 500,000 times.
// For 10,000 at 115ns/iter that's only 5 seconds.
// So we're probably okay, even at 64-byte arrays
fn dist(a: &BitVec, b: &BitVec) -> usize {
    a.iter()
        .zip(b.iter())
        .filter(|&(left, right)| left != right)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bit_vec::BitVec;
    use rand::{thread_rng, Rng};
    use test::Bencher;

    #[bench]
    fn bench_standard(b: &mut Bencher) {
        let mut a_hash_bytes = [0u8, 64];
        let mut b_hash_bytes = [0u8, 64];
        thread_rng().fill(&mut a_hash_bytes[..]);
        thread_rng().fill(&mut b_hash_bytes[..]);
        let a_bits = BitVec::from_bytes(&a_hash_bytes);
        let b_bits = BitVec::from_bytes(&b_hash_bytes);

        b.iter(|| dist(&a_bits, &b_bits))
    }
}
