//! Encode

use core::fmt;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::io::{self, Write};

#[cfg(not(feature = "std"))]
use crate::io::{self, Write};

#[cfg(feature = "serde")]
use crate::serde::encode::Encoder;
#[cfg(feature = "serde")]
use serde::ser::Serialize;

use crate::array::Array;
use crate::map::Map;
use crate::value::{Binary, Value};

#[derive(Debug)]
#[non_exhaustive]
pub enum EncodeError {
    IoError(io::Error),
    InvalidKeyLen(usize, String),
    Unknown(String),
    #[cfg(feature = "serde")]
    Serde(crate::serde::EncodeError),
}

impl From<io::Error> for EncodeError {
    fn from(err: io::Error) -> EncodeError {
        EncodeError::IoError(err)
    }
}

#[cfg(feature = "serde")]
impl From<crate::serde::EncodeError> for EncodeError {
    fn from(err: crate::serde::EncodeError) -> EncodeError {
        EncodeError::Serde(err)
    }
}

impl fmt::Display for EncodeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            #[cfg(feature = "std")]
            EncodeError::IoError(ref inner) => inner.fmt(fmt),
            #[cfg(not(feature = "std"))]
            EncodeError::IoError(ref inner) => write!(fmt, "{:?}", inner),
            EncodeError::InvalidKeyLen(ref len, ref desc) => {
                write!(fmt, "Invalid key len: {}, {}", len, desc)
            }
            EncodeError::Unknown(ref inner) => inner.fmt(fmt),
            #[cfg(feature = "serde")]
            EncodeError::Serde(ref inner) => inner.fmt(fmt),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for EncodeError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            EncodeError::IoError(ref inner) => Some(inner),
            _ => None,
        }
    }
}

pub type EncodeResult<T> = Result<T, EncodeError>;

#[inline]
pub(crate) fn write_i32(writer: &mut impl Write, val: i32) -> EncodeResult<()> {
    writer.write_all(&val.to_le_bytes()).map_err(From::from)
}

#[inline]
pub(crate) fn write_u32(writer: &mut impl Write, val: u32) -> EncodeResult<()> {
    writer.write_all(&val.to_le_bytes()).map_err(From::from)
}

#[inline]
pub(crate) fn write_i64(writer: &mut impl Write, val: i64) -> EncodeResult<()> {
    writer.write_all(&val.to_le_bytes()).map_err(From::from)
}

#[inline]
pub(crate) fn write_u64(writer: &mut impl Write, val: u64) -> EncodeResult<()> {
    writer.write_all(&val.to_le_bytes()).map_err(From::from)
}

#[inline]
pub(crate) fn write_f32(writer: &mut impl Write, val: f32) -> EncodeResult<()> {
    writer.write_all(&val.to_le_bytes()).map_err(From::from)
}

#[inline]
pub(crate) fn write_f64(writer: &mut impl Write, val: f64) -> EncodeResult<()> {
    writer.write_all(&val.to_le_bytes()).map_err(From::from)
}

pub(crate) fn write_key(writer: &mut impl Write, s: &str) -> EncodeResult<()> {
    if s.is_empty() || s.len() >= 255 {
        return Err(EncodeError::InvalidKeyLen(
            s.len(),
            "key len must > 0 and < 255".to_string(),
        ));
    }

    writer.write_all(&[s.len() as u8 + 1])?;
    writer.write_all(s.as_bytes())?;
    Ok(())
}

pub(crate) fn write_string(writer: &mut impl Write, s: &str) -> EncodeResult<()> {
    write_u32(writer, s.len() as u32 + 4)?;
    writer.write_all(s.as_bytes())?;
    Ok(())
}

pub(crate) fn write_binary(writer: &mut impl Write, binary: &Binary) -> EncodeResult<()> {
    write_u32(writer, binary.0.len() as u32 + 4)?;
    writer.write_all(&binary.0)?;
    Ok(())
}

pub(crate) fn encode_array(writer: &mut impl Write, array: &Array) -> EncodeResult<()> {
    let len = array.bytes_size();

    write_u32(writer, len as u32)?;

    for val in array.iter() {
        encode_value(writer, val)?;
    }

    writer.write_all(&[0])?;

    Ok(())
}

pub(crate) fn encode_map(writer: &mut impl Write, map: &Map) -> EncodeResult<()> {
    let len = map.bytes_size();

    write_u32(writer, len as u32)?;

    for (key, val) in map {
        write_key(writer, key)?;

        encode_value(writer, val)?;
    }

    writer.write_all(&[0])?;

    Ok(())
}

pub fn encode_value(writer: &mut impl Write, val: &Value) -> EncodeResult<()> {
    writer.write_all(&[val.element_type() as u8])?;

    match *val {
        Value::F32(v) => write_f32(writer, v),
        Value::F64(v) => write_f64(writer, v),
        Value::I32(v) => write_i32(writer, v),
        Value::I64(v) => write_i64(writer, v),
        Value::U32(v) => write_u32(writer, v),
        Value::U64(v) => write_u64(writer, v),
        Value::String(ref s) => write_string(writer, s),
        Value::Array(ref a) => encode_array(writer, a),
        Value::Map(ref o) => encode_map(writer, o),
        Value::Bool(b) => writer
            .write_all(&[if b { 0x01 } else { 0x00 }])
            .map_err(From::from),
        Value::Null => Ok(()),
        Value::Binary(ref binary) => write_binary(writer, binary),
        Value::TimeStamp(v) => write_u64(writer, v.0),
        Value::Id(ref id) => writer.write_all(&id.bytes()).map_err(From::from),
    }
}

impl Value {
    pub fn to_bytes(&self) -> EncodeResult<Vec<u8>> {
        let mut buf = Vec::new();
        encode_value(&mut buf, self)?;
        Ok(buf)
    }
}

impl Map {
    pub fn to_bytes(&self) -> EncodeResult<Vec<u8>> {
        let len = self.bytes_size();

        let mut buf = Vec::with_capacity(len);
        write_u32(&mut buf, len as u32)?;

        for (key, val) in self {
            write_key(&mut buf, key)?;

            encode_value(&mut buf, val)?;
        }

        buf.write_all(&[0])?;

        Ok(buf)
    }
}

impl Array {
    pub fn to_bytes(&self) -> EncodeResult<Vec<u8>> {
        let len = self.bytes_size();

        let mut buf = Vec::with_capacity(len);
        write_u32(&mut buf, len as u32)?;

        for val in self.iter() {
            encode_value(&mut buf, val)?;
        }

        buf.write_all(&[0])?;

        Ok(buf)
    }
}

#[cfg(feature = "serde")]
pub fn to_nson<T: ?Sized>(value: &T) -> EncodeResult<Value>
where
    T: Serialize,
{
    let ser = Encoder::new();
    value.serialize(ser).map_err(EncodeError::Serde)
}

#[cfg(feature = "serde")]
pub fn to_bytes<T: ?Sized>(value: &T) -> EncodeResult<Vec<u8>>
where
    T: Serialize,
{
    let value = to_nson(value)?;
    value.to_bytes()
}

#[cfg(test)]
mod test {
    use crate::decode::decode_map;
    use crate::encode::encode_map;
    use crate::m;

    use alloc::vec::Vec;

    #[cfg(feature = "std")]
    use std::io::Cursor;

    #[cfg(not(feature = "std"))]
    use crate::io::Cursor;

    #[test]
    fn encode() {
        let m = m! {"aa": "bb", "cc": [1, 2, 3, 4]};

        let mut buf: Vec<u8> = Vec::new();

        encode_map(&mut buf, &m).unwrap();

        let mut reader = Cursor::new(buf);

        let m2 = decode_map(&mut reader).unwrap();

        assert_eq!(m, m2);
    }
}
