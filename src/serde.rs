// This file simply re-exports some methods from serde_json
// The reason is two fold:
// 1. To easily ensure that all calling libraries use the same version (minimize code size)
// 2. To allow us to switch out to eg. serde-json-core more easily

use std::vec::Vec;

use crates::errors::{Error, ParseErr, SerializeErr};

pub fn from_slice<'a, T>(v: &'a [u8]) -> Result<T, Error>
    where
        T: core_serde_json::de::Deserialize<'a> {
    let res = core_serde_json::from_slice(v);
    if let Err(e) = res {
        return ParseErr{}.fail();
    }
    res
}

/// Serializes the given data structure as a JSON byte vector
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, Error>
    where
        T: core_serde_json::ser::Serialize + ?Sized,
{
    let res = core_serde_json::to_vec(value);
    match res {
        Ok(v) => Ok(Vec::from(&v)),
        Err(e) => SerializeErr{}.fail(),
    }
}

