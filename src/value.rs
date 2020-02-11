use std::fmt;
use std::{f64, i64};
use std::convert::Into;

use crate::message::Message;
use crate::array::Array;
use crate::spec::ElementType;
use crate::util::hex::{ToHex, FromHex};
use crate::message_id::MessageId;
use crate::msg;

#[derive(Clone, PartialEq)]
pub enum Value {
    F32(f32),
    F64(f64),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    String(String),
    Array(Array),
    Message(Message),
    Boolean(bool),
    Null,
    Binary(Vec<u8>),
    TimeStamp(TimeStamp),
    MessageId(MessageId)
}

impl Eq for Value {}

impl fmt::Debug for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::F32(f) => write!(fmt, "F32({:?})", f),
            Value::F64(f) => write!(fmt, "F64({:?})", f),
            Value::I32(i) => write!(fmt, "I32({:?})", i),
            Value::I64(i) => write!(fmt, "I64({:?})", i),
            Value::U32(u) => write!(fmt, "U32({:?})", u),
            Value::U64(u) => write!(fmt, "U64({:?})", u),
            Value::String(ref s) => write!(fmt, "String({:?})", s),
            Value::Array(ref vec) => write!(fmt, "Array({:?})", vec),
            Value::Message(ref o) => write!(fmt, "{:?}", o),
            Value::Boolean(b) => write!(fmt, "Boolean({:?})", b),
            Value::Null => write!(fmt, "Null"),
            Value::Binary(ref vec) => write!(fmt, "BinData(0x{})", vec.to_hex()),
            Value::TimeStamp(t) => {
                write!(fmt, "TimeStamp({})", t.0)
            },
            Value::MessageId(ref id) => write!(fmt, "MessageId({})", id),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::F32(f) => write!(fmt, "{}", f),
            Value::F64(f) => write!(fmt, "{}", f),
            Value::I32(i) => write!(fmt, "{}", i),
            Value::I64(i) => write!(fmt, "{}", i),
            Value::U32(u) => write!(fmt, "{}", u),
            Value::U64(u) => write!(fmt, "{}", u),
            Value::String(ref s) => write!(fmt, "\"{}\"", s),
            Value::Array(ref vec) => {
                write!(fmt, "[")?;

                let mut first = true;
                for value in vec.iter() {
                    if !first {
                        write!(fmt, ", ")?;
                    }

                    write!(fmt, "{}", value)?;
                    first = false;
                }

                write!(fmt, "]")
            },
            Value::Message(ref o) => write!(fmt, "{}", o),
            Value::Boolean(b) => write!(fmt, "{}", b),
            Value::Null => write!(fmt, "null"),
            Value::Binary(ref vec) => write!(fmt, "BinData(0x{})", vec.to_hex()),
            Value::TimeStamp(t) => {
                write!(fmt, "TimeStamp({})", t.0)
            },
            Value::MessageId(ref id) => write!(fmt, "MessageId(\"{}\")", id),
        }
    }
}

impl From<f32> for Value {
    fn from(f: f32) -> Value {
        Value::F32(f)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Value {
        Value::F64(f)
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Value {
        Value::I32(i)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Value {
        Value::I64(i)
    }
}

impl From<u32> for Value {
    fn from(u: u32) -> Value {
        Value::U32(u)
    }
}

impl From<u64> for Value {
    fn from(u: u64) -> Value {
        Value::U64(u)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(s: &str) -> Value {
        Value::String(s.to_owned())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Value {
        Value::String(s)
    }
}

impl<'a> From<&'a String> for Value {
    fn from(s: &'a String) -> Value {
        Value::String(s.to_owned())
    }
}

impl From<Array> for Value {
    fn from(a: Array) -> Value {
        Value::Array(a)
    }
}

impl From<Message> for Value {
    fn from(d: Message) -> Value {
        Value::Message(d)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Value {
        Value::Boolean(b)
    }
}

impl From<Vec<u8>> for Value {
    fn from(b: Vec<u8>) -> Value {
        Value::Binary(b)
    }
}

impl From<[u8; 16]> for Value {
    fn from(o: [u8; 16]) -> Value {
        Value::MessageId(MessageId::with_bytes(o))
    }
}

impl From<TimeStamp> for Value {
    fn from(t: TimeStamp) -> Self {
        Value::TimeStamp(t)
    }
}

impl From<MessageId> for Value {
    fn from(o: MessageId) -> Value {
        Value::MessageId(o)
    }
}

impl<'a> From<&'a MessageId> for Value {
    fn from(o: &'a MessageId) -> Value {
        Value::MessageId(o.to_owned())
    }
}

macro_rules! value_from_impls {
    ($($T:ty)+) => {
        $(
            impl From<Vec<$T>> for Value {
                fn from(vec: Vec<$T>) -> Value {
                    Value::Array(vec.into())
                }
            }
        )+
    }
}

value_from_impls! {
    f32 f64 i32 i64 &str String &String Array
    Message bool Vec<u8> MessageId
}

impl Value {
    pub fn element_type(&self) -> ElementType {
        match self {
            Value::F32(..) => ElementType::F32,
            Value::F64(..) => ElementType::F64,
            Value::I32(..) => ElementType::I32,
            Value::I64(..) => ElementType::I64,
            Value::U32(..) => ElementType::U32,
            Value::U64(..) => ElementType::U64,
            Value::String(..) => ElementType::String,
            Value::Array(..) => ElementType::Array,
            Value::Message(..) => ElementType::Message,
            Value::Boolean(..) => ElementType::Boolean,
            Value::Null => ElementType::Null,
            Value::Binary(..) => ElementType::Binary,
            Value::TimeStamp(..) => ElementType::TimeStamp,
            Value::MessageId(..) => ElementType::MessageId
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Value::F32(ref v) => Some(*v),
            _ => None
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::F64(ref v) => Some(*v),
            _ => None
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Value::I32(ref v) => Some(*v),
            _ => None
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Value::U32(ref v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::I64(ref v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::U64(ref v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(ref s) => Some(s),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Array> {
        match self {
            Value::Array(ref v) => Some(v),
            _ => None,
        }
    }

    pub fn as_message(&self) -> Option<&Message> {
        match self {
            Value::Message(ref v) => Some(v),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(ref v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_message_id(&self) -> Option<&MessageId> {
        match self {
            Value::MessageId(ref v) => Some(v),
            _ => None,
        }
    }

    pub fn as_timestamp(&self) -> Option<TimeStamp> {
        match self {
            Value::TimeStamp(v) => Some(*v),
            _ => None
        }
    }

    pub fn as_null(&self) -> Option<()> {
        match self {
            Value::Null => Some(()),
            _ => None
        }
    }

    pub fn as_binary(&self) -> Option<&[u8]> {
        match self {
            Value::Binary(b) => Some(&b),
            _ => None
        }
    }

    pub fn to_extended_message(&self) -> Message {
        match self {
            Value::Binary(ref v) => {
                msg!{
                    "$binary": v.to_hex()
                }
            }
            Value::TimeStamp(v) => {
                let time = (v.0 >> 32) as u32;
                let inc = (v.0 & 0xFFFF_FFFF) as u32;
                msg!{
                    "t": time,
                    "i": inc
                }
            }
            Value::MessageId(ref v) => {
                msg!{
                    "$id": v.to_string()
                }
            }
            _ => panic!("Attempted conversion of invalid data type: {}", self)
        }
    }

    pub fn from_extended_message(values: Message) -> Value {
        if values.len() == 2 {
            if let (Ok(t), Ok(i)) = (values.get_u32("t"), values.get_u32("i")) {
                let timestamp = (u64::from(t) << 32) + u64::from(i);
                return Value::TimeStamp(timestamp.into());

            } else if let (Ok(t), Ok(i)) = (values.get_u64("t"), values.get_u64("i")) {
                let timestamp = (t << 32) + i;
                return Value::TimeStamp(timestamp.into());

            } else if let (Ok(t), Ok(i)) = (values.get_i32("t"), values.get_i32("i")) {
                let timestamp = (i64::from(t) << 32) + i64::from(i);
                return Value::TimeStamp((timestamp as u64).into());

            } else if let (Ok(t), Ok(i)) = (values.get_i64("t"), values.get_i64("i")) {
                let timestamp = (t << 32) + i;
                return Value::TimeStamp((timestamp as u64).into());

            }

        } else if values.len() == 1 {
            if let Ok(hex) = values.get_str("$binary") {
                return Value::Binary(FromHex::from_hex(hex.as_bytes()).unwrap());
            } else if let Ok(hex) = values.get_str("$id") {
                return Value::MessageId(MessageId::with_string(hex).unwrap());
            }
        }

        Value::Message(values)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct TimeStamp(pub u64);

impl From<u64> for TimeStamp {
    fn from(v: u64) -> Self {
        TimeStamp(v)
    }
}
