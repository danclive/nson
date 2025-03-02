//! Value

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use core::ops::{Deref, DerefMut};

use super::array::Array;
use super::id::Id;
use super::map::Map;
use super::spec::ElementType;

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
    Map(Map),
    Bool(bool),
    Null,
    Binary(Binary),
    TimeStamp(TimeStamp),
    Id(Id),
}

impl Eq for Value {}

impl fmt::Debug for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::F32(f) => write!(fmt, "F32({:?})", f),
            Value::F64(f) => write!(fmt, "F64({:?})", f),
            Value::I32(i) => write!(fmt, "I32({:?})", i),
            Value::I64(i) => write!(fmt, "I64({:?})", i),
            Value::U32(u) => write!(fmt, "U32({:?})", u),
            Value::U64(u) => write!(fmt, "U64({:?})", u),
            Value::String(s) => write!(fmt, "String({:?})", s),
            Value::Array(vec) => write!(fmt, "Array({:?})", vec),
            Value::Map(o) => write!(fmt, "{:?}", o),
            Value::Bool(b) => write!(fmt, "Bool({:?})", b),
            Value::Null => write!(fmt, "Null"),
            Value::Binary(vec) => write!(fmt, "Binary(0x{})", const_hex::encode(&vec.0)),
            Value::TimeStamp(t) => {
                write!(fmt, "TimeStamp({})", t.0)
            }
            Value::Id(id) => write!(fmt, "Id({})", id),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::F32(f) => write!(fmt, "F32({})", f),
            Value::F64(f) => write!(fmt, "F64({})", f),
            Value::I32(i) => write!(fmt, "I32({})", i),
            Value::I64(i) => write!(fmt, "I64({})", i),
            Value::U32(u) => write!(fmt, "U32({})", u),
            Value::U64(u) => write!(fmt, "U64({})", u),
            Value::String(s) => write!(fmt, "String({})", s),
            Value::Array(vec) => {
                write!(fmt, "Array[")?;

                let mut first = true;
                for value in vec.iter() {
                    if !first {
                        write!(fmt, ", ")?;
                    }

                    write!(fmt, "{}", value)?;
                    first = false;
                }

                write!(fmt, "]")
            }
            Value::Map(o) => write!(fmt, "Map({})", o),
            Value::Bool(b) => write!(fmt, "{}", b),
            Value::Null => write!(fmt, "null"),
            Value::Binary(vec) => write!(fmt, "Binary(0x{})", const_hex::encode(&vec.0)),
            Value::TimeStamp(t) => {
                write!(fmt, "TimeStamp({})", t.0)
            }
            Value::Id(id) => write!(fmt, "Id({})", id),
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

impl From<&str> for Value {
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

impl From<Binary> for Value {
    fn from(b: Binary) -> Value {
        Value::Binary(b)
    }
}

impl<'a> From<&'a Binary> for Value {
    fn from(b: &'a Binary) -> Value {
        Value::Binary(b.to_owned())
    }
}

impl From<Array> for Value {
    fn from(a: Array) -> Value {
        Value::Array(a)
    }
}

impl From<Map> for Value {
    fn from(d: Map) -> Value {
        Value::Map(d)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Value {
        Value::Bool(b)
    }
}

impl From<Vec<u8>> for Value {
    fn from(b: Vec<u8>) -> Value {
        Value::Binary(Binary(b))
    }
}

impl From<[u8; 12]> for Value {
    fn from(o: [u8; 12]) -> Value {
        Value::Id(Id::with_bytes(o))
    }
}

impl From<TimeStamp> for Value {
    fn from(t: TimeStamp) -> Self {
        Value::TimeStamp(t)
    }
}

impl From<Id> for Value {
    fn from(o: Id) -> Value {
        Value::Id(o)
    }
}

impl<'a> From<&'a Id> for Value {
    fn from(o: &'a Id) -> Value {
        Value::Id(o.to_owned())
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(v: Option<T>) -> Value {
        v.map(|v| v.into()).unwrap_or(Value::Null)
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
    Map bool Vec<u8> Id
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
            Value::Map(..) => ElementType::Map,
            Value::Bool(..) => ElementType::Bool,
            Value::Null => ElementType::Null,
            Value::Binary(..) => ElementType::Binary,
            Value::TimeStamp(..) => ElementType::TimeStamp,
            Value::Id(..) => ElementType::Id,
        }
    }

    pub fn bytes_size(&self) -> usize {
        match self {
            Value::F32(_) => 4,
            Value::F64(_) => 8,
            Value::I32(_) => 4,
            Value::I64(_) => 8,
            Value::U32(_) => 4,
            Value::U64(_) => 8,
            Value::String(s) => 4 + s.len(),
            Value::Array(a) => a.bytes_size(),
            Value::Map(m) => m.bytes_size(),
            Value::Bool(_) => 1,
            Value::Null => 0,
            Value::Binary(b) => 4 + b.0.len(),
            Value::TimeStamp(_) => 8,
            Value::Id(_) => 12,
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Value::F32(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::F64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Value::I32(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Value::U32(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::I64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::U64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Array> {
        match self {
            Value::Array(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&Map> {
        match self {
            Value::Map(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_id(&self) -> Option<&Id> {
        match self {
            Value::Id(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_timestamp(&self) -> Option<TimeStamp> {
        match self {
            Value::TimeStamp(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_null(&self) -> Option<()> {
        match self {
            Value::Null => Some(()),
            _ => None,
        }
    }

    pub fn as_binary(&self) -> Option<&Binary> {
        match self {
            Value::Binary(b) => Some(b),
            _ => None,
        }
    }

    #[cfg(feature = "serde")]
    pub(crate) fn to_extended_map(&self) -> Map {
        match self {
            Value::Binary(v) => {
                let mut msg = Map::with_capacity(1);
                msg.insert("$bin", const_hex::encode(&v.0));
                msg
            }
            Value::TimeStamp(v) => {
                let mut msg = Map::with_capacity(1);
                msg.insert("$tim", v.0);
                msg
            }
            Value::Id(v) => {
                let mut msg = Map::with_capacity(1);
                msg.insert("$mid", v.to_hex());
                msg
            }
            _ => panic!("Attempted conversion of invalid data type: {}", self),
        }
    }

    #[cfg(feature = "serde")]
    pub(crate) fn from_extended_map(msg: Map) -> Value {
        if msg.len() == 1 {
            let (key, value) = msg.get_index(0).unwrap();

            match key.as_str() {
                "$bin" => {
                    if let Value::String(hex) = value {
                        if let Ok(bin) = const_hex::decode(hex.as_bytes()) {
                            return Value::Binary(Binary(bin));
                        }
                    }
                }
                "$tim" => {
                    if let Value::U64(u) = value {
                        return Value::TimeStamp((*u).into());
                    }
                }
                "$mid" => {
                    if let Value::String(hex) = value {
                        if let Ok(id) = Id::with_string(hex) {
                            return id.into();
                        }
                    }
                }
                _ => (),
            }
        }

        Value::Map(msg)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct Binary(pub Vec<u8>);

impl From<Vec<u8>> for Binary {
    fn from(v: Vec<u8>) -> Self {
        Binary(v)
    }
}

impl From<Binary> for Vec<u8> {
    fn from(b: Binary) -> Self {
        b.0
    }
}

impl Deref for Binary {
    type Target = Vec<u8>;
    fn deref(&self) -> &Vec<u8> {
        &self.0
    }
}

impl DerefMut for Binary {
    fn deref_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct TimeStamp(pub u64);

impl From<u64> for TimeStamp {
    fn from(v: u64) -> Self {
        TimeStamp(v)
    }
}

impl From<TimeStamp> for u64 {
    fn from(t: TimeStamp) -> Self {
        t.0
    }
}
