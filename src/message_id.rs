//! MessageId
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{io, fmt, result, error};

use byteorder::{ByteOrder, BigEndian};

use rand::{self, thread_rng, Rng};

use crate::util::hex::{ToHex, FromHex, FromHexError};

static mut MACHINE_BYTES: Option<[u8; 5]> = None;
static OID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MessageId {
    bytes: [u8; 12]
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
        let machine_id = machine_id();
        let counter = gen_count();

        let mut buf: [u8; 12] = [0; 12];

        buf[0] = timestamp[0];
        buf[1] = timestamp[1];
        buf[2] = timestamp[2];
        buf[3] = timestamp[3];

        buf[4] = machine_id[0];
        buf[5] = machine_id[1];
        buf[6] = machine_id[2];
        buf[7] = machine_id[3];
        buf[8] = machine_id[4];


        buf[9] = counter[0];
        buf[10] = counter[1];
        buf[11] = counter[2];

        MessageId {
            bytes: buf
        }
    }

    /// Generate an MessageId with bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use nson::message_id::MessageId;
    ///
    /// let id = MessageId::with_bytes([90, 167, 114, 110, 99, 55, 51, 218, 65, 162, 186, 71]);
    ///
    /// assert_eq!(format!("{}", id), "5aa7726e633733da41a2ba47")
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
    /// use nson::message_id::MessageId;
    ///
    /// let id = MessageId::with_string("5932a005b4b4b4ac168cd9e4").unwrap();
    ///
    /// assert_eq!(format!("{}", id), "5932a005b4b4b4ac168cd9e4")
    /// ```
    pub fn with_string(str: &str) -> Result<MessageId> {
        let bytes: Vec<u8> = FromHex::from_hex(str.as_bytes())?;
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
    pub fn timestamp(&self) -> u32 {
        BigEndian::read_u32(&self.bytes)
    }

    /// Convert this MessageId to a 12-byte hexadecimal string.
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

#[inline]
fn timestamp() -> [u8; 4] {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!")
        .as_secs() as u32;

    let mut buf: [u8; 4] = [0; 4];
    BigEndian::write_u32(&mut buf, time);
    buf
}

#[inline]
fn machine_id() -> [u8; 5] {
    unsafe {
        if let Some(bytes) = MACHINE_BYTES.as_ref() {
            return *bytes;
        }
    }

    let mut buf = [0u8; 5];

    thread_rng().fill(&mut buf);

    unsafe {
        MACHINE_BYTES = Some(buf);
    }

    buf
}

#[inline]
fn gen_count() -> [u8; 3] {

    const MAX_U24: usize = 0x00FF_FFFF;

    if OID_COUNTER.load(Ordering::SeqCst) == 0 {
        let start = thread_rng().gen_range(0, MAX_U24 + 1);
        OID_COUNTER.store(start, Ordering::SeqCst);
    }

    let count = OID_COUNTER.fetch_add(1, Ordering::SeqCst);

    let u = count % MAX_U24;

    let mut buf: [u8; 8] = [0; 8];
    BigEndian::write_u64(&mut buf, u as u64);

    [buf[5], buf[6], buf[7]]
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
