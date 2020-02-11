use std::fmt;
use std::ops::{Deref, DerefMut};
use std::{f64, i64};
use std::iter::FromIterator;
use std::convert::Into;
use std::io::{Write, Read, Cursor};

use byteorder::WriteBytesExt;

use crate::value::Value;
use crate::message::Message;
use crate::message_id::MessageId;
use crate::encode::{encode_array, encode_value, write_u32, EncodeResult};
use crate::decode::{decode_array, DecodeResult};

#[derive(Clone, PartialEq, Default, Eq)]
pub struct Array {
    inner: Vec<Value>
}

impl Array {
    pub fn new() -> Array {
        Array {
            inner: Vec::new()
        }
    }

    pub fn with_capacity(capacity: usize) -> Array {
        Array {
            inner: Vec::with_capacity(capacity)
        }
    }

    pub fn from_vec(vec: Vec<Value>) -> Array {
        Array {
            inner: vec
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn push(&mut self, value: impl Into<Value>) {
        self.inner.push(value.into());
    }

    pub fn push_value(&mut self, value: Value) {
        self.inner.push(value);
    }

    pub fn inner(&self) -> &Vec<Value> {
        &self.inner
    }

    pub fn as_mut_inner(&mut self) -> &mut Vec<Value> {
        &mut self.inner
    }

    pub fn into_inner(self) -> Vec<Value> {
        self.inner
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Value> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Value> {
        self.into_iter()
    }

    pub fn encode(&self, writer: &mut impl Write) -> EncodeResult<()> {
        encode_array(writer, self)
    }

    pub fn decode(reader: &mut impl Read) -> DecodeResult<Array> {
        decode_array(reader)
    }

    pub fn to_vec(&self) -> EncodeResult<Vec<u8>> {
        let mut buf = Vec::with_capacity(64);
        write_u32(&mut buf, 0)?;

        for (key, val) in self.iter().enumerate() {
            encode_value(&mut buf, &key.to_string(), val)?;
        }

        buf.write_u8(0)?;

        let len_bytes = (buf.len() as i32).to_le_bytes();

        buf[..4].clone_from_slice(&len_bytes);

        Ok(buf)
    }

    pub fn from_slice(slice: &[u8]) -> DecodeResult<Array> {
        let mut reader = Cursor::new(slice);
        decode_array(&mut reader)
    }
}

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl Deref for Array {
    type Target = Vec<Value>;
    fn deref(&self) -> &Vec<Value> {
        &self.inner
    }
}

impl DerefMut for Array {
    fn deref_mut(&mut self) -> &mut Vec<Value> {
        &mut self.inner
    }
}

macro_rules! array_from_impls {
    ($($T:ty)+) => {
        $(
            impl From<Vec<$T>> for Array {
                fn from(vec: Vec<$T>) -> Array {
                    vec.into_iter().map(Into::into).collect()
                }
            }
        )+
    }
}

array_from_impls! {
    f32 f64 i32 i64 u32 u64 &str String &String Array
    Message bool Vec<u8> Vec<Vec<u8>> MessageId
}

impl IntoIterator for Array {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Array {
    type Item = &'a Value;
    type IntoIter = std::slice::Iter<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut Array {
    type Item = &'a mut Value;
    type IntoIter = std::slice::IterMut<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

impl FromIterator<Value> for Array {
    fn from_iter<I: IntoIterator<Item=Value>>(iter: I) -> Self {
        let mut array = Array::new();

        for i in iter {
            array.push(i);
        }

        array
    }
}

#[cfg(test)]
mod test {
    use crate::Array;

    #[test]
    fn to_vec() {
        let mut array = Array::new();

        array.push(123);
        array.push("haha");

        let vec = array.to_vec().unwrap();

        let array2 = Array::from_slice(&vec).unwrap();

        assert_eq!(array, array2);
    }
}
