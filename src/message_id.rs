//! MessageId
use std::sync::atomic::{AtomicU16, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{io, fmt, result, error};

use byteorder::{ByteOrder, BigEndian};

use rand::{self, thread_rng, Rng};

use crate::util::hex::{ToHex, FromHex, FromHexError};

static mut IDENTIFY_BYTES: Option<[u8; 4]> = None;
static COUNTER: AtomicU16 = AtomicU16::new(0);

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MessageId {
    bytes: [u8; 16]
}

pub type Result<T> = result::Result<T, Error>;

impl MessageId {
    /// Generate a new MessageId
    /// 
    /// # Examples
    ///
    /// ```
    /// use nson::message_id::MessageId;
    ///
    /// let id = MessageId::new();
    ///
    /// println!("{:?}", id);
    /// ```
    pub fn new() -> MessageId {
        let timestamp = timestamp();
        let counter = gen_count();
        let identify_bytes = identify_bytes();
        let random_bytes = random_bytes();

        let mut bytes: [u8; 16] = [0; 16];

        bytes[0..6].clone_from_slice(&timestamp[2..]);
        bytes[6..8].clone_from_slice(&counter);
        bytes[8..12].clone_from_slice(&identify_bytes);
        bytes[12..].clone_from_slice(&random_bytes);

        MessageId { bytes }
    }

    /// Generate an MessageId with bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use nson::message_id::MessageId;
    ///
    /// let id = MessageId::with_bytes([1, 111, 157, 189, 157, 247, 247, 220, 156, 134, 213, 115, 239, 90, 50, 156]);
    ///
    /// assert_eq!(format!("{}", id), "016f9dbd9df7f7dc9c86d573ef5a329c")
    /// ```
    pub fn with_bytes(bytes: [u8; 16]) -> Self {
        MessageId { bytes }
    }

    /// Generate an MessageId with string.
    /// Provided string must be a 12-byte hexadecimal string
    ///
    /// # Examples
    ///
    /// ```
    /// use nson::message_id::MessageId;
    ///
    /// let id = MessageId::with_string("016f9dbd9df7f7dc9c86d573ef5a329c").unwrap();
    ///
    /// assert_eq!(format!("{}", id), "016f9dbd9df7f7dc9c86d573ef5a329c")
    /// ```
    pub fn with_string(str: &str) -> Result<MessageId> {
        let bytes: Vec<u8> = FromHex::from_hex(str.as_bytes())?;
        if bytes.len() != 16 {
            return Err(Error::ArgumentError("Provided string must be a 16-byte hexadecimal string.".to_string()))
        }

        let mut buf = [0u8; 16];
        buf[..].copy_from_slice(&bytes);

        Ok(MessageId {
            bytes: buf
        })
    }

    /// 16-byte binary representation of this MessageId.
    pub fn bytes(&self) -> [u8; 16] {
        self.bytes
    }

    /// Timstamp of this MessageId
    pub fn timestamp(&self) -> u64 {
        BigEndian::read_u48(&self.bytes)
    }

    pub fn counter(&self) -> u16 {
        BigEndian::read_u16(&self.bytes[6..8])
    }

    pub fn identify(&self) -> u32 {
        BigEndian::read_u32(&self.bytes[8..12])
    }

    /// Convert this MessageId to a 16-byte hexadecimal string.
    pub fn to_hex(&self) -> String {
        self.bytes.to_hex()
    }
}

impl Default for MessageId {
    fn default() -> Self {
        MessageId::new()
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

impl From<[u8; 16]> for MessageId {
    fn from(bytes: [u8; 16]) -> Self {
        MessageId { bytes }
    }
}

#[inline]
fn timestamp() -> [u8; 8] {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!")
        .as_millis() as u64;

    time.to_be_bytes()
}

pub fn set_identify(identify: u32) {
    let bytes = identify.to_be_bytes();
    unsafe {
       IDENTIFY_BYTES = Some(bytes);
    }
}

#[inline]
fn identify_bytes() -> [u8; 4] {
    unsafe {
        if let Some(bytes) = IDENTIFY_BYTES.as_ref() {
            return *bytes;
        }
    }

    let rand_num: u32 = thread_rng().gen();

    let bytes = rand_num.to_be_bytes();

    unsafe {
       IDENTIFY_BYTES = Some(bytes);
    }

    bytes
}

#[inline]
fn random_bytes() -> [u8; 4] {
    let rand_num: u32 = thread_rng().gen();

    rand_num.to_be_bytes()
}

#[inline]
fn gen_count() -> [u8; 2] {
    if COUNTER.load(Ordering::SeqCst) == 0 {
        let start: u16 = thread_rng().gen();
        COUNTER.store(start, Ordering::SeqCst);
    }

    let count = COUNTER.fetch_add(1, Ordering::SeqCst);

    count.to_be_bytes()
}

#[derive(Debug)]
pub enum Error {
    ArgumentError(String),
    FromHexError(FromHexError),
    IoError(io::Error),
    RandError(rand::Error)
}

impl From<FromHexError> for Error {
    fn from(err: FromHexError) -> Error {
        Error::FromHexError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<rand::Error> for Error {
    fn from(err: rand::Error) -> Error {
        Error::RandError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ArgumentError(ref err) => err.fmt(fmt),
            Error::FromHexError(ref err) => err.fmt(fmt),
            Error::IoError(ref inner) => inner.fmt(fmt),
            Error::RandError(ref inner) => inner.fmt(fmt)
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ArgumentError(ref err) => &err,
            Error::FromHexError(ref err) => err.description(),
            Error::IoError(ref err) => err.description(),
            Error::RandError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::ArgumentError(_) => None,
            Error::FromHexError(ref err) => Some(err),
            Error::IoError(ref err) => Some(err),
            Error::RandError(ref err) => Some(err)
        }
    }
}
