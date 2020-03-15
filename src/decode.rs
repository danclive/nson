use std::{io, error, fmt, string};
use std::io::{Read, Cursor};

use byteorder::{LittleEndian, ReadBytesExt};

use serde::de::Deserialize;

use crate::spec::ElementType;
use crate::value::Value;
use crate::message::Message;
use crate::array::Array;
use crate::serde_impl::decode::Decoder;
use crate::message_id::MessageId;

#[derive(Debug)]
pub enum DecodeError {
    IoError(io::Error),
    FromUtf8Error(string::FromUtf8Error),
    UnrecognizedElementType(u8),
    InvalidArrayKey(usize, String),
    ExpectedField(&'static str),
    UnknownField(String),
    SyntaxError(String),
    EndOfStream,
    InvalidType(String),
    InvalidLength(usize, String),
    DuplicatedField(&'static str),
    UnknownVariant(String),
    InvalidValue(String),
    InvalidTimestamp(i64),
    AmbiguousTimestamp(i64),
    Unknown(String)
}

impl From<io::Error> for DecodeError {
    fn from(err: io::Error) -> DecodeError {
        DecodeError::IoError(err)
    }
}

impl From<string::FromUtf8Error> for DecodeError {
    fn from(err: string::FromUtf8Error) -> DecodeError {
        DecodeError::FromUtf8Error(err)
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecodeError::IoError(ref inner) => inner.fmt(fmt),
            DecodeError::FromUtf8Error(ref inner) => inner.fmt(fmt),
            DecodeError::UnrecognizedElementType(tag) => {
                write!(fmt, "Unrecognized element type `{}`", tag)
            }
            DecodeError::InvalidArrayKey(ref want, ref got) => {
                write!(fmt, "Invalid array key: expected `{}`, got `{}`", want, got)
            }
            DecodeError::ExpectedField(field_type) => {
                write!(fmt, "Expected a field of type `{}`", field_type)
            }
            DecodeError::UnknownField(ref field) => write!(fmt, "Unknown field `{}`", field),
            DecodeError::SyntaxError(ref inner) => inner.fmt(fmt),
            DecodeError::EndOfStream => write!(fmt, "End of stream"),
            DecodeError::InvalidType(ref desc) => desc.fmt(fmt),
            DecodeError::InvalidLength(ref len, ref desc) => {
                write!(fmt, "Expecting length {}, {}", len, desc)
            }
            DecodeError::DuplicatedField(ref field) => write!(fmt, "Duplicated field `{}`", field),
            DecodeError::UnknownVariant(ref var) => write!(fmt, "Unknown variant `{}`", var),
            DecodeError::InvalidValue(ref desc) => desc.fmt(fmt),
            DecodeError::InvalidTimestamp(ref i) => write!(fmt, "no such local time {}", i),
            DecodeError::AmbiguousTimestamp(ref i) => write!(fmt, "ambiguous local time {}", i),
            DecodeError::Unknown(ref inner) => inner.fmt(fmt),
        }
    }
}

impl error::Error for DecodeError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            DecodeError::IoError(ref inner) => Some(inner),
            DecodeError::FromUtf8Error(ref inner) => Some(inner),
            _ => None,
        }
    }
}

pub type DecodeResult<T> = Result<T, DecodeError>;

pub(crate) fn read_string(reader: &mut impl Read) -> DecodeResult<String> {
    let len = reader.read_u32::<LittleEndian>()?;

    if len > crate::MAX_NSON_SIZE {
        return Err(DecodeError::InvalidLength(len as usize, format!("Invalid binary length of {}", len)));
    }

    let mut s = String::with_capacity(len as usize - 1);
    reader.take(len as u64 - 1).read_to_string(&mut s)?;
    reader.read_u8()?; // The last 0x00

    Ok(s)
}

pub(crate) fn read_cstring(reader: &mut impl Read) -> DecodeResult<String> {
    let mut v = Vec::new();

    loop {
        let c = reader.read_u8()?;
        if c == 0 {
            break;
        }
        v.push(c);
    }

    Ok(String::from_utf8(v)?)
}

#[inline]
pub(crate) fn read_i32(reader: &mut impl Read) -> DecodeResult<i32> {
    reader.read_i32::<LittleEndian>().map_err(From::from)
}

#[inline]
pub(crate) fn read_i64(reader: &mut impl Read) -> DecodeResult<i64> {
    reader.read_i64::<LittleEndian>().map_err(From::from)
}

#[inline]
pub(crate) fn read_u32(reader: &mut impl Read) -> DecodeResult<u32> {
    reader.read_u32::<LittleEndian>().map_err(From::from)
}

#[inline]
pub(crate) fn read_u64(reader: &mut impl Read) -> DecodeResult<u64> {
    reader.read_u64::<LittleEndian>().map_err(From::from)
}

#[inline]
pub(crate) fn read_f32(reader: &mut impl Read) -> DecodeResult<f32> {
    reader.read_f32::<LittleEndian>().map_err(From::from)
}

#[inline]
pub(crate) fn read_f64(reader: &mut impl Read) -> DecodeResult<f64> {
    reader.read_f64::<LittleEndian>().map_err(From::from)
}

pub fn decode_array(reader: &mut impl Read) -> DecodeResult<Array> {
    let mut arr = Array::new();

    // disregard the length: using Read::take causes infinite type recursion
    read_u32(reader)?;

    loop {
        let tag = reader.read_u8()?;
        if tag == 0 {
            break;
        }

        // check that the key is as expected
        let key = read_cstring(reader)?;
        match key.parse::<usize>() {
            Err(..) => return Err(DecodeError::InvalidArrayKey(arr.len(), key)),
            Ok(idx) => {
                if idx != arr.len() {
                    return Err(DecodeError::InvalidArrayKey(arr.len(), key));
                }
            }
        }

        let val = decode_value(reader, tag)?;
        arr.push(val)
    }

    Ok(arr)
}

fn decode_value(reader: &mut impl Read, tag: u8) -> DecodeResult<Value> {
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
        Some(ElementType::Message) => {
            decode_message(reader).map(Value::Message)
        }
        Some(ElementType::Array) => {
            decode_array(reader).map(Value::Array)
        }
        Some(ElementType::Binary) => {
            let len = read_u32(reader)?;

            if len > crate::MAX_NSON_SIZE {
                return Err(DecodeError::InvalidLength(len as usize, format!("Invalid binary length of {}", len)));
            }

            let mut data = Vec::with_capacity(len as usize);

            reader.take(len as u64).read_to_end(&mut data)?;

            Ok(Value::Binary(data))
        }
        Some(ElementType::Boolean) => {
            Ok(Value::Boolean(reader.read_u8()? != 0))
        }
        Some(ElementType::Null) => {
            Ok(Value::Null)
        }
        Some(ElementType::TimeStamp) => {
            read_u64(reader).map(|v|v.into())
        }
        Some(ElementType::MessageId) => {
            let mut buf = [0; 16];
            reader.read_exact(&mut buf)?;

            Ok(Value::MessageId(MessageId::with_bytes(buf)))
        }
        None => {
            Err(DecodeError::UnrecognizedElementType(tag))
        }
    }
}

pub fn decode_message(reader: &mut impl Read) -> DecodeResult<Message> {
    let mut msg = Message::new();

    // disregard the length: using Read::take causes infinite type recursion
    read_u32(reader)?;

    loop {
        let tag = reader.read_u8()?;

        if tag == 0 {
            break;
        }

        let key = read_cstring(reader)?;
        let val = decode_value(reader, tag)?;

        msg.insert(key, val);
    }

    Ok(msg)
}

pub fn from_nson<'de, T>(value: Value) -> DecodeResult<T>
    where T: Deserialize<'de>
{
    let de = Decoder::new(value);
    Deserialize::deserialize(de)
}

pub fn from_slice<'de, T>(slice: &[u8]) -> DecodeResult<T>
    where T: Deserialize<'de>
{
    let mut reader = Cursor::new(slice);
    let msg = decode_message(&mut reader)?;
    from_nson(Value::Message(msg))
}
