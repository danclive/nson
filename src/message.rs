use std::io::{Write, Read};

use crate::encode::{encode_message, EncodeResult};
use crate::decode::{decode_message, DecodeResult};

pub use crate::core::message::*;

impl Message {
    pub fn encode(&self, writer: &mut impl Write) -> EncodeResult<()> {
        encode_message(writer, self)
    }

    pub fn decode(reader: &mut impl Read) -> DecodeResult<Message> {
        decode_message(reader)
    }
}

#[cfg(test)]
mod test {
    use crate::Message;
    use crate::msg;

    #[test]
    fn to_vec() {
        let msg = msg!{"aa": "bb"};

        let vec = msg.to_bytes().unwrap();

        let msg2 = Message::from_bytes(&vec).unwrap();

        assert_eq!(msg, msg2);
    }

    #[test]
    fn extend() {
        let msg1 = msg!{"aa": "bb"};

        let mut msg2 = msg!{"cc": "dd"};
        msg2.extend(msg1);

        let msg3 = msg!{"aa": "bb", "cc": "dd"};

        assert_eq!(msg2, msg3);
    }
}
