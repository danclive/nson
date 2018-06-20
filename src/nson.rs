use std::fmt;
use std::ops::{Deref, DerefMut};
use std::hash::{Hash, Hasher};

use chrono::{DateTime, Utc, Timelike};
use chrono::offset::TimeZone;

use object::Object;
use spec::ElementType;
use util::hex::{ToHex, FromHex};
use encode;

#[derive(Clone, PartialEq)]
pub enum Nson {
    Double(f64),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    String(String),
    Array(Array),
    Object(Object),
    Boolean(bool),
    Null,
    Binary(Vec<u8>),
    TimeStamp(i64),
    UTCDatetime(DateTime<Utc>),
}

impl Eq for Nson {}

pub type Array = Vec<Nson>;

impl fmt::Debug for Nson {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Nson::Double(d) => write!(fmt, "Double({:?})", d),
            Nson::I32(i) => write!(fmt, "I32({:?})", i),
            Nson::I64(i) => write!(fmt, "I64({:?})", i),
            Nson::U32(u) => write!(fmt, "U32({:?})", u),
            Nson::U64(u) => write!(fmt, "U64({:?})", u),
            Nson::String(ref s) => write!(fmt, "String({:?})", s),
            Nson::Array(ref vec) => write!(fmt, "Array({:?})", vec),
            Nson::Object(ref o) => write!(fmt, "{:?}", o),
            Nson::Boolean(b) => write!(fmt, "Boolean({:?})", b),
            Nson::Null => write!(fmt, "Null"),
            Nson::Binary(ref vec) => write!(fmt, "BinData(0x{})", vec.to_hex()),
            Nson::TimeStamp(t) => {
                let time = (t >> 32) as i32;
                let inc = (t & 0xFFFFFF) as i32;

                write!(fmt, "TimeStamp({}, {})", time, inc)
            },
            Nson::UTCDatetime(u) => write!(fmt, "UTCDatetime({:?})", u)
        }
    }
}

impl fmt::Display for Nson {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Nson::Double(d) => write!(fmt, "{}", d),
            Nson::I32(i) => write!(fmt, "{}", i),
            Nson::I64(i) => write!(fmt, "{}", i),
            Nson::U32(u) => write!(fmt, "{}", u),
            Nson::U64(u) => write!(fmt, "{}", u),
            Nson::String(ref s) => write!(fmt, "\"{}\"", s),
            Nson::Array(ref vec) => {
                write!(fmt, "[")?;

                let mut first = true;
                for nson in vec.iter() {
                    if !first {
                        write!(fmt, ", ")?;
                    }

                    write!(fmt, "{}", nson)?;
                    first = false;
                }

                write!(fmt, "]")
            },
            Nson::Object(ref o) => write!(fmt, "{}", o),
            Nson::Boolean(b) => write!(fmt, "{}", b),
            Nson::Null => write!(fmt, "null"),
            Nson::Binary(ref vec) => write!(fmt, "BinData(0x{})", vec.to_hex()),
            Nson::TimeStamp(t) => {
                let time = (t >> 32) as i32;
                let inc = (t & 0xFFFFFF) as i32;

                write!(fmt, "TimeStamp({}, {})", time, inc)
            },
            Nson::UTCDatetime(u) => write!(fmt, "Date({})", u)
        }
    }
}

impl From<f32> for Nson {
    fn from(f: f32) -> Nson {
        Nson::Double(f as f64)
    }
}

impl From<f64> for Nson {
    fn from(f: f64) -> Nson {
        Nson::Double(f)
    }
}

impl From<i32> for Nson {
    fn from(i: i32) -> Nson {
        Nson::I32(i)
    }
}

impl From<i64> for Nson {
    fn from(i: i64) -> Nson {
        Nson::I64(i)
    }
}

impl From<u32> for Nson {
    fn from(u: u32) -> Nson {
        Nson::U32(u)
    }
}

impl From<u64> for Nson {
    fn from(u: u64) -> Nson {
        Nson::U64(u)
    }
}

impl<'a> From<&'a str> for Nson {
    fn from(s: &str) -> Nson {
        Nson::String(s.to_owned())
    }
}

impl From<String> for Nson {
    fn from(s: String) -> Nson {
        Nson::String(s)
    }
}

impl<'a> From<&'a String> for Nson {
    fn from(s: &'a String) -> Nson {
        Nson::String(s.to_owned())
    }
}

impl From<Array> for Nson {
    fn from(a: Array) -> Nson {
        Nson::Array(a)
    }
}

impl From<Object> for Nson {
    fn from(d: Object) -> Nson {
        Nson::Object(d)
    }
}

impl From<bool> for Nson {
    fn from(b: bool) -> Nson {
        Nson::Boolean(b)
    }
}

impl From<Vec<u8>> for Nson {
    fn from(b: Vec<u8>) -> Nson {
        Nson::Binary(b)
    }
}

impl From<DateTime<Utc>> for Nson {
    fn from(d: DateTime<Utc>) -> Nson {
        Nson::UTCDatetime(d)
    }
}

impl Nson {
    pub fn element_type(&self) -> ElementType {
        match *self {
            Nson::Double(..) => ElementType::Double,
            Nson::I32(..) => ElementType::I32,
            Nson::I64(..) => ElementType::I64,
            Nson::U32(..) => ElementType::U32,
            Nson::U64(..) => ElementType::U64,
            Nson::String(..) => ElementType::String,
            Nson::Array(..) => ElementType::Array,
            Nson::Object(..) => ElementType::Object,
            Nson::Boolean(..) => ElementType::Boolean,
            Nson::Null => ElementType::Null,
            Nson::Binary(..) => ElementType::Binary,
            Nson::TimeStamp(..) => ElementType::TimeStamp,
            Nson::UTCDatetime(..) => ElementType::UTCDatetime
        }
    }

    pub fn to_extended_object(&self) -> Object {
        match *self {
            Nson::Binary(ref v) => {
                object! {
                    "$binary": v.to_hex()
                }
            }
            Nson::TimeStamp(v) => {
                let time = (v >> 32) as i32;
                let inc = (v & 0xFFFFFFFF) as i32;

                object! {
                    "t": time,
                    "i": inc
                }
            }
            Nson::UTCDatetime(ref v) => {
                object! {
                    "$date": {
                        "$numberLong": (v.timestamp() * 1000) + v.nanosecond() as i64 / 1000000
                    }
                }
            }
            _ => panic!("Attempted conversion of invalid data type: {}", self)
        }
    }

    pub fn from_extended_object(values: Object) -> Nson {
        if values.len() == 2 {
            if let (Ok(t), Ok(i)) = (values.get_i32("t"), values.get_i32("i")) {
                let timestamp = ((t as i64) << 32) + (i as i64);
                return Nson::TimeStamp(timestamp);

            } else if let (Ok(t), Ok(i)) = (values.get_i64("t"), values.get_i64("i")) {
                let timestamp = (t << 32) + i;
                return Nson::TimeStamp(timestamp);

            }

        } else if values.len() == 1 {
            if let Ok(hex) = values.get_str("$binary") {
                return Nson::Binary(FromHex::from_hex(hex.as_bytes()).unwrap());
            } else if let Ok(long) = values.get_object("$date").and_then(|inner| inner.get_i64("$numberLong")) {
                return Nson::UTCDatetime(Utc.timestamp(long / 1000, ((long % 1000) * 1000000) as u32));
            }
        }

        Nson::Object(values)
    }
}

impl Hash for Nson {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut buf: Vec<u8> = Vec::new();
        let _ = encode::encode_nson(&mut buf, "", self);
        buf.hash(state);
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct UTCDateTime(pub DateTime<Utc>);

impl Deref for UTCDateTime {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UTCDateTime {
    fn deref_mut(&mut self) -> &mut DateTime<Utc> {
        &mut self.0
    }
}

impl Into<DateTime<Utc>> for UTCDateTime {
    fn into(self) -> DateTime<Utc> {
        self.0
    }
}

impl From<DateTime<Utc>> for UTCDateTime {
    fn from(x: DateTime<Utc>) -> Self {
        UTCDateTime(x)
    }
}
