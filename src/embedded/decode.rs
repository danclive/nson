use core::fmt;

use alloc::format;
use alloc::string::{String, FromUtf8Error};
use alloc::vec::Vec;

use embedded_io::{Read, ReadExactError};

use super::cursor::Cursor;
use crate::core::spec::ElementType;
use crate::core::{Value, Array, Map, Binary, Id};

#[derive(Debug)]
pub enum DecodeError<E: embedded_io::Error> {
    ReadError(E),
    ReadExactError(ReadExactError<E>),
    FromUtf8Error(FromUtf8Error),
    UnrecognizedElementType(u8),
    InvalidLength(usize, String),
    Unknown(String),
    #[cfg(feature = "serde")]
    Serde(crate::serde::DecodeError)
}

impl<E: embedded_io::Error> From<E> for DecodeError<E> {
    fn from(err: E) -> DecodeError<E> {
        DecodeError::ReadError(err)
    }
}

impl<E: embedded_io::Error> From<ReadExactError<E>> for DecodeError<E> {
    fn from(err: ReadExactError<E>) -> DecodeError<E> {
        DecodeError::ReadExactError(err)
    }
}

// impl<E: embedded_io::Error> From<FromUtf8Error> for DecodeError<E> {
//     fn from(err: FromUtf8Error) -> DecodeError<E> {
//         DecodeError::FromUtf8Error(err)
//     }
// }

#[cfg(feature = "serde")]
impl<E: embedded_io::Error> From<crate::serde::DecodeError> for DecodeError<E> {
    fn from(err: crate::serde::DecodeError) -> DecodeError<E> {
        DecodeError::Serde(err)
    }
}


impl<E: embedded_io::Error> fmt::Display for DecodeError<E> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecodeError::ReadError(ref inner) => inner.fmt(fmt),
            DecodeError::ReadExactError(ref inner) => inner.fmt(fmt),
            DecodeError::FromUtf8Error(ref inner) => inner.fmt(fmt),
            DecodeError::UnrecognizedElementType(tag) => {
                write!(fmt, "Unrecognized element type `{}`", tag)
            }
            DecodeError::InvalidLength(ref len, ref desc) => {
                write!(fmt, "Expecting length {}, {}", len, desc)
            }
            DecodeError::Unknown(ref inner) => inner.fmt(fmt),
            DecodeError::Serde(ref inner) => inner.fmt(fmt),
        }
    }
}

impl<E: embedded_io::Error> embedded_io::Error for DecodeError<E> {
    fn kind(&self) -> embedded_io::ErrorKind {
        match *self {
            DecodeError::ReadError(ref inner) => inner.kind(),
            _ => embedded_io::ErrorKind::Other
        }
    }
}

pub type DecodeResult<T, E> = Result<T, DecodeError<E>>;

#[inline]
pub(crate) fn read_u8<R: Read>(reader: &mut R) -> DecodeResult<u8, R::Error> {
    let mut buf = [0; 1];
    reader.read_exact(&mut buf)?;
    Ok(u8::from_le_bytes(buf))
}

#[inline]
pub(crate) fn read_i32<R: Read>(reader: &mut R) -> DecodeResult<i32, R::Error> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    Ok(i32::from_le_bytes(buf))
}

#[inline]
pub(crate) fn read_i64<R: Read>(reader: &mut R) -> DecodeResult<i64, R::Error> {
    let mut buf = [0; 8];
    reader.read_exact(&mut buf)?;
    Ok(i64::from_le_bytes(buf))
}

#[inline]
pub(crate) fn read_u32<R: Read>(reader: &mut R) -> DecodeResult<u32, R::Error> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

#[inline]
pub(crate) fn read_u64<R: Read>(reader: &mut R) -> DecodeResult<u64, R::Error> {
    let mut buf = [0; 8];
    reader.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}

#[inline]
pub(crate) fn read_f32<R: Read>(reader: &mut R) -> DecodeResult<f32, R::Error> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    Ok(f32::from_le_bytes(buf))
}

#[inline]
pub(crate) fn read_f64<R: Read>(reader: &mut R) -> DecodeResult<f64, R::Error> {
    let mut buf = [0; 8];
    reader.read_exact(&mut buf)?;
    Ok(f64::from_le_bytes(buf))
}

pub(crate) fn read_string<R: Read>(reader: &mut R) -> DecodeResult<String, R::Error> {
    let len = read_u32(reader)?;

    if len < crate::MIN_NSON_SIZE - 1 {
        return Err(DecodeError::InvalidLength(len as usize, format!("Invalid string length of {}", len)));
    }

    if len > crate::MAX_NSON_SIZE {
        return Err(DecodeError::InvalidLength(len as usize, format!("Invalid string length of {}", len)));
    }

    let len = len - 4;

    let mut buf = Vec::with_capacity(len as usize);
    reader.read_exact(&mut buf)?;

    let s = String::from_utf8(buf).map_err(DecodeError::FromUtf8Error)?;

    Ok(s)
}

pub(crate) fn read_binary<R: Read>(reader: &mut R) -> DecodeResult<Binary, R::Error> {
    let len = read_u32(reader)?;

    if len < crate::MIN_NSON_SIZE - 1 {
        return Err(DecodeError::InvalidLength(len as usize, format!("Invalid binary length of {}", len)));
    }

    if len > crate::MAX_NSON_SIZE {
        return Err(DecodeError::InvalidLength(len as usize, format!("Invalid binary length of {}", len)));
    }

    let len = len - 4;

    let mut buf = Vec::with_capacity(len as usize);
    reader.read_exact(&mut buf)?;

    Ok(Binary(buf))
}

pub(crate) fn decode_array<R: Read>(reader: &mut R) -> DecodeResult<Array, R::Error> {
    let mut arr = Array::new();

    let len = read_u32(reader)?;

    if len < crate::MIN_NSON_SIZE {
        return Err(DecodeError::InvalidLength(len as usize, format!("Invalid array length of {}", len)));
    }

    if len > crate::MAX_NSON_SIZE {
        return Err(DecodeError::InvalidLength(len as usize, format!("Invalid array length of {}", len)));
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

pub(crate) fn decode_map<R: Read>(reader: &mut R) -> DecodeResult<Map, R::Error> {
    let mut map = Map::new();

    // disregard the length: using Read::take causes infinite type recursion
    let len = read_u32(reader)?;

    if len < crate::MIN_NSON_SIZE {
        return Err(DecodeError::InvalidLength(len as usize, format!("Invalid map length of {}", len)));
    }

    if len > crate::MAX_NSON_SIZE {
        return Err(DecodeError::InvalidLength(len as usize, format!("Invalid map length of {}", len)));
    }

    loop {
        let key = {
            let len = read_u8(reader)?;
            if len == 0 {
                break;
            }

            let len = len - 1;

            let mut buf = Vec::with_capacity(len as usize);
            reader.read_exact(&mut buf)?;

            let s = String::from_utf8(buf).map_err(DecodeError::FromUtf8Error)?;
            s
        };

        let val = decode_value(reader)?;

        map.insert(key, val);
    }

    Ok(map)
}

pub fn decode_value<R: Read>(reader: &mut R) -> DecodeResult<Value, R::Error> {
    let tag = read_u8(reader)?;
    decode_value_with_tag(reader, tag)
}

fn decode_value_with_tag<R: Read>(reader: &mut R, tag: u8) -> DecodeResult<Value, R::Error> {
    match ElementType::from(tag) {
        Some(ElementType::F32) => {
            read_f32(reader).map(Value::F32)
        }
        Some(ElementType::F64) => {
            read_f64(reader).map(Value::F64)
        }
        Some(ElementType::I32) => {
            read_i32(reader).map(Value::I32)
        }
        Some(ElementType::I64) => {
            read_i64(reader).map(Value::I64)
        }
        Some(ElementType::U32) => {
            read_u32(reader).map(Value::U32)
        }
        Some(ElementType::U64) => {
            read_u64(reader).map(Value::U64)
        }
        Some(ElementType::String) => {
            read_string(reader).map(Value::String)
        }
        Some(ElementType::Map) => {
            decode_map(reader).map(Value::Map)
        }
        Some(ElementType::Array) => {
            decode_array(reader).map(Value::Array)
        }
        Some(ElementType::Binary) => {
            read_binary(reader).map(Value::Binary)
        }
        Some(ElementType::Bool) => {
            Ok(Value::Bool(read_u8(reader)? != 0))
        }
        Some(ElementType::Null) => {
            Ok(Value::Null)
        }
        Some(ElementType::TimeStamp) => {
            read_u64(reader).map(|v| Value::TimeStamp(v.into()))
        }
        Some(ElementType::Id) => {
            let mut buf = [0; 12];
            reader.read_exact(&mut buf)?;

            Ok(Value::Id(Id::with_bytes(buf)))
        }
        None => {
            Err(DecodeError::UnrecognizedElementType(tag))
        }
    }
}

impl Value {
    pub fn from_bytes(bytes: &[u8]) -> DecodeResult<Value, core::convert::Infallible> {
        let mut reader = Cursor::new(bytes);
        decode_value(&mut reader)
    }
}

impl Map {
    pub fn from_bytes(slice: &[u8]) -> DecodeResult<Map, core::convert::Infallible> {
        let mut reader = Cursor::new(slice);
        decode_map(&mut reader)
    }
}

impl Array {
    pub fn from_bytes(slice: &[u8]) -> DecodeResult<Array, core::convert::Infallible> {
        let mut reader = Cursor::new(slice);
        decode_array(&mut reader)
    }
}

#[cfg(feature = "serde")]
use serde::de::Deserialize;
#[cfg(feature = "serde")]
use crate::serde::decode::Decoder;

#[cfg(feature = "serde")]
pub fn from_nson<'de, T, E: embedded_io::Error>(value: Value) -> DecodeResult<T, E>
    where T: Deserialize<'de>
{
    let de = Decoder::new(value);
    Deserialize::deserialize(de).map_err(DecodeError::Serde)
}

#[cfg(feature = "serde")]
pub fn from_bytes<'de, T, E: embedded_io::Error>(bytes: &[u8]) -> DecodeResult<T, core::convert::Infallible>
    where T: Deserialize<'de>
{
    let value = Value::from_bytes(bytes)?;
    from_nson(value)
}
