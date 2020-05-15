use std::convert::TryFrom;

use multihash::{Code, Multihash, MultihashGeneric, MultihashRefGeneric};

/// Changed `Multihash`, with `ExtCode`.
pub type ExtMultihash = MultihashGeneric<ExtCode>;

/// Changed `MultihashRef`, with `ExtCode`.
pub type ExtMultihashRef<'a> = MultihashRefGeneric<'a, ExtCode>;

/// Same as Into, convert self to a extensional type
pub trait IntoExt<T>: Sized {
    /// Performs the conversion.
    fn into_ext(self) -> T;
}

impl IntoExt<ExtMultihash> for Multihash {
    /// upgrade src `Multihash` to `Multihash` with `ExtCode`
    fn into_ext(self) -> ExtMultihash {
        ExtMultihash::from_bytes(self.to_vec())
            .expect("a valid src `Multihash` to `Multihash` with `ExtCode` be sure to succeed")
    }
}

/// extension for `Code`, provide some new types, e.g. FilecoinMultihashCode
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExtCode {
    /// multihash src `Code`
    NormalCode(Code),
    /// Filecoin proof multihash code
    FL(FilecoinMultihashCode),
}

impl ExtCode {
    /// Hash some input and return the raw binary digest.
    pub fn digest(&self, data: &[u8]) -> ExtMultihash {
        match self {
            ExtCode::NormalCode(code) => code.digest(data).into_ext(),
            ExtCode::FL(fl_code) => fl_code.digest(data),
        }
    }
}

/// Filecoin Multihash proof code
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FilecoinMultihashCode {
    /// FcUnsealedV1 is the v1 hashing algorithm used in
    /// constructing merkleproofs of unsealed data
    FcUnsealedV1 = 0xfc1,
    /// FcSealedV1 is the v1 hashing algorithm used in
    /// constructing merkleproofs of sealed replicated data
    FcSealedV1,
    /// FcReserved3 is reserved for future use
    FcReserved3,
    /// FcReserved4 is reserved for future use
    FcReserved4,
    /// FcReserved5 is reserved for future use
    FcReserved5,
    /// FcReserved6 is reserved for future use
    FcReserved6,
    /// FcReserved7 is reserved for future use
    FcReserved7,
    /// FcReserved8 is reserved for future use
    FcReserved8,
    /// FcReserved9 is reserved for future use
    FcReserved9,
    /// FcReserved10 is reserved for future use
    FcReserved10,
}

impl FilecoinMultihashCode {
    /// Hash some input and return the raw binary digest.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn digest(&self, data: &[u8]) -> ExtMultihash {
        match self {
            FilecoinMultihashCode::FcUnsealedV1 => FilecoinUnsealedV1::digest(data),
            FilecoinMultihashCode::FcSealedV1 => FilecoinSealedV1::digest(data),
            FilecoinMultihashCode::FcReserved3
            | FilecoinMultihashCode::FcReserved4
            | FilecoinMultihashCode::FcReserved5
            | FilecoinMultihashCode::FcReserved6
            | FilecoinMultihashCode::FcReserved7
            | FilecoinMultihashCode::FcReserved8
            | FilecoinMultihashCode::FcReserved9
            | FilecoinMultihashCode::FcReserved10 => multihash::wrap(ExtCode::FL(*self), data),
        }
    }
}

impl TryFrom<u64> for ExtCode {
    type Error = String;
    /// Return the `Code` based on the integer value
    fn try_from(raw: u64) -> Result<Self, Self::Error> {
        use std::mem::transmute;
        let fl_first = FilecoinMultihashCode::FcUnsealedV1 as u64;
        let fl_last = FilecoinMultihashCode::FcReserved10 as u64;
        match raw {
            c if fl_first <= c && c <= fl_last => {
                let fl: FilecoinMultihashCode = unsafe { transmute(c) };
                Ok(Self::FL(fl))
            }
            // Fallback to the default values
            _ => match Code::try_from(raw) {
                Ok(code) => Ok(Self::NormalCode(code)),
                Err(_) => Err("invalid code".to_string()),
            },
        }
    }
}

impl From<ExtCode> for u64 {
    fn from(code: ExtCode) -> Self {
        match code {
            ExtCode::FL(code) => code as u64,
            ExtCode::NormalCode(normal_code) => normal_code.into(),
        }
    }
}

impl TryFrom<ExtCode> for Code {
    type Error = String;
    fn try_from(extended: ExtCode) -> Result<Self, Self::Error> {
        match extended {
            ExtCode::NormalCode(code) => Ok(code),
            _ => Err("Not a default code".to_string()),
        }
    }
}

impl From<Code> for ExtCode {
    fn from(code: Code) -> Self {
        ExtCode::NormalCode(code)
    }
}

/// Filecoin proof, Unsealed v1 hash
#[derive(Clone, Copy, Debug)]
pub struct FilecoinUnsealedV1;
impl FilecoinUnsealedV1 {
    /// The code of the FilecoinUnsealedV1 hasher, 0xfc1.
    pub const CODE: ExtCode = ExtCode::FL(FilecoinMultihashCode::FcUnsealedV1);
    /// Hash a comm ([u8; 32]) and return the raw binary digest.
    pub fn digest(data: &[u8]) -> ExtMultihash {
        multihash::wrap(Self::CODE, data)
    }
}

/// Filecoin proof, Sealed v1 hash
#[derive(Clone, Copy, Debug)]
pub struct FilecoinSealedV1;
impl FilecoinSealedV1 {
    /// The code of the FilecoinUnsealedV1 hasher, 0xfc2.
    pub const CODE: ExtCode = ExtCode::FL(FilecoinMultihashCode::FcSealedV1);
    /// Hash a comm ([u8; 32]) and return the raw binary digest.
    pub fn digest(data: &[u8]) -> ExtMultihash {
        multihash::wrap(Self::CODE, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fl() {
        let h = FilecoinUnsealedV1::digest(&[1_u8; 32][..]);
        assert_eq!(h.digest(), &[1_u8; 32][..]);
        assert_eq!(
            h.algorithm(),
            ExtCode::FL(FilecoinMultihashCode::FcUnsealedV1)
        );
        assert_eq!(
            h.to_vec(),
            vec![
                193, 31, 32, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1
            ]
        );

        let h = FilecoinSealedV1::digest(&[2_u8; 32][..]);
        assert_eq!(h.digest(), &[2_u8; 32][..]);
        assert_eq!(
            h.algorithm(),
            ExtCode::FL(FilecoinMultihashCode::FcSealedV1)
        );
        assert_eq!(
            h.to_vec(),
            vec![
                194, 31, 32, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2
            ]
        );

        let hash: ExtMultihash = multihash::Sha2_256::digest(b"123").into_ext();
        assert_eq!(hash.algorithm(), ExtCode::NormalCode(Code::Sha2_256));
    }
}
