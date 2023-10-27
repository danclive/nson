use std::io::{Write, Read};

use crate::encode::{encode_array, EncodeResult};
use crate::decode::{decode_array, DecodeResult};

pub use crate::core::array::*;

impl Array {
    pub fn encode(&self, writer: &mut impl Write) -> EncodeResult<()> {
        encode_array(writer, self)
    }

    pub fn decode(reader: &mut impl Read) -> DecodeResult<Array> {
        decode_array(reader)
    }
}

#[cfg(test)]
mod test {
    use crate::Array;

    #[test]
    fn to_vec() {
        let mut array = Array::new();

        array.push(123);
        array.push("haha");

        let vec = array.to_bytes().unwrap();

        let array2 = Array::from_bytes(&vec).unwrap();

        assert_eq!(array, array2);
    }
}
