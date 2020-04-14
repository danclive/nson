use std::result;
use std::fmt;
use std::io::{Write, Read, Cursor};
use std::iter::{FromIterator, Extend};
use std::cmp::Ordering;
use std::ops::RangeFull;

use indexmap::IndexMap;
use byteorder::WriteBytesExt;

use crate::value::{Value, TimeStamp};
use crate::array::Array;
use crate::message_id::MessageId;
use crate::encode::{encode_message, encode_value, write_u32, EncodeResult};
use crate::decode::{decode_message, DecodeResult};

pub use indexmap::map::{IntoIter, Iter, IterMut, Entry, Keys, Values, ValuesMut, Drain};

#[derive(PartialEq, Debug)]
pub enum Error {
    NotPresent,
    UnexpectedType,
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Message {
    inner: IndexMap<String, Value>
}

impl Message {
    pub fn new() -> Message {
        Message {
            inner: IndexMap::new()
        }
    }

    pub fn with_capacity(n: usize) -> Message {
        Message {
            inner: IndexMap::with_capacity(n)
        }
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.inner.get(key)
    }

    pub fn get_full(&self, key: &str) -> Option<(usize, &String, &Value)> {
        self.inner.get_full(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.inner.get_mut(key)
    }

    pub fn get_mut_full(&mut self, key: &str) -> Option<(usize, &String, &mut Value)> {
        self.inner.get_full_mut(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.inner.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn entry(&mut self, key: impl Into<String>) -> Entry<String, Value> {
        self.inner.entry(key.into())
    }

    pub fn insert_value(&mut self, key: impl Into<String>, value: Value) -> Option<Value> {
        self.inner.insert(key.into(), value)
    }

    pub fn insert_value_full(&mut self, key: impl Into<String>, value: impl Into<Value>) -> (usize, Option<Value>) {
        self.inner.insert_full(key.into(), value.into())
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<Value>) -> Option<Value> {
        self.insert_value(key.into(), value.into())
    }

    pub fn insert_full(&mut self, key: impl Into<String>, value: impl Into<Value>) -> (usize, Option<Value>) {
        self.insert_value_full(key.into(), value.into())
    }

    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.inner.swap_remove(key)
    }

    pub fn swap_remove(&mut self, key: &str) -> Option<Value> {
        self.inner.swap_remove(key)
    }

    pub fn swap_remove_full(&mut self, key: &str) -> Option<(usize, String, Value)> {
        self.inner.swap_remove_full(key)
    }

    pub fn shift_remove(&mut self, key: &str) -> Option<Value> {
        self.inner.shift_remove(key)
    }

    pub fn shift_remove_full(&mut self, key: &str) -> Option<(usize, String, Value)> {
        self.inner.shift_remove_full(key)
    }

    pub fn pop(&mut self) -> Option<(String, Value)> {
        self.inner.pop()
    }

    pub fn retain<F>(&mut self, keep: F)
        where F: FnMut(&String, &mut Value) -> bool
    {
        self.inner.retain(keep)
    }

    pub fn sort_keys(&mut self) {
        self.inner.sort_keys()
    }

    pub fn sort_by<F>(&mut self, compare: F)
        where F: FnMut(&String, &Value, &String, &Value) -> Ordering
    {
        self.inner.sort_by(compare)
    }

    pub fn sorted_by<F>(self, compare: F) -> IntoIter<String, Value>
        where F: FnMut(&String, &Value, &String, &Value) -> Ordering
    {
        self.inner.sorted_by(compare)
    }

    pub fn drain(&mut self, range: RangeFull) -> Drain<String, Value> {
        self.inner.drain(range)
    }

    pub fn iter(&self) -> Iter<'_, String, Value> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, String, Value> {
        self.into_iter()
    }

    pub fn keys(&self) -> Keys<String, Value> {
        self.inner.keys()
    }

    pub fn values(&self) -> Values<String, Value> {
        self.inner.values()
    }

    pub fn value_mut(&mut self) -> ValuesMut<String, Value> {
        self.inner.values_mut()
    }

    pub fn get_f32(&self, key: &str) -> Result<f32> {
        match self.get(key) {
            Some(&Value::F32(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_f64(&self, key: &str) -> Result<f64> {
        match self.get(key) {
            Some(&Value::F64(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_i32(&self, key: &str) -> Result<i32> {
        match self.get(key) {
            Some(&Value::I32(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_u32(&self, key: &str) -> Result<u32> {
        match self.get(key) {
            Some(&Value::U32(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_i64(&self, key: &str) -> Result<i64> {
        match self.get(key) {
            Some(&Value::I64(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_u64(&self, key: &str) -> Result<u64> {
        match self.get(key) {
            Some(&Value::U64(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_str(&self, key: &str) -> Result<&str> {
        match self.get(key) {
            Some(&Value::String(ref v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_array(&self, key: &str) -> Result<&Array> {
        match self.get(key) {
            Some(&Value::Array(ref v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_message(&self, key: &str) -> Result<&Message> {
        match self.get(key) {
            Some(&Value::Message(ref v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_bool(&self, key: &str) -> Result<bool> {
        match self.get(key) {
            Some(&Value::Bool(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn is_null(&self, key: &str) -> bool {
        self.get(key) == Some(&Value::Null)
    }

    pub fn get_binary(&self, key: &str) -> Result<&Vec<u8>> {
        match self.get(key) {
            Some(&Value::Binary(ref v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_message_id(&self, key: &str) -> Result<&MessageId> {
        match self.get(key) {
            Some(&Value::MessageId(ref v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_timestamp(&self, key: &str) -> Result<&TimeStamp> {
        match self.get(key) {
            Some(&Value::TimeStamp(ref v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn encode(&self, writer: &mut impl Write) -> EncodeResult<()> {
        encode_message(writer, self)
    }

    pub fn decode(reader: &mut impl Read) -> DecodeResult<Message> {
        decode_message(reader)
    }

    pub fn to_vec(&self) -> EncodeResult<Vec<u8>> {
        let len = self.bytes_size();

        let mut buf = Vec::with_capacity(len);
        write_u32(&mut buf, len as u32)?;

        for (key, val) in self {
            encode_value(&mut buf, key.as_ref(), val)?;
        }

        buf.write_u8(0)?;

        Ok(buf)
    }

    pub fn from_slice(slice: &[u8]) -> DecodeResult<Message> {
        let mut reader = Cursor::new(slice);
        decode_message(&mut reader)
    }

    pub fn extend<I: IntoIterator<Item=(String, Value)>>(&mut self, iter: I) {
        self.inner.extend(iter);
    }

    pub fn get_index(&self, index: usize) -> Option<(&String, &Value)> {
        self.inner.get_index(index)
    }

    pub fn get_index_mut(&mut self, index: usize) -> Option<(&mut String, &mut Value)> {
        self.inner.get_index_mut(index)
    }

    pub fn swap_remove_index(&mut self, index: usize) -> Option<(String, Value)> {
        self.inner.swap_remove_index(index)
    }

    pub fn shift_remove_index(&mut self, index: usize) -> Option<(String, Value)> {
        self.inner.shift_remove_index(index)
    }

    pub fn bytes_size(&self) -> usize {
        4 + self.iter().map(|(k, v)| { 1 + k.len() + 1 + v.bytes_size() }).sum::<usize>() + 1
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Message({:?})", self.inner)
    }
}

impl fmt::Display for Message {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{{")?;

        let mut first = true;
        for (k, v) in self.iter() {
            if first {
                first = false;
                write!(fmt, " ")?;
            } else {
                write!(fmt, ", ")?;
            }

            write!(fmt, "{}: {}", k, v)?;
        }

        write!(fmt, "{}}}", if !first { " " } else { "" })?;

        Ok(())
    }
}

impl IntoIterator for Message {
    type Item = (String, Value);
    type IntoIter = IntoIter<String, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Message {
    type Item = (&'a String, &'a Value);
    type IntoIter = Iter<'a, String, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut Message {
    type Item = (&'a String, &'a mut Value);
    type IntoIter = IterMut<'a, String, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

impl FromIterator<(String, Value)> for Message {
    fn from_iter<I: IntoIterator<Item=(String, Value)>>(iter: I) -> Self {
        let mut msg = Message::with_capacity(8);

        for (k, v) in iter {
            msg.insert(k, v);
        }

        msg
    }
}

impl From<IndexMap<String, Value>> for Message {
    fn from(map: IndexMap<String, Value>) -> Message {
        Message { inner: map }
    }
}

#[cfg(test)]
mod test {
    use crate::Message;
    use crate::msg;

    #[test]
    fn to_vec() {
        let msg = msg!{"aa": "bb"};

        let vec = msg.to_vec().unwrap();

        let msg2 = Message::from_slice(&vec).unwrap();

        assert_eq!(msg, msg2);
    }

    #[test]
    fn extend() {
        let msg1 = msg!{"aa": "bb"};

        let mut msg2 = msg!{"cc": "dd"};
        msg2.extend(msg1);

        let msg3 = msg!{"aa": "bb", "cc": "dd"};

        assert_eq!(msg2, msg3);
    }
}
