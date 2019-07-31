use std::io::{self, Write};
use std::fmt;
use std::error;
use std::{i32, i64};

use byteorder::{LittleEndian, WriteBytesExt};
use chrono::Timelike;
use serde::ser::{self, Serialize};

use crate::value::Value;
use crate::serde_impl::encode::Encoder;

#[derive(Debug)]
pub enum EncodeError {
    IoError(io::Error),
    InvalidMapKeyType(Value),
    Unknown(String),
    UnsupportedUnsignedType
}

impl From<io::Error> for EncodeError {
    fn from(err: io::Error) -> EncodeError {
        EncodeError::IoError(err)
    }
}

impl fmt::Display for EncodeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EncodeError::IoError(ref inner) => inner.fmt(fmt),
            EncodeError::InvalidMapKeyType(ref bson) => {
                write!(fmt, "Invalid map key type: {:?}", bson)
            }
            EncodeError::Unknown(ref inner) => inner.fmt(fmt),
            EncodeError::UnsupportedUnsignedType => write!(fmt, "bson does not support unsigned type"),
        }
    }
}

impl error::Error for EncodeError {
    fn description(&self) -> &str {
        match *self {
            EncodeError::IoError(ref inner) => inner.description(),
            EncodeError::InvalidMapKeyType(_) => "Invalid map key type",
            EncodeError::Unknown(ref inner) => inner,
            EncodeError::UnsupportedUnsignedType => "bson does not support unsigned type",
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            EncodeError::IoError(ref inner) => Some(inner),
            _ => None,
        }
    }
}

impl ser::Error for EncodeError {
    fn custom<T: fmt::Display>(msg: T) -> EncodeError {
        EncodeError::Unknown(msg.to_string())
    }
}

pub type EncodeResult<T> = Result<T, EncodeError>;

pub(crate) fn write_string(writer: &mut impl Write, s: &str) -> EncodeResult<()> {
    writer.write_i32::<LittleEndian>(s.len() as i32 + 1)?;
    writer.write_all(s.as_bytes())?;
    writer.write_u8(0)?;
    Ok(())
}

pub(crate) fn write_cstring(writer: &mut impl Write, s: &str) -> EncodeResult<()> {
    writer.write_all(s.as_bytes())?;
    writer.write_u8(0)?;
    Ok(())
}

#[inline]
pub(crate) fn write_i32(writer: &mut impl Write, val: i32) -> EncodeResult<()> {
    writer.write_i32::<LittleEndian>(val).map_err(From::from)
}

#[inline]
pub(crate) fn write_u32(writer: &mut impl Write, val: u32) -> EncodeResult<()> {
    writer.write_u32::<LittleEndian>(val).map_err(From::from)
}

#[inline]
pub(crate) fn write_i64(writer: &mut impl Write, val: i64) -> EncodeResult<()> {
    writer.write_i64::<LittleEndian>(val).map_err(From::from)
}

#[inline]
pub(crate) fn write_u64(writer: &mut impl Write, val: u64) -> EncodeResult<()> {
    writer.write_u64::<LittleEndian>(val).map_err(From::from)
}

#[inline]
pub(crate) fn write_f32(writer: &mut impl Write, val: f32) -> EncodeResult<()> {
    writer.write_f32::<LittleEndian>(val).map_err(From::from)
}

#[inline]
pub(crate) fn write_f64(writer: &mut impl Write, val: f64) -> EncodeResult<()> {
    writer.write_f64::<LittleEndian>(val).map_err(From::from)
}

fn encode_array(writer: &mut impl Write, arr: &[Value]) -> EncodeResult<()> {
    let mut buf = Vec::with_capacity(64);
    write_i32(&mut buf, 0)?;

    for (key, val) in arr.iter().enumerate() {
        encode_value(&mut buf, &key.to_string(), val)?;
    }

    buf.write_u8(0)?;

    let len_bytes = (buf.len() as i32).to_le_bytes();

    buf[..4].clone_from_slice(&len_bytes);

    writer.write_all(&buf)?;
    Ok(())
}

pub fn encode_value(writer: &mut impl Write, key: &str, val: &Value) -> EncodeResult<()> {
    writer.write_u8(val.element_type() as u8)?;
    write_cstring(writer, key)?;

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
        Value::Boolean(b) => writer.write_u8(if b { 0x01 } else { 0x00 }).map_err(From::from),
        Value::Null => Ok(()),
        Value::Binary(ref data) => {
            write_i32(writer, data.len() as i32)?;
            writer.write_all(data).map_err(From::from)
        }
        Value::TimeStamp(v) => write_u64(writer, v),
        Value::UTCDatetime(ref v) => {
            write_i64(
                writer,
                (v.timestamp() * 1000) + i64::from(v.nanosecond() / 1_000_000)
            )
        }
        Value::MessageId(ref id) => writer.write_all(&id.bytes()).map_err(From::from)
    }
}

pub fn encode_message<'a, S, D> (writer: &mut impl Write, message: D) -> EncodeResult<()>
    where S: AsRef<str> + 'a, D: IntoIterator<Item = (&'a S, &'a Value)>
{
    let mut buf = Vec::with_capacity(64);
    write_i32(&mut buf, 0)?;

    for (key, val) in message {
        encode_value(&mut buf, key.as_ref(), val)?;
    }

    buf.write_u8(0)?;

    let len_bytes = (buf.len() as i32).to_le_bytes();

    buf[..4].clone_from_slice(&len_bytes);

    writer.write_all(&buf)?;
    Ok(())
}

pub fn to_nson<T: ?Sized>(value: &T) -> EncodeResult<Value>
    where T: Serialize
{
    let ser = Encoder::new();
    value.serialize(ser)
}

pub fn to_vec<T: ?Sized>(value: &T) -> EncodeResult<Vec<u8>>
    where T: Serialize
{
    let bson = to_nson(value)?;

    if let Value::Message(msg) = bson {
        let mut buf: Vec<u8> = Vec::new();
        encode_message(&mut buf, &msg)?;
        return Ok(buf)
    }

    Err(EncodeError::InvalidMapKeyType(bson))
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use crate::encode::encode_message;
    use crate::decode::decode_message;
    use crate::msg;

    #[test]
    fn encode() {
        let msg = msg!{"aa": "bb", "cc": [1, 2, 3, 4]};

        let mut buf: Vec<u8> = Vec::new();

        encode_message(&mut buf, &msg).unwrap();

        let mut reader = Cursor::new(buf);

        let msg2 = decode_message(&mut reader).unwrap();

        assert_eq!(msg, msg2);
    }
}
