use core::fmt;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use embedded_io::Write;

use crate::core::{Value, Array, Message, Binary};

#[derive(Debug)]
#[non_exhaustive]
pub enum EncodeError<E: embedded_io::Error> {
    WriteError(E),
    InvalidKeyLen(usize, String),
    Unknown(String),
    #[cfg(feature = "serde")]
    Serde(crate::serde::EncodeError)
}

impl<E: embedded_io::Error> From<E> for EncodeError<E> {
    fn from(err: E) -> EncodeError<E> {
        EncodeError::WriteError(err)
    }
}

#[cfg(feature = "serde")]
impl<E: embedded_io::Error> From<crate::serde::EncodeError> for EncodeError<E> {
    fn from(err: crate::serde::EncodeError) -> EncodeError<E> {
        EncodeError::Serde(err)
    }
}

impl<E: embedded_io::Error> fmt::Display for EncodeError<E> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EncodeError::WriteError(ref inner) => inner.fmt(fmt),
            EncodeError::InvalidKeyLen(ref len, ref desc) => {
                write!(fmt, "Invalid key len: {}, {}", len, desc)
            }
            EncodeError::Unknown(ref inner) => inner.fmt(fmt),
            #[cfg(feature = "serde")]
            EncodeError::Serde(ref inner) => inner.fmt(fmt),
        }
    }
}

impl<E: embedded_io::Error> embedded_io::Error for EncodeError<E> {
    fn kind(&self) -> embedded_io::ErrorKind {
        match *self {
            EncodeError::WriteError(ref inner) => inner.kind(),
            _ => embedded_io::ErrorKind::Other
        }
    }
}

pub type EncodeResult<T, E> = Result<T, EncodeError<E>>;

#[inline]
pub(crate) fn write_i32<W: Write>(writer: &mut W, val: i32) -> EncodeResult<(), W::Error> {
    writer.write_all(&val.to_le_bytes()).map_err(From::from)
}

#[inline]
pub(crate) fn write_u32<W: Write>(writer: &mut W, val: u32) -> EncodeResult<(), W::Error> {
    writer.write_all(&val.to_le_bytes()).map_err(From::from)
}

#[inline]
pub(crate) fn write_i64<W: Write>(writer: &mut W, val: i64) -> EncodeResult<(), W::Error> {
    writer.write_all(&val.to_le_bytes()).map_err(From::from)
}

#[inline]
pub(crate) fn write_u64<W: Write>(writer: &mut W, val: u64) -> EncodeResult<(), W::Error> {
    writer.write_all(&val.to_le_bytes()).map_err(From::from)
}

#[inline]
pub(crate) fn write_f32<W: Write>(writer: &mut W, val: f32) -> EncodeResult<(), W::Error> {
    writer.write_all(&val.to_le_bytes()).map_err(From::from)
}

#[inline]
pub(crate) fn write_f64<W: Write>(writer: &mut W, val: f64) -> EncodeResult<(), W::Error> {
    writer.write_all(&val.to_le_bytes()).map_err(From::from)
}
pub(crate) fn write_key<W: Write>(writer: &mut W, s: &str) -> EncodeResult<(), W::Error> {
    if s.is_empty() || s.len() >= 255 {
        return Err(EncodeError::InvalidKeyLen(s.len(), "key len must > 0 and < 255".to_string()))
    }

    writer.write_all(&[s.len() as u8 + 1])?;
    writer.write_all(s.as_bytes())?;
    Ok(())
}

pub(crate) fn write_string<W: Write>(writer: &mut W, s: &str) -> EncodeResult<(), W::Error> {
    write_u32(writer, s.len() as u32 + 4)?;
    writer.write_all(s.as_bytes())?;
    Ok(())
}

pub(crate) fn write_binary<W: Write>(writer: &mut W, binary: &Binary) -> EncodeResult<(), W::Error> {
    write_u32(writer, binary.0.len() as u32 + 4)?;
    writer.write_all(&binary.0)?;
    Ok(())
}

pub(crate) fn encode_array<W: Write>(writer: &mut W, array: &Array) -> EncodeResult<(), W::Error> {
    let len = array.bytes_size();

    write_u32(writer, len as u32)?;

    for val in array.iter() {
        encode_value(writer, val)?;
    }

    writer.write_all(&[0])?;

    Ok(())
}

pub(crate) fn encode_message<W: Write>(writer: &mut W, message: &Message) -> EncodeResult<(), W::Error> {
    let len = message.bytes_size();

    write_u32(writer, len as u32)?;

    for (key, val) in message {
        write_key(writer, key)?;

        encode_value(writer, val)?;
    }

    writer.write_all(&[0])?;

    Ok(())
}

pub fn encode_value<W: Write>(writer: &mut W, val: &Value) -> EncodeResult<(), W::Error> {
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
        Value::Message(ref o) => encode_message(writer, o),
        Value::Bool(b) => writer.write_all(&[if b { 0x01 } else { 0x00 }]).map_err(From::from),
        Value::Null => Ok(()),
        Value::Binary(ref binary) => write_binary(writer, binary),
        Value::TimeStamp(v) => write_u64(writer, v.0),
        Value::MessageId(ref id) => writer.write_all(&id.bytes()).map_err(From::from)
    }
}

impl Value {
    pub fn to_bytes(&self) -> EncodeResult<Vec<u8>, core::convert::Infallible> {
        let mut buf = Vec::new();
        encode_value(&mut buf, self)?;
        Ok(buf)
    }
}

impl Message {
    pub fn to_bytes(&self) -> EncodeResult<Vec<u8>, core::convert::Infallible> {
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
    pub fn to_bytes(&self) -> EncodeResult<Vec<u8>, core::convert::Infallible> {
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
use serde::ser::Serialize;
#[cfg(feature = "serde")]
use crate::serde::encode::Encoder;

#[cfg(feature = "serde")]
pub fn to_nson<T: ?Sized, E: embedded_io::Error>(value: &T) -> EncodeResult<Value, E>
    where T: Serialize
{
    let ser = Encoder::new();
    value.serialize(ser).map_err(EncodeError::Serde)
}

#[cfg(feature = "serde")]
pub fn to_bytes<T: ?Sized>(value: &T) -> EncodeResult<Vec<u8>, core::convert::Infallible>
    where T: Serialize
{
    let value = to_nson(value)?;
    value.to_bytes()
}
