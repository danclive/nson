use std::io::{self, Write};
use std::fmt;
use std::error;
use std::{i32, i64};

use serde::ser::Serialize;

use crate::serde::encode::Encoder;

use crate::value::{Value, Binary};
use crate::message::Message;
use crate::array::Array;

#[derive(Debug)]
#[non_exhaustive]
pub enum EncodeError {
    IoError(io::Error),
    InvalidKeyLen(usize, String),
    Unknown(String),
    Serde(crate::serde::EncodeError)
}

impl From<io::Error> for EncodeError {
    fn from(err: io::Error) -> EncodeError {
        EncodeError::IoError(err)
    }
}

impl From<crate::serde::EncodeError> for EncodeError {
    fn from(err: crate::serde::EncodeError) -> EncodeError {
        EncodeError::Serde(err)
    }
}

impl fmt::Display for EncodeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EncodeError::IoError(ref inner) => inner.fmt(fmt),
            EncodeError::InvalidKeyLen(ref len, ref desc) => {
                write!(fmt, "Invalid key len: {}, {}", len, desc)
            }
            EncodeError::Unknown(ref inner) => inner.fmt(fmt),
            EncodeError::Serde(ref inner) => inner.fmt(fmt),
        }
    }
}

impl error::Error for EncodeError {
    fn cause(&self) -> Option<&dyn error::Error> {
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
        return Err(EncodeError::InvalidKeyLen(s.len(), "key len must > 0 and < 255".to_string()))
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

pub(crate) fn encode_message(writer: &mut impl Write, message: &Message) -> EncodeResult<()> {
    let len = message.bytes_size();

    write_u32(writer, len as u32)?;

    for (key, val) in message {
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
        Value::Message(ref o) => encode_message(writer, o),
        Value::Bool(b) => writer.write_all(&[if b { 0x01 } else { 0x00 }]).map_err(From::from),
        Value::Null => Ok(()),
        Value::Binary(ref binary) => write_binary(writer, binary),
        Value::TimeStamp(v) => write_u64(writer, v.0),
        Value::MessageId(ref id) => writer.write_all(&id.bytes()).map_err(From::from)
    }
}

impl Value {
    pub fn to_bytes(&self) -> EncodeResult<Vec<u8>> {
        let mut buf = Vec::new();
        encode_value(&mut buf, self)?;
        Ok(buf)
    }
}

impl Message {
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

pub fn to_nson<T: ?Sized>(value: &T) -> EncodeResult<Value>
    where T: Serialize
{
    let ser = Encoder::new();
    value.serialize(ser).map_err(EncodeError::Serde)
}

pub fn to_bytes<T: ?Sized>(value: &T) -> EncodeResult<Vec<u8>>
    where T: Serialize
{
    let value = to_nson(value)?;
    value.to_bytes()
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
