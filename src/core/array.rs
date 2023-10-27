use core::fmt;
use core::iter::FromIterator;
use core::ops::{Deref, DerefMut};

use alloc::vec::Vec;
use alloc::string::String;

use super::value::Value;
use super::message::Message;
use super::message_id::MessageId;

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

    pub fn iter(&self) -> core::slice::Iter<'_, Value> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Value> {
        self.into_iter()
    }

    pub fn bytes_size(&self) -> usize {
        4 + self.iter().map(|v| v.bytes_size() + 1).sum::<usize>() + 1
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
    Message bool Vec<u8> MessageId
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
    fn from_iter<I: IntoIterator<Item=Value>>(iter: I) -> Self {
        let mut array = Array::new();

        for i in iter {
            array.push(i);
        }

        array
    }
}
