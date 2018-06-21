use hash_type;
use img_hash::HashType as InnerHashType;
use std::collections::HashMap;
use std::str::FromStr;

lazy_static! {
    static ref HASH_TYPES: HashMap<&'static str, hash_type::HashTypeWrapper> = {
        let mut m = HashMap::new();
        m.insert(
            "block",
            HashTypeWrapper {
                hash_type: InnerHashType::Block,
                desc: "The Blockhash.io algorithm.  Fastest, but also inaccurate.",
            },
        );
        m.insert(
            "mean",
            HashTypeWrapper {
                hash_type: InnerHashType::Mean,
                desc: "Averages pixels.  Fast, but inaccurate unless looking for exact duplicates.",
            },
        );
        m.insert(
            "gradient",
            HashTypeWrapper {
                hash_type: InnerHashType::Gradient,
                desc: "Compares edges and color boundaries.  More accurate than mean.",
            },
        );
        m.insert(
            "doublegradient",
            HashTypeWrapper {
                hash_type: InnerHashType::DoubleGradient,
                desc: "Gradient but with an extra hash pass.  Slower, but more accurate.",
            },
        );
        m.insert(
            "dct",
            HashTypeWrapper {
                hash_type: InnerHashType::DCT,
                desc: "Runs a Discrete Cosine Transform.  Slowest, but can detect color changes.",
            },
        );
        m
    };
}

#[derive(Clone, Copy, Debug)]
pub struct HashType(InnerHashType);

#[derive(Debug, Fail)]
pub enum HashTypeError {
    #[fail(display = "Failure to parse: {}", name)]
    InvalidHashError { name: String },
}

impl FromStr for HashType {
    type Err = HashTypeError;
    //TODO: Create own parse errors
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match HASH_TYPES.get(s) {
            Some(wrapper) => Ok(HashType(wrapper.hash_type)),
            None => Err(HashTypeError::InvalidHashError { name: s.to_owned() }),
        }
    }
}

impl Into<InnerHashType> for HashType {
    fn into(self) -> InnerHashType {
        self.0
    }
}

impl HashType {
    pub fn available_methods() -> Vec<(&'static str, &'static str)> {
        HASH_TYPES.iter().map(|(k, v)| (*k, v.desc)).collect()
    }
}

struct HashTypeWrapper {
    hash_type: InnerHashType,
    desc: &'static str,
}
