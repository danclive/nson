//! Array

use core::fmt;
use core::iter::FromIterator;
use core::ops::{Deref, DerefMut};

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::io::{Read, Write};

#[cfg(not(feature = "std"))]
use crate::io::{Read, Write};

use crate::decode::{decode_array, DecodeResult};
use crate::encode::{encode_array, EncodeResult};

use super::id::Id;
use super::map::Map;
use super::value::Value;

#[derive(Clone, PartialEq, Default, Eq)]
pub struct Array {
    inner: Vec<Value>,
}

impl Array {
    pub fn new() -> Array {
        Array { inner: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Array {
        Array {
            inner: Vec::with_capacity(capacity),
        }
    }

    pub fn from_vec(vec: Vec<Value>) -> Array {
        Array { inner: vec }
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

    pub fn iter(&self) -> core::slice::Iter<'_, Value> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Value> {
        self.into_iter()
    }

    pub fn bytes_size(&self) -> usize {
        4 + self.iter().map(|v| v.bytes_size() + 1).sum::<usize>() + 1
    }

    pub fn encode(&self, writer: &mut impl Write) -> EncodeResult<()> {
        encode_array(writer, self)
    }

    pub fn decode(reader: &mut impl Read) -> DecodeResult<Array> {
        decode_array(reader)
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

impl AsRef<Vec<Value>> for Array {
    fn as_ref(&self) -> &Vec<Value> {
        &self.inner
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
    Map bool Vec<u8> Id
}

impl IntoIterator for Array {
    type Item = Value;
    type IntoIter = alloc::vec::IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Array {
    type Item = &'a Value;
    type IntoIter = core::slice::Iter<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut Array {
    type Item = &'a mut Value;
    type IntoIter = core::slice::IterMut<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

impl FromIterator<Value> for Array {
    fn from_iter<I: IntoIterator<Item = Value>>(iter: I) -> Self {
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

        let vec = array.to_bytes().unwrap();

        let array2 = Array::from_bytes(&vec).unwrap();

        assert_eq!(array, array2);
    }
}
