use std::io::{self, Write};
use std::fmt;
use std::error;
use std::mem;
use std::i64;

use byteorder::{LittleEndian, WriteBytesExt};
use chrono::Timelike;
use serde::ser::{self, Serialize};

use nson::Nson;
use serde_impl::encode::Encoder;

#[derive(Debug)]
pub enum EncodeError {
    IoError(io::Error),
    InvalidMapKeyType(Nson),
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
            EncodeError::UnsupportedUnsignedType => write!(fmt, "NSON does not support unsigned type"),
        }
    }
}

impl error::Error for EncodeError {
    fn description(&self) -> &str {
        match *self {
            EncodeError::IoError(ref inner) => inner.description(),
            EncodeError::InvalidMapKeyType(_) => "Invalid map key type",
            EncodeError::Unknown(ref inner) => inner,
            EncodeError::UnsupportedUnsignedType => "NSON does not support unsigned type",
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

fn write_string<W>(writer: &mut W, s: &str) -> EncodeResult<()> 
    where W: Write + ?Sized
{
    writer.write_i32::<LittleEndian>(s.len() as i32 + 1)?;
    writer.write_all(s.as_bytes())?;
    writer.write_u8(0)?;
    Ok(())
}

fn write_cstring<W>(writer: &mut W, s: &str) -> EncodeResult<()>
    where W: Write + ?Sized
{
    writer.write_all(s.as_bytes())?;
    writer.write_u8(0)?;
    Ok(())
}

#[inline]
fn write_i32<W>(writer: &mut W, val: i32) -> EncodeResult<()> 
    where W: Write + ?Sized
{
    writer.write_i32::<LittleEndian>(val).map_err(From::from)
}

#[inline]
fn write_i64<W>(writer: &mut W, val: i64) -> EncodeResult<()>
    where W: Write + ?Sized
{
    writer.write_i64::<LittleEndian>(val).map_err(From::from)
}

#[inline]
fn write_u32<W>(writer: &mut W, val: u32) -> EncodeResult<()> 
    where W: Write + ?Sized
{
    writer.write_u32::<LittleEndian>(val).map_err(From::from)
}

#[inline]
fn write_u64<W>(writer: &mut W, val: u64) -> EncodeResult<()>
    where W: Write + ?Sized
{
    writer.write_u64::<LittleEndian>(val).map_err(From::from)
}

#[inline]
fn write_f64<W>(writer: &mut W, val: f64) -> EncodeResult<()>
    where W: Write + ?Sized
{
    writer.write_f64::<LittleEndian>(val).map_err(From::from)
}

fn encode_array<W>(writer: &mut W, arr: &[Nson]) -> EncodeResult<()>
    where W: Write + ?Sized
{
    let mut buf = Vec::new();
    for (key, val) in arr.iter().enumerate() {
        encode_nson(&mut buf, &key.to_string(), val)?;
    }

    write_i32(
        writer,
        (buf.len() + mem::size_of::<i32>() + mem::size_of::<u8>()) as i32
    )?;

    writer.write_all(&buf)?;
    writer.write_u8(0)?;
    Ok(())
}

pub fn encode_nson<W>(writer: &mut W, key: &str, val: &Nson) -> EncodeResult<()>
    where W: Write + ?Sized
{
    writer.write_u8(val.element_type() as u8)?;
    write_cstring(writer, key)?;

    match *val {
        Nson::Double(v) => write_f64(writer, v),
        Nson::I32(v) => write_i32(writer, v),
        Nson::I64(v) => write_i64(writer, v),
        Nson::U32(v) => write_u32(writer, v),
        Nson::U64(v) => write_u64(writer, v),
        Nson::String(ref s) => write_string(writer, s),
        Nson::Array(ref a) => encode_array(writer, a),
        Nson::Object(ref o) => encode_object(writer, o),
        Nson::Boolean(b) => writer.write_u8(if b { 0x01 } else { 0x00 }).map_err(From::from),
        Nson::Null => Ok(()),
        Nson::Binary(ref data) => {
            write_i32(writer, data.len() as i32)?;
            writer.write_all(data).map_err(From::from)
        }
        Nson::TimeStamp(v) => write_i64(writer, v),
        Nson::UTCDatetime(ref v) => {
            write_i64(
                writer,
                (v.timestamp() * 1000) + i64::from(v.nanosecond() / 1_000_000)
            )
        }
    }
}

pub fn encode_object<'a, S, W, D> (writer: &mut W, object: D) -> EncodeResult<()>
    where S: AsRef<str> + 'a, W: Write + ?Sized, D: IntoIterator<Item = (&'a S, &'a Nson)>
{
    let mut buf = Vec::new();
    for (key, val) in object {
        encode_nson(&mut buf, key.as_ref(), val)?;
    }

    write_i32(
        writer,
        (buf.len() + mem::size_of::<i32>() + mem::size_of::<u8>()) as i32
    )?;

    writer.write_all(&buf)?;
    writer.write_u8(0)?;
    Ok(())
}

pub fn to_nson<T: ?Sized>(value: &T) -> EncodeResult<Nson>
    where T: Serialize
{
    let ser = Encoder::new();
    value.serialize(ser)
}

pub fn to_vec<T: ?Sized>(value: &T) -> EncodeResult<Vec<u8>>
    where T: Serialize
{
    let nson = to_nson(value)?;

    if let Nson::Object(object) = nson {
        let mut buf: Vec<u8> = Vec::new();
        encode_object(&mut buf, &object)?;
        return Ok(buf)
    }

    Err(EncodeError::InvalidMapKeyType(nson))
}
