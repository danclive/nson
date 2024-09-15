//! Id

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;
use core::str::FromStr;

use const_hex::FromHexError;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Id {
    bytes: [u8; 12],
}

pub type Result<T> = core::result::Result<T, Error>;

// Unique incrementing Id.
//
//   +---+---+---+---+---+---+---+---+---+---+---+---+
//   |       timestamp       | count |    random     |
//   +---+---+---+---+---+---+---+---+---+---+---+---+
//     0   1   2   3   4   5   6   7   8   9   10  11
impl Id {
    /// Generate a new Id
    ///
    /// # Examples
    ///
    /// ```
    /// use nson::id::Id;
    ///
    /// let id = Id::new();
    ///
    /// println!("{:?}", id);
    /// ```
    #[cfg(feature = "std")]
    pub fn new() -> Id {
        let timestamp = use_std::timestamp();
        let counter = use_std::gen_count();
        let random_bytes = use_std::random_bytes();

        let mut bytes: [u8; 12] = [0; 12];

        bytes[..6].copy_from_slice(&timestamp[2..]);
        bytes[6..8].copy_from_slice(&counter);
        bytes[8..].copy_from_slice(&random_bytes);

        Id::with_bytes(bytes)
    }

    /// Generate a new Id
    pub fn new_raw(timestamp: u64, count: u16, random: u32) -> Id {
        let mut bytes: [u8; 12] = [0; 12];

        bytes[..6].copy_from_slice(&timestamp.to_be_bytes()[2..]);
        bytes[6..8].copy_from_slice(&count.to_be_bytes());
        bytes[8..].copy_from_slice(&random.to_be_bytes());

        Id { bytes }
    }

    /// Generate an Id with bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use nson::Id;
    ///
    /// let id = Id::with_bytes([1, 111, 157, 189, 157, 247, 247, 220, 156, 134, 213, 115]);
    ///
    /// assert_eq!(format!("{}", id), "016f9dbd9df7f7dc9c86d573")
    /// ```
    pub fn with_bytes(bytes: [u8; 12]) -> Self {
        Id { bytes }
    }

    /// Generate an Id with string.
    /// Provided string must be a 12-byte hexadecimal string
    ///
    /// # Examples
    ///
    /// ```
    /// use nson::Id;
    ///
    /// let id = Id::with_string("016f9dbd9df7f7dc9c86d573").unwrap();
    ///
    /// assert_eq!(format!("{}", id), "016f9dbd9df7f7dc9c86d573")
    /// ```
    pub fn with_string(str: &str) -> Result<Id> {
        let bytes: Vec<u8> = const_hex::decode(str)?;
        if bytes.len() != 12 {
            return Err(Error::ArgumentError(
                "Provided string must be a 12-byte hexadecimal string.".to_string(),
            ));
        }

        let mut buf = [0u8; 12];
        buf[..].copy_from_slice(&bytes);

        Ok(Id { bytes: buf })
    }

    /// 12-byte binary representation of this Id.
    pub fn bytes(&self) -> [u8; 12] {
        self.bytes
    }

    /// Timstamp of this Id
    pub fn timestamp(&self) -> u64 {
        let mut buf = [0u8; 8];
        buf[2..8].copy_from_slice(&self.bytes[..6]);
        u64::from_be_bytes(buf)
    }

    /// Convert this Id to a 16-byte hexadecimal string.
    pub fn to_hex(&self) -> String {
        const_hex::encode(self.bytes)
    }

    pub fn zero() -> Id {
        Id {
            bytes: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    pub fn is_zero(&self) -> bool {
        self == &Id::zero()
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("Id({})", self.to_hex()))
    }
}

impl From<[u8; 12]> for Id {
    fn from(bytes: [u8; 12]) -> Self {
        Id { bytes }
    }
}

impl FromStr for Id {
    type Err = Error;

    fn from_str(s: &str) -> Result<Id> {
        Self::with_string(s)
    }
}

#[cfg(feature = "std")]
impl Default for Id {
    fn default() -> Self {
        Id::new()
    }
}

#[derive(Debug)]
pub enum Error {
    ArgumentError(String),
    FromHexError(FromHexError),
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
            Error::FromHexError(ref err) => err.fmt(fmt),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Error::ArgumentError(_) => None,
            Error::FromHexError(ref err) => Some(err),
        }
    }
}

#[cfg(feature = "std")]
mod use_std {
    use std::sync::atomic::{AtomicU16, Ordering};
    use std::sync::LazyLock;
    use std::time::{SystemTime, UNIX_EPOCH};

    use rand::{self, thread_rng, Rng};

    static COUNTER: LazyLock<AtomicU16> = LazyLock::new(|| AtomicU16::new(thread_rng().gen()));

    #[inline]
    pub fn timestamp() -> [u8; 8] {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_millis() as u64;

        time.to_be_bytes()
    }

    #[inline]
    pub fn gen_count() -> [u8; 2] {
        let count = COUNTER.fetch_add(1, Ordering::SeqCst);

        count.to_be_bytes()
    }

    #[inline]
    pub fn random_bytes() -> [u8; 4] {
        let rand_num: u32 = thread_rng().gen();

        rand_num.to_be_bytes()
    }
}
