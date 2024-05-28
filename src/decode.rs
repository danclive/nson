//! Decode

use core::fmt;

use alloc::format;
use alloc::string::{FromUtf8Error, String};

#[cfg(feature = "std")]
use std::io::{self, Cursor, Read};

#[cfg(not(feature = "std"))]
use crate::io::{self, Cursor, Read};

#[cfg(feature = "serde")]
use crate::serde::decode::Decoder;
#[cfg(feature = "serde")]
use serde::de::Deserialize;

use crate::array::Array;
use crate::id::Id;
use crate::map::Map;
use crate::spec::ElementType;
use crate::value::{Binary, Value};

#[derive(Debug)]
pub enum DecodeError {
    IoError(io::Error),
    FromUtf8Error(FromUtf8Error),
    UnrecognizedElementType(u8),
    InvalidLength(usize, String),
    Unknown(String),
    #[cfg(feature = "serde")]
    Serde(crate::serde::DecodeError),
}

impl From<io::Error> for DecodeError {
    fn from(err: io::Error) -> DecodeError {
        DecodeError::IoError(err)
    }
}

impl From<FromUtf8Error> for DecodeError {
    fn from(err: FromUtf8Error) -> DecodeError {
        DecodeError::FromUtf8Error(err)
    }
}

#[cfg(feature = "serde")]
impl From<crate::serde::DecodeError> for DecodeError {
    fn from(err: crate::serde::DecodeError) -> DecodeError {
        DecodeError::Serde(err)
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            #[cfg(feature = "std")]
            DecodeError::IoError(ref inner) => inner.fmt(fmt),
            #[cfg(not(feature = "std"))]
            DecodeError::IoError(ref inner) => write!(fmt, "{:?}", inner),
            DecodeError::FromUtf8Error(ref inner) => inner.fmt(fmt),
            DecodeError::UnrecognizedElementType(tag) => {
                write!(fmt, "Unrecognized element type `{}`", tag)
            }
            DecodeError::InvalidLength(ref len, ref desc) => {
                write!(fmt, "Expecting length {}, {}", len, desc)
            }
            DecodeError::Unknown(ref inner) => inner.fmt(fmt),
            #[cfg(feature = "serde")]
            DecodeError::Serde(ref inner) => inner.fmt(fmt),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            DecodeError::IoError(ref inner) => Some(inner),
            _ => None,
        }
    }
}

pub type DecodeResult<T> = Result<T, DecodeError>;

#[inline]
pub(crate) fn read_u8(reader: &mut impl Read) -> DecodeResult<u8> {
    let mut buf = [0; 1];
    reader.read_exact(&mut buf)?;
    Ok(u8::from_le_bytes(buf))
}

#[inline]
pub(crate) fn read_i32(reader: &mut impl Read) -> DecodeResult<i32> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    Ok(i32::from_le_bytes(buf))
}

#[inline]
pub(crate) fn read_i64(reader: &mut impl Read) -> DecodeResult<i64> {
    let mut buf = [0; 8];
    reader.read_exact(&mut buf)?;
    Ok(i64::from_le_bytes(buf))
}

#[inline]
pub(crate) fn read_u32(reader: &mut impl Read) -> DecodeResult<u32> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

#[inline]
pub(crate) fn read_u64(reader: &mut impl Read) -> DecodeResult<u64> {
    let mut buf = [0; 8];
    reader.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}

#[inline]
pub(crate) fn read_f32(reader: &mut impl Read) -> DecodeResult<f32> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    Ok(f32::from_le_bytes(buf))
}

#[inline]
pub(crate) fn read_f64(reader: &mut impl Read) -> DecodeResult<f64> {
    let mut buf = [0; 8];
    reader.read_exact(&mut buf)?;
    Ok(f64::from_le_bytes(buf))
}

pub(crate) fn read_string(reader: &mut impl Read) -> DecodeResult<String> {
    let len = read_u32(reader)?;

    if len < crate::MIN_NSON_SIZE - 1 {
        return Err(DecodeError::InvalidLength(
            len as usize,
            format!("Invalid string length of {}", len),
        ));
    }

    if len > crate::MAX_NSON_SIZE {
        return Err(DecodeError::InvalidLength(
            len as usize,
            format!("Invalid string length of {}", len),
        ));
    }

    let len = len - 4;

    let mut buf = alloc::vec![0u8; len as usize];
    reader.read_exact(&mut buf)?;

    let s = String::from_utf8(buf)?;

    Ok(s)
}

pub(crate) fn read_binary(reader: &mut impl Read) -> DecodeResult<Binary> {
    let len = read_u32(reader)?;

    if len < crate::MIN_NSON_SIZE - 1 {
        return Err(DecodeError::InvalidLength(
            len as usize,
            format!("Invalid binary length of {}", len),
        ));
    }

    if len > crate::MAX_NSON_SIZE {
        return Err(DecodeError::InvalidLength(
            len as usize,
            format!("Invalid binary length of {}", len),
        ));
    }

    let len = len - 4;

    let mut data = alloc::vec![0u8; len as usize];
    reader.read_exact(&mut data)?;

    Ok(Binary(data))
}

pub(crate) fn decode_array(reader: &mut impl Read) -> DecodeResult<Array> {
    let mut arr = Array::new();

    let len = read_u32(reader)?;

    if len < crate::MIN_NSON_SIZE {
        return Err(DecodeError::InvalidLength(
            len as usize,
            format!("Invalid array length of {}", len),
        ));
    }

    if len > crate::MAX_NSON_SIZE {
        return Err(DecodeError::InvalidLength(
            len as usize,
            format!("Invalid array length of {}", len),
        ));
    }

    loop {
        let tag = read_u8(reader)?;
        if tag == 0 {
            break;
        }

        let val = decode_value_with_tag(reader, tag)?;
        arr.push(val)
    }

    Ok(arr)
}

pub(crate) fn decode_map(reader: &mut impl Read) -> DecodeResult<Map> {
    let mut map = Map::new();

    // disregard the length: using Read::take causes infinite type recursion
    let len = read_u32(reader)?;

    if len < crate::MIN_NSON_SIZE {
        return Err(DecodeError::InvalidLength(
            len as usize,
            format!("Invalid map length of {}", len),
        ));
    }

    if len > crate::MAX_NSON_SIZE {
        return Err(DecodeError::InvalidLength(
            len as usize,
            format!("Invalid map length of {}", len),
        ));
    }

    loop {
        let key = {
            let len = read_u8(reader)?;
            if len == 0 {
                break;
            }

            let len = len - 1;

            let mut buf = alloc::vec![0u8; len as usize];
            reader.read_exact(&mut buf)?;

            String::from_utf8(buf)?
        };

        let val = decode_value(reader)?;

        map.insert(key, val);
    }

    Ok(map)
}

pub fn decode_value(reader: &mut impl Read) -> DecodeResult<Value> {
    let tag = read_u8(reader)?;
    decode_value_with_tag(reader, tag)
}

fn decode_value_with_tag(reader: &mut impl Read, tag: u8) -> DecodeResult<Value> {
    match ElementType::from(tag) {
        Some(ElementType::F32) => read_f32(reader).map(Value::F32),
        Some(ElementType::F64) => read_f64(reader).map(Value::F64),
        Some(ElementType::I32) => read_i32(reader).map(Value::I32),
        Some(ElementType::I64) => read_i64(reader).map(Value::I64),
        Some(ElementType::U32) => read_u32(reader).map(Value::U32),
        Some(ElementType::U64) => read_u64(reader).map(Value::U64),
        Some(ElementType::String) => read_string(reader).map(Value::String),
        Some(ElementType::Map) => decode_map(reader).map(Value::Map),
        Some(ElementType::Array) => decode_array(reader).map(Value::Array),
        Some(ElementType::Binary) => read_binary(reader).map(Value::Binary),
        Some(ElementType::Bool) => Ok(Value::Bool(read_u8(reader)? != 0)),
        Some(ElementType::Null) => Ok(Value::Null),
        Some(ElementType::TimeStamp) => read_u64(reader).map(|v| Value::TimeStamp(v.into())),
        Some(ElementType::Id) => {
            let mut buf = [0; 12];
            reader.read_exact(&mut buf)?;

            Ok(Value::Id(Id::with_bytes(buf)))
        }
        None => Err(DecodeError::UnrecognizedElementType(tag)),
    }
}

#[cfg(feature = "serde")]
pub fn from_nson<'de, T>(value: Value) -> DecodeResult<T>
where
    T: Deserialize<'de>,
{
    let de = Decoder::new(value);
    Deserialize::deserialize(de).map_err(DecodeError::Serde)
}

#[cfg(feature = "serde")]
pub fn from_bytes<'de, T>(bytes: &[u8]) -> DecodeResult<T>
where
    T: Deserialize<'de>,
{
    let value = Value::from_bytes(bytes)?;
    from_nson(value)
}

impl Value {
    pub fn from_bytes(bytes: &[u8]) -> DecodeResult<Value> {
        let mut reader = Cursor::new(bytes);
        decode_value(&mut reader)
    }
}

impl Map {
    pub fn from_bytes(slice: &[u8]) -> DecodeResult<Map> {
        let mut reader = Cursor::new(slice);
        decode_map(&mut reader)
    }
}

impl Array {
    pub fn from_bytes(slice: &[u8]) -> DecodeResult<Array> {
        let mut reader = Cursor::new(slice);
        decode_array(&mut reader)
    }
}
