use bit_vec::BitVec;
use crate::hash_type::HashType;
use failure::Error;
use image;
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

pub type PriorityDupes = BinaryHeap<(usize, (PathBuf, PathBuf))>;

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
) -> Result<PriorityDupes, Error> {
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

fn sort_ham(hashes: Vec<(ImageHash, PathBuf)>) -> PriorityDupes {
    hashes
        .into_iter()
        .map(|(hash, path)| (hash.bitv, path))
        .tuple_combinations()
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|((hash_a, path_a), (hash_b, path_b))| (dist(&hash_a, &hash_b), (path_a, path_b)))
        .collect()
}

// Todo: Make fast
fn dist(a: &BitVec, b: &BitVec) -> usize {
    a.iter()
        .zip(b.iter())
        .filter(|&(left, right)| left != right)
        .count()
}
