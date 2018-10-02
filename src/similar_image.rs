use image::{DynamicImage, GenericImage};
use std::cmp::Ordering;
use std::path::PathBuf;

/// Represents an image and attributes that may be used for comparison
/// to choose the favored of two duplicates
#[derive(Clone, Debug, Eq, Ord)]
pub struct SimilarImage {
    /// Path to the image
    pub path: PathBuf,
    width: u32,
    height: u32,
}

impl SimilarImage {
    /// Create a new `SimilarImage`.
    /// This function will scan the image for attributes like dimensions
    pub fn new(path: PathBuf, image: &DynamicImage) -> SimilarImage {
        let (width, height) = image.dimensions();
        SimilarImage {
            path,
            width,
            height,
        }
    }

    /// Do not use outside of testing
    pub fn test_image(path: PathBuf) -> SimilarImage {
        SimilarImage {
            path: path,
            width: 0,
            height: 0,
        }
    }
}

impl PartialEq for SimilarImage {
    fn eq(&self, other: &SimilarImage) -> bool {
        self.path == other.path
    }
}

/// Greater is more desireable
/// TODO: Based on color type -> color depth -> size -> format -> path length
impl PartialOrd for SimilarImage {
    fn partial_cmp(&self, other: &SimilarImage) -> Option<Ordering> {
        self.path.partial_cmp(&other.path)
    }
}

/// A set of two similar images
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct SimilarPair {
    // TODO: Normalize this as a ratio so we can say, e.g. 93% similar
    // But keep Ord
    ///Similarity Index
    pub similarity: usize,
    // TODO: Update this so one (left or right) is the path we most
    // expect the user to delete, based on size of the file, length of
    // path, etc.
    /// Image least expected to be deleted
    pub left: SimilarImage,
    /// Image most expected to be deleted
    pub right: SimilarImage,
}

impl SimilarPair {
    /// Create a new similar pair
    /// TODO: Does this need to be public?  Maybe look at crate visibility
    pub fn new(similarity: usize, left: SimilarImage, right: SimilarImage) -> SimilarPair {
        SimilarPair {
            similarity,
            left,
            right,
        }
    }
}
