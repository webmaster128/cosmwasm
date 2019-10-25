// This file simply re-exports some methods from serde_json
// The reason is two fold:
// 1. To easily ensure that all calling libraries use the same version (minimize code size)
// 2. To allow us to switch out to eg. serde-json-core more easily

use heapless::consts::U2048;
use std::vec::Vec;

use crate::errors::{Error, ParseErr, SerializeErr};

pub fn from_slice<'a, T>(v: &'a [u8]) -> Result<T, Error>
    where
        T: serde::de::Deserialize<'a> {
    let res = core_serde_json::from_slice(v);
    match res {
        Err(_) => ParseErr{}.fail(),
        Ok(v) => Ok(v),
    }
}

/// Serializes the given data structure as a JSON byte vector
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, Error>
    where
        T: serde::ser::Serialize + ?Sized,
{
    let res = core_serde_json::to_vec::<U2048, T>(value);
    match res {
        Ok(v) => {
            let s: &[u8] = &v;
            Ok(Vec::from(s))
        },
        Err(_) => SerializeErr{}.fail(),
    }
}

