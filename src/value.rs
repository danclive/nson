use std::fmt;
use std::ops::{Deref, DerefMut};
use std::{f64, i64};
use std::iter::FromIterator;
use std::convert::Into;

use chrono::{DateTime, Utc, Timelike};
use chrono::offset::TimeZone;

use crate::message::Message;
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
    TimeStamp(i64),
    UTCDatetime(DateTime<Utc>),
    MessageId(MessageId)
}

impl Eq for Value {}

#[derive(Clone, PartialEq, Default)]
pub struct Array {
    inner: Vec<Value>
}

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
                let time = (t >> 32) as i32;
                let inc = (t & 0x00FF_FFFF) as i32;

                write!(fmt, "TimeStamp({}, {})", time, inc)
            },
            Value::UTCDatetime(u) => write!(fmt, "UTCDatetime({:?})", u),
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
                let time = (t >> 32) as i32;
                let inc = (t & 0x00FF_FFFF) as i32;

                write!(fmt, "TimeStamp({}, {})", time, inc)
            },
            Value::UTCDatetime(u) => write!(fmt, "Date({})", u),
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

impl From<DateTime<Utc>> for Value {
    fn from(d: DateTime<Utc>) -> Value {
        Value::UTCDatetime(d)
    }
}

impl From<[u8; 12]> for Value {
    fn from(o: [u8; 12]) -> Value {
        Value::MessageId(MessageId::with_bytes(o))
    }
}

impl From<MessageId> for Value {
    fn from(o: MessageId) -> Value {
        Value::MessageId(o)
    }
}


impl From<Vec<Vec<u8>>> for Value {
    fn from(vec: Vec<Vec<u8>>) -> Value {
        let array: Array = vec.into_iter().map(Into::into).collect();
        Value::Array(array)
    }
}

impl Value {
    pub fn element_type(&self) -> ElementType {
        match *self {
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
            Value::UTCDatetime(..) => ElementType::UTCDatetime,
            Value::MessageId(..) => ElementType::MessageId
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        match *self {
            Value::F32(ref v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Value::F64(ref v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match *self {
            Value::I32(ref v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        match *self {
            Value::U32(ref v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Value::I64(ref v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            Value::U64(ref v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Value::String(ref s) => Some(s),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Array> {
        match *self {
            Value::Array(ref v) => Some(v),
            _ => None,
        }
    }

    pub fn as_message(&self) -> Option<&Message> {
        match *self {
            Value::Message(ref v) => Some(v),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Boolean(ref v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_message_id(&self) -> Option<&MessageId> {
        match *self {
            Value::MessageId(ref v) => Some(v),
            _ => None,
        }
    }

    pub fn as_utc_date_time(&self) -> Option<&DateTime<Utc>> {
        match *self {
            Value::UTCDatetime(ref v) => Some(v),
            _ => None,
        }
    }

    pub fn as_timestamp(&self) -> Option<i64> {
        match *self {
            Value::TimeStamp(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_null(&self) -> Option<()> {
        match *self {
            Value::Null => Some(()),
            _ => None,
        }
    }

    pub fn to_extended_message(&self) -> Message {
        match *self {
            Value::Binary(ref v) => {
                msg!{
                    "$binary": v.to_hex()
                }
            }
            Value::TimeStamp(v) => {
                let time = (v >> 32) as i32;
                let inc = (v & 0xFFFF_FFFF) as i32;
                msg!{
                    "t": time,
                    "i": inc
                }
            }
            Value::UTCDatetime(ref v) => {
                msg!{
                    "$date": {
                        "$numberLong": v.timestamp() * 1000 + i64::from(v.nanosecond()) / 1_000_000
                    }
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
            if let (Ok(t), Ok(i)) = (values.get_i32("t"), values.get_i32("i")) {
                let timestamp = (i64::from(t) << 32) + i64::from(i);
                return Value::TimeStamp(timestamp);

            } else if let (Ok(t), Ok(i)) = (values.get_i64("t"), values.get_i64("i")) {
                let timestamp = (t << 32) + i;
                return Value::TimeStamp(timestamp);

            }

        } else if values.len() == 1 {
            if let Ok(hex) = values.get_str("$binary") {
                return Value::Binary(FromHex::from_hex(hex.as_bytes()).unwrap());
            } else if let Ok(long) = values.get_message("$date").and_then(|inner| inner.get_i64("$numberLong")) {
                return Value::UTCDatetime(Utc.timestamp(long / 1000, ((long % 1000) * 1_000_000) as u32));
            } else if let Ok(hex) = values.get_str("$id") {
                return Value::MessageId(MessageId::with_string(hex).unwrap());
            }
        }

        Value::Message(values)
    }
}

impl Array {
    pub fn new() -> Array {
        Array {
            inner: Vec::new()
        }
    }

    pub fn with_capacity(capacity: usize) -> Array {
        Array {
            inner: Vec::with_capacity(capacity)
        }
    }

    pub fn from_vec(vec: Vec<Value>) -> Array {
        Array {
            inner: vec
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn push(&mut self, value: Value) {
        self.inner.push(value);
    }

    pub fn inner(&self) -> &Vec<Value> {
        &self.inner
    }

    pub fn as_mut_inner(&mut self) -> &mut Vec<Value> {
        &mut self.inner
    }

    pub fn into_inner(self) -> Vec<Value> {
        self.inner
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Value> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Value> {
        self.into_iter()
    }
}

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl Deref for Array {
    type Target = Vec<Value>;
    fn deref(&self) -> &Vec<Value> {
        &self.inner
    }
}

impl DerefMut for Array {
    fn deref_mut(&mut self) -> &mut Vec<Value> {
        &mut self.inner
    }
}

macro_rules! from_impls {
    ($($T:ty)+) => {
        $(
            impl From<Vec<$T>> for Array {
                fn from(vec: Vec<$T>) -> Array {
                    vec.into_iter().map(Into::into).collect()
                }
            }
        )+
    }
}

from_impls! {
    f32 f64 i32 i64 u32 u64 &str String &String Array
    Message bool DateTime<Utc> Vec<u8> Vec<Vec<u8>>
}

impl IntoIterator for Array {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Array {
    type Item = &'a Value;
    type IntoIter = std::slice::Iter<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut Array {
    type Item = &'a mut Value;
    type IntoIter = std::slice::IterMut<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

impl FromIterator<Value> for Array {
    fn from_iter<I: IntoIterator<Item=Value>>(iter: I) -> Self {
        let mut array = Array::new();

        for i in iter {
            array.push(i);
        }

        array
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
