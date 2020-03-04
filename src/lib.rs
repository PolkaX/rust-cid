//! # cid
//!
//! Implementation of [cid](https://github.com/ipld/cid) in Rust.

#![deny(missing_docs)]

mod cid;
mod codec;
mod error;
mod version;

#[cfg(any(test, feature = "test"))]
mod arb;

pub use self::cid::{Cid, CidGeneric};
pub use self::codec::Codec;
pub use self::error::{Error, Result};
pub use self::version::Version;

#[cfg(any(feature = "cbor", feature = "json"))]
mod serde;

#[cfg(feature = "cbor")]
pub use self::serde::{IPLD_DAG_CBOR_TAG_CID, RAW_BINARY_MULTIBASE_IDENTITY};
