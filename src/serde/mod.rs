/// IPLD DagCBOR serialization/deserialization.
/// See the [link](https://github.com/ipld/specs/blob/master/block-layer/codecs/dag-cbor.md#link-format) for details.
///
/// As with all IPLD formats, DagCBOR must be able to encode [Links](https://github.com/ipld/specs/blob/master/data-model-layer/data-model.md#link-kind).
/// In DagCBOR, links are CIDs encoded using the raw-binary identity Multibase.
/// That Multibase prefix (0x00) must not be omitted.
/// They are stored as byte-string type (major type 2), with the tag 42.
#[cfg(feature = "cbor")]
mod cbor;

#[cfg(feature = "cbor")]
pub use self::cbor::{IPLD_DAG_CBOR_TAG_CID, RAW_BINARY_MULTIBASE_IDENTITY};

/// IPLD DagJSON serialization/deserialization.
#[cfg(feature = "json")]
mod json;
