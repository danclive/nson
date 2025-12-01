//! Serde

use core::fmt;

use alloc::string::{String, ToString};

use serde::de::{Deserialize, Deserializer, Error};
use serde::ser::{self, Serialize};

use crate::Value;
use crate::spec::DataType;

pub mod decode;
pub mod encode;

use decode::Decoder;
use encode::Encoder;

impl ser::Serialize for DataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let u = *self as u8;
        serializer.serialize_u8(u)
    }
}

impl<'de> Deserialize<'de> for DataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let u = u8::deserialize(deserializer)?;

        if let Some(a) = DataType::from(u) {
            return Ok(a);
        }

        Err(D::Error::custom("expecting DataType"))
    }
}

pub fn to_nson<T: Serialize + ?Sized>(value: &T) -> EncodeResult<Value> {
    let ser = Encoder::new();
    value.serialize(ser)
}

pub fn from_nson<'de, T: Deserialize<'de>>(value: Value) -> DecodeResult<T> {
    let de = Decoder::new(value);
    Deserialize::deserialize(de)
}

#[derive(Debug)]
#[non_exhaustive]
pub enum DecodeError {
    ExpectedField(&'static str),
    UnknownField(String),
    SyntaxError(String),
    EndOfStream,
    InvalidType(String),
    InvalidLength(usize, String),
    DuplicatedField(&'static str),
    UnknownVariant(String),
    InvalidValue(String),
    Unknown(String),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
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
            DecodeError::Unknown(ref inner) => inner.fmt(fmt),
        }
    }
}

impl core::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        None
    }
}

pub type DecodeResult<T> = Result<T, DecodeError>;

#[derive(Debug)]
#[non_exhaustive]
pub enum EncodeError {
    InvalidMapKeyType(Value),
    Unknown(String),
}

impl fmt::Display for EncodeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EncodeError::InvalidMapKeyType(ref nson) => {
                write!(fmt, "Invalid type: {:?}", nson)
            }
            EncodeError::Unknown(ref inner) => inner.fmt(fmt),
        }
    }
}

impl core::error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        None
    }
}

impl ser::Error for EncodeError {
    fn custom<T: fmt::Display>(msg: T) -> EncodeError {
        EncodeError::Unknown(msg.to_string())
    }
}

pub type EncodeResult<T> = Result<T, EncodeError>;
