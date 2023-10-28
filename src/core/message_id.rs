use core::fmt;
use core::str::FromStr;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

use hex::FromHexError;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MessageId {
    bytes: [u8; 12]
}

pub type Result<T> = core::result::Result<T, Error>;

// Unique incrementing MessageId.
//
//   +---+---+---+---+---+---+---+---+---+---+---+---+
//   |       timestamp       | count |    random     |
//   +---+---+---+---+---+---+---+---+---+---+---+---+
//     0   1   2   3   4   5   6   7   8   9   10  11
impl MessageId {
    /// Generate a new MessageId
    pub fn new_raw(timestamp: u64, count: u16, random: u32) -> MessageId {
        let mut bytes: [u8; 12] = [0; 12];

        bytes[..6].copy_from_slice(&timestamp.to_be_bytes()[2..]);
        bytes[6..8].copy_from_slice(&count.to_be_bytes());
        bytes[8..].copy_from_slice(&random.to_be_bytes());

        MessageId { bytes }
    }

    /// Generate an MessageId with bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use nson::core::message_id::MessageId;
    ///
    /// let id = MessageId::with_bytes([1, 111, 157, 189, 157, 247, 247, 220, 156, 134, 213, 115]);
    ///
    /// assert_eq!(format!("{}", id), "016f9dbd9df7f7dc9c86d573")
    /// ```
    pub fn with_bytes(bytes: [u8; 12]) -> Self {
        MessageId { bytes }
    }

    /// Generate an MessageId with string.
    /// Provided string must be a 12-byte hexadecimal string
    ///
    /// # Examples
    ///
    /// ```
    /// use nson::core::message_id::MessageId;
    ///
    /// let id = MessageId::with_string("016f9dbd9df7f7dc9c86d573").unwrap();
    ///
    /// assert_eq!(format!("{}", id), "016f9dbd9df7f7dc9c86d573")
    /// ```
    pub fn with_string(str: &str) -> Result<MessageId> {
        let bytes: Vec<u8> = hex::decode(str)?;
        if bytes.len() != 12 {
            return Err(Error::ArgumentError("Provided string must be a 12-byte hexadecimal string.".to_string()))
        }

        let mut buf = [0u8; 12];
        buf[..].copy_from_slice(&bytes);

        Ok(MessageId {
            bytes: buf
        })
    }

    /// 12-byte binary representation of this MessageId.
    pub fn bytes(&self) -> [u8; 12] {
        self.bytes
    }

    /// Timstamp of this MessageId
    pub fn timestamp(&self) -> u64 {
        let mut buf = [0u8; 8];
        buf[2..8].copy_from_slice(&self.bytes[..6]);
        u64::from_be_bytes(buf)
    }

    /// Convert this MessageId to a 16-byte hexadecimal string.
    pub fn to_hex(&self) -> String {
        hex::encode(self.bytes)
    }

    pub fn zero() -> MessageId {
        MessageId { bytes: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}
    }

    pub fn is_zero(&self) -> bool {
        self == &MessageId::zero()
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl fmt::Debug for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("MessageId({})", self.to_hex()))
    }
}

impl From<[u8; 12]> for MessageId {
    fn from(bytes: [u8; 12]) -> Self {
        MessageId { bytes }
    }
}

impl FromStr for MessageId {
    type Err = Error;

    fn from_str(s: &str) -> Result<MessageId> {
        Self::with_string(s)
    }
}

// static COUNTER: Lazy<AtomicU16> = Lazy::new(|| AtomicU16::new(0));

// #[inline]
// fn gen_count() -> [u8; 2] {
//     let count = COUNTER.fetch_add(1, Ordering::SeqCst);

//     count.to_be_bytes()
// }

#[derive(Debug)]
pub enum Error {
    ArgumentError(String),
    FromHexError(FromHexError)
}

impl From<FromHexError> for Error {
    fn from(err: FromHexError) -> Error {
        Error::FromHexError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ArgumentError(ref err) => err.fmt(fmt),
            Error::FromHexError(ref err) => err.fmt(fmt)
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Error::ArgumentError(_) => None,
            Error::FromHexError(ref err) => Some(err)
        }
    }
}
