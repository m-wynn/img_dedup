use hash_type;
pub use img_hash::HashType as InnerHashType;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

lazy_static! {
    static ref HASH_TYPES: HashMap<&'static str, hash_type::HashTypeWrapper> = {
        let mut m = HashMap::new();
        m.insert(
            "Block",
            HashTypeWrapper {
                hash_type: InnerHashType::Block,
                desc: "The Blockhash.io algorithm.  Fastest, but also inaccurate.",
            },
        );
        m.insert(
            "Mean",
            HashTypeWrapper {
                hash_type: InnerHashType::Mean,
                desc: "Averages pixels.  Fast, but inaccurate unless looking for exact duplicates.",
            },
        );
        m.insert(
            "Gradient",
            HashTypeWrapper {
                hash_type: InnerHashType::Gradient,
                desc: "Compares edges and color boundaries.  More accurate than mean.",
            },
        );
        m.insert(
            "DoubleGradient",
            HashTypeWrapper {
                hash_type: InnerHashType::DoubleGradient,
                desc: "Gradient but with an extra hash pass.  Slower, but more accurate.",
            },
        );
        m.insert(
            "DCT",
            HashTypeWrapper {
                hash_type: InnerHashType::DCT,
                desc: "Runs a Discrete Cosine Transform.  Slowest, but can detect color changes.",
            },
        );
        m
    };
}

#[derive(Clone, Debug)]
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
        match HASH_TYPES.get(s) {
            Some(wrapper) => Ok(HashType::new(wrapper.hash_type)),
            None => Err(HashTypeError::InvalidHashError { name: s.to_owned() }),
        }
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
    pub fn new(hash_type: InnerHashType) -> HashType {
        HashType {
            hash: hash_type,
            name: format!("{:?}", hash_type),
        }
    }

    pub fn available_methods() -> Vec<(&'static str, &'static str)> {
        HASH_TYPES.iter().map(|(k, v)| (*k, v.desc)).collect()
    }
}

struct HashTypeWrapper {
    hash_type: InnerHashType,
    desc: &'static str,
}
