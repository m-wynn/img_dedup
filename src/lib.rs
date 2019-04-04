//! A crate that provides a convenient way to scan and review possible image
//! duplicates in a folder.  It is still very much a work in progress but there
//! is a GUI program that uses it attached to this crate.
#![warn(
    bare_trait_objects,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_allocation,
    unused_import_braces,
    unused_qualifications
)]

mod config;
mod hash_type;
mod similar_image;

use bit_vec::BitVec;
use failure::{format_err, Error};
use img_hash::ImageHash;
use itertools::Itertools;
use log::{debug, info, warn};
use rayon::prelude::*;
use std::{cell::RefCell, collections::BinaryHeap, path::PathBuf, rc::Rc, sync::mpsc::Sender};
use walkdir::WalkDir;

pub use self::config::Config;
pub use self::hash_type::{HashType, InnerHashType};
pub use self::similar_image::{SimilarImage, SimilarPair};

/// Taken from the image crate's list of valid images
const VALID_IMAGES: [&str; 15] = [
    "jpg", "jpeg", "png", "gif", "webp", "tif", "tiff", "tga", "bmp", "ico", "hdr", "pbm", "pam",
    "pgm", "ppm",
];

/// Scan image files in a directory
/// Outputs an priority queue of close matches
/// starting with exact duplicates
pub fn scan_files(
    dir: &PathBuf,
    method: HashType,
    hash_size: u32,
    sender: Sender<StatusMsg>,
) -> Result<BinaryHeap<SimilarPair>, Error> {
    let files_to_process = discover_files(dir);
    if files_to_process.is_empty() {
        return Err(format_err!("No readable image files found in {:#?}", dir));
    }
    debug!("List of files found: {:#?}", files_to_process);

    // Alert the GUI how many need to be processed.
    // I originally performed this type of communication of this via two AtomicU32's.
    // My concern with messages was that this might scan too quickly for the GUI to keep up and it
    // would create a backlog of messages.  I've decided to go with messages for these reasons:
    // 1. "Do not communicate by sharing memory; instead, share memory by communicating." -- Go people
    // 2. My tests show that updating the GUI takes barely any time at all.
    //    Meanwhile, hashing tiny images takes about 50ms, and even medium images take a few hundred.
    // 3. Relm (my GUI library) is event-based.  I would end up having to poll the shared values.
    sender
        .send(StatusMsg::Total(files_to_process.len()))
        .unwrap();

    let hashes = hash_files(files_to_process, hash_size, method, sender);

    Ok(sort_ham(hashes))
}

fn discover_files(dir: &PathBuf) -> Vec<PathBuf> {
    info!("Scanning {:?}", dir);
    WalkDir::new(dir)
        .follow_links(false) // no symlinks (TODO: Allow via config?)
        .into_iter()
        .filter_map(|f| match f {
            Ok(f) => Some(f),
            Err(e) => {
                warn!("Cannot access {}", e);
                None
            }
        }) // only files that can be accessed
        .filter(|f| !f.file_type().is_dir()) // no directories, only images
        .filter(|f| {
            VALID_IMAGES.contains(
                &f.path()
                    .extension()
                    .and_then(|s| s.to_str())
                    .map_or("".to_string(), |s| s.to_ascii_lowercase())
                    .as_str(),
            )
        })
        .map(|f| f.path().to_path_buf()) // convert to PathBufs
        .collect()
}

fn hash_files(
    files_to_process: Vec<PathBuf>,
    hash_size: u32,
    method: HashType,
    sender: Sender<StatusMsg>,
) -> Vec<(ImageHash, SimilarImage)> {
    let inner_method = method.into();
    files_to_process
        .into_par_iter()
        .filter_map(|f| match image::open(&f) {
            Ok(i) => Some((SimilarImage::new(f, &i), i)),
            _ => None,
        })
        .map_with(sender, |s, (f, i)| {
            let i = ImageHash::hash(&i, hash_size, inner_method);
            // TODO: An error here probably shouldn't crash the program
            // it's a non-essential status pipe
            s.send(StatusMsg::ImageProcessed)
                .unwrap_or_else(|_| warn!("Could not send status message to other side"));
            (i, f)
        })
        .collect()
}

fn sort_ham(hashes: Vec<(ImageHash, SimilarImage)>) -> BinaryHeap<SimilarPair> {
    hashes
        .into_iter()
        .map(|(hash, image)| (hash.bitv, Rc::new(RefCell::new(image))))
        .tuple_combinations() // yikes C(n, 2)
        // I do not think it is worth the allocation to gain parallelism
        // .collect::<Vec<_>>()
        // .into_par_iter()
        .map(|((hash_a, image_a), (hash_b, image_b))| {
            SimilarPair::new(dist(&hash_a, &hash_b), image_a, image_b)
        })
        .collect()
}

// For 100 images, this will be called 5000 times
// For 1000, this will be called 500,000 times.
// For 10,000 at 115ns/iter that's only 5 seconds.
// So we're probably okay, even at 64-byte arrays

/// Return distance between the vectors
fn dist(a: &BitVec, b: &BitVec) -> usize {
    a.iter()
        .zip(b.iter())
        .filter(|&(left, right)| left == right)
        .count()
}

#[derive(Copy, Clone, Debug)]
/// Status message from scanner
pub enum StatusMsg {
    /// All image files have been discovered
    Total(usize),
    /// An image has been processed
    ImageProcessed,
}

#[cfg(test)]
mod tests {
    use super::*;
    use bit_vec::BitVec;
    use hash_type::InnerHashType;
    use lazy_static::lazy_static;
    use std::path::PathBuf;
    use std::sync::mpsc::channel;

    lazy_static! {
        static ref TEST_PATHS: Vec<PathBuf> = vec![
            PathBuf::from("test/rustBsquish500.jpg"),
            PathBuf::from("test/rustA500.jpg"),
            PathBuf::from("test/rustA500_copy.jpg"),
            PathBuf::from("test/rustB250.jpg")
        ];
        static ref TEST_DATA: Vec<(ImageHash, SimilarImage)> = vec![
            (
                ImageHash {
                    hash_type: InnerHashType::Mean,
                    bitv: BitVec::from_bytes(&[73, 96, 39, 31, 219, 255, 177, 191]),
                },
                SimilarImage::test_image(PathBuf::from("test/rustBsquish500.jpg")),
            ),
            (
                ImageHash {
                    hash_type: InnerHashType::Mean,
                    bitv: BitVec::from_bytes(&[227, 235, 255, 249, 243, 120, 62, 60]),
                },
                SimilarImage::test_image(PathBuf::from("test/rustA500.jpg")),
            ),
            (
                ImageHash {
                    hash_type: InnerHashType::Mean,

                    bitv: BitVec::from_bytes(&[227, 235, 255, 249, 243, 120, 62, 60]),
                },
                SimilarImage::test_image(PathBuf::from("test/rustA500_copy.jpg")),
            ),
            (
                ImageHash {
                    hash_type: InnerHashType::Mean,
                    bitv: BitVec::from_bytes(&[137, 107, 126, 63, 190, 185, 243, 187]),
                },
                SimilarImage::test_image(PathBuf::from("test/rustB250.jpg")),
            ),
        ];
    }

    #[test]
    fn test_hash_files() {
        let (sender, _) = channel();
        let hashed_files = hash_files(TEST_PATHS.to_vec(), 8, "Mean".parse().unwrap(), sender);

        for ((hash1, img1), (hash2, img2)) in TEST_DATA.iter().zip(hashed_files.iter()) {
            assert_eq!(img1.path, img2.path);
            assert_eq!(hash1, hash2);
        }
    }

    #[test]
    fn test_sort_ham() {
        let test_images: Vec<_> = TEST_DATA
            .iter()
            .map(|d| Rc::new(RefCell::new(d.1.clone())))
            .collect();
        let expected_result: [SimilarPair; 6] = [
            SimilarPair::new(33, test_images[0].clone(), test_images[1].clone()),
            SimilarPair::new(33, test_images[0].clone(), test_images[2].clone()),
            SimilarPair::new(37, test_images[1].clone(), test_images[3].clone()),
            SimilarPair::new(37, test_images[2].clone(), test_images[3].clone()),
            SimilarPair::new(44, test_images[0].clone(), test_images[3].clone()),
            SimilarPair::new(64, test_images[1].clone(), test_images[2].clone()),
        ];
        let actual_results = sort_ham(TEST_DATA.to_vec()).into_sorted_vec();
        assert_eq!(actual_results.len(), expected_result.len());
        for (pair_a, pair_b) in expected_result.iter().zip(actual_results.into_iter()) {
            assert_eq!(*pair_a, pair_b);
        }
    }
}
