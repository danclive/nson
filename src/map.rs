use std::io::{Write, Read};

use crate::encode::{encode_map, EncodeResult};
use crate::decode::{decode_map, DecodeResult};

pub use crate::core::map::*;

impl Map {
    pub fn encode(&self, writer: &mut impl Write) -> EncodeResult<()> {
        encode_map(writer, self)
    }

    pub fn decode(reader: &mut impl Read) -> DecodeResult<Map> {
        decode_map(reader)
    }
}

#[cfg(test)]
mod test {
    use crate::Map;
    use crate::m;

    #[test]
    fn to_vec() {
        let m = m!{"aa": "bb"};

        let vec = m.to_bytes().unwrap();

        let m2 = Map::from_bytes(&vec).unwrap();

        assert_eq!(m, m2);
    }

    #[test]
    fn extend() {
        let m1 = m!{"aa": "bb"};

        let mut m2 = m!{"cc": "dd"};
        m2.extend(m1);

        let m3 = m!{"aa": "bb", "cc": "dd"};

        assert_eq!(m2, m3);
    }
}
