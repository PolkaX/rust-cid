use std::convert::TryFrom;

use minicbor::data::Tag;
use minicbor::decode::{self, Decoder};
use minicbor::encode::{self, Encoder};

use crate::cid::Cid;

/// Raw binary multibase identity
pub const RAW_BINARY_MULTIBASE_IDENTITY: u8 = 0;
/// The specific CBOR tag for IPLD DagCBOR serialization/deserialization
pub const IPLD_DAG_CBOR_TAG_CID: u64 = 42;

impl encode::Encode for Cid {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        let mut bytes = self.to_bytes();
        bytes.insert(0, RAW_BINARY_MULTIBASE_IDENTITY);
        e.tag(Tag::Unassigned(IPLD_DAG_CBOR_TAG_CID))?
            .bytes(&bytes)?
            .ok()
    }
}

impl<'b> decode::Decode<'b> for Cid {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        match d.tag()? {
            Tag::Unassigned(IPLD_DAG_CBOR_TAG_CID) => {
                let bytes = d.bytes()?;
                if bytes.is_empty() || bytes[0] != RAW_BINARY_MULTIBASE_IDENTITY {
                    return Err(decode::Error::Message(
                        "raw binary multibase identity 0x00 must not be omitted",
                    ));
                }
                Ok(Cid::try_from(&bytes[1..])
                    .map_err(|_| decode::Error::Message("expected cid bytes"))?)
            }
            _ => Err(decode::Error::TypeMismatch(
                IPLD_DAG_CBOR_TAG_CID as u8,
                "expected tag 42",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cid::Cid;
    use crate::codec::Codec;
    use crate::version::Version;

    #[test]
    fn serde_for_cid_v0() {
        let cid = "Qmf5Qzp6nGBku7CEn2UQx4mgN8TW69YUok36DrGa6NN893"
            .parse::<Cid>()
            .unwrap();
        assert_eq!(cid.version(), Version::V0);
        assert_eq!(cid.codec(), Codec::DagProtobuf);
        assert_eq!(
            cid.hash().to_vec(),
            vec![
                18, 32, 248, 175, 118, 33, 111, 145, 175, 205, 162, 241, 159, 194, 73, 247, 191,
                123, 200, 8, 195, 247, 188, 251, 25, 128, 235, 202, 135, 150, 161, 75, 202, 70
            ]
        );

        let cbor = minicbor::to_vec(&cid).unwrap();
        assert_eq!(
            cbor,
            vec![
                216, 42, 88, 35, 0, 18, 32, 248, 175, 118, 33, 111, 145, 175, 205, 162, 241, 159,
                194, 73, 247, 191, 123, 200, 8, 195, 247, 188, 251, 25, 128, 235, 202, 135, 150,
                161, 75, 202, 70
            ]
        );

        let out: Cid = minicbor::decode(&cbor).unwrap();
        assert_eq!(out, cid);
    }

    #[test]
    fn serde_for_cid_v1() {
        let cid = "bafkreie5qrjvaw64n4tjm6hbnm7fnqvcssfed4whsjqxzslbd3jwhsk3mm"
            .parse::<Cid>()
            .unwrap();
        assert_eq!(cid.version(), Version::V1);
        assert_eq!(cid.codec(), Codec::Raw);
        assert_eq!(
            cid.hash().to_vec(),
            vec![
                18, 32, 157, 132, 83, 80, 91, 220, 111, 38, 150, 120, 225, 107, 62, 86, 194, 162,
                148, 138, 65, 242, 199, 146, 97, 124, 201, 97, 30, 211, 99, 201, 91, 99
            ]
        );

        let cbor = minicbor::to_vec(&cid).unwrap();
        assert_eq!(
            cbor,
            vec![
                216, 42, 88, 37, 0, 1, 85, 18, 32, 157, 132, 83, 80, 91, 220, 111, 38, 150, 120,
                225, 107, 62, 86, 194, 162, 148, 138, 65, 242, 199, 146, 97, 124, 201, 97, 30, 211,
                99, 201, 91, 99
            ]
        );

        let out: Cid = minicbor::decode(&cbor).unwrap();
        assert_eq!(out, cid);
    }
}
