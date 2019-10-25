extern crate alloc;

use alloc::string::String;
use snafu_derive::{Snafu};


#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("Received null pointer, refuse to use"))]
    NullPointer {
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
//    #[snafu(display("Parse error: {}", source))]
    #[snafu(display("Parse error"))]
    ParseErr {
//        source: core_serde_json::de::Error,
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
    //    #[snafu(display("Serialization error: {}", source))]
    #[snafu(display("Serialization error"))]
    SerializeErr {
//        source: core_serde_json::ser::Error,
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Contract error: {}", msg))]
    ContractErr {
        msg: String,
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Unauthorized"))]
    Unauthorized {
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
}

pub type Result<T, E = Error> = core::result::Result<T, E>;