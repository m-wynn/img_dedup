use failure::Fail;
pub use img_hash::HashType as InnerHashType;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

lazy_static! {
    static ref HASH_TYPES: HashMap<&'static str, HashTypeWrapper> = {
        vec![
        ("Block",
            HashTypeWrapper {
                hash_type: InnerHashType::Block,
                desc: "The Blockhash.io algorithm.  Fastest, but also inaccurate.",
            },
        ),
        (
            "Mean",
            HashTypeWrapper {
                hash_type: InnerHashType::Mean,
                desc: "Averages pixels.  Fast, but inaccurate unless looking for exact duplicates.",
            },
        ),
        (
            "Gradient",
            HashTypeWrapper {
                hash_type: InnerHashType::Gradient,
                desc: "Compares edges and color boundaries.  More accurate than mean.",
            },
        ),
        (
            "DoubleGradient",
            HashTypeWrapper {
                hash_type: InnerHashType::DoubleGradient,
                desc: "Gradient but with an extra hash pass.  Slower, but more accurate.",
            },
        ),
        (
            "DCT",
            HashTypeWrapper {
                hash_type: InnerHashType::DCT,
                desc: "Runs a Discrete Cosine Transform.  Slowest, but can detect color changes.",
            },
        )]
        .into_iter()
        .collect::<HashMap<_, _>>()
    };
}

const DEFAULT_METHOD: InnerHashType = InnerHashType::Gradient;

/// Describes a hashtype
/// This struct exists because I need to do parsing to and from strings
/// on the `img_hash::HashType` enum
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HashType {
    hash: InnerHashType,
    name: String,
}

#[derive(Debug, Fail)]
pub enum HashTypeError {
    #[fail(display = "Failure to parse: {}", name)]
    InvalidHashError { name: String },
}

impl FromStr for HashType {
    type Err = HashTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Case Insensitive... maybe UniCase?
        match HASH_TYPES.get(s) {
            Some(wrapper) => Ok(HashType::new(wrapper.hash_type)),
            None => Err(HashTypeError::InvalidHashError { name: s.to_owned() }),
        }
    }
}

impl From<&str> for HashType {
    fn from(s: &str) -> Self {
        // TODO: Case Insensitive... maybe UniCase?
        match HASH_TYPES.get(s) {
            Some(wrapper) => HashType::new(wrapper.hash_type),
            None => panic!("Invalid string"),
        }
    }
}

impl Default for HashType {
    fn default() -> HashType {
        HashType::new(DEFAULT_METHOD)
    }
}

impl ToString for HashType {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl Into<InnerHashType> for HashType {
    fn into(self) -> InnerHashType {
        self.hash
    }
}

impl HashType {
    /// Creates a new HashType from a `img_hash::HashType`
    pub fn new(hash_type: InnerHashType) -> HashType {
        HashType {
            hash: hash_type,
            name: format!("{:?}", hash_type),
        }
    }

    /// Lists the available hashing methods and their descriptions
    pub fn available_methods() -> Vec<(&'static str, &'static str)> {
        HASH_TYPES.iter().map(|(k, v)| (*k, v.desc)).collect()
    }
}

struct HashTypeWrapper {
    hash_type: InnerHashType,
    desc: &'static str,
}
