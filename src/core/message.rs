use core::cmp::Ordering;
use core::ops::RangeFull;
use core::fmt;
use core::iter::FromIterator;

use alloc::string::String;

use indexmap::IndexMap;
pub use indexmap::map::{IntoIter, Iter, IterMut, Entry, Keys, Values, ValuesMut, Drain};

#[cfg(not(feature = "std"))]
use hash32::{BuildHasherDefault, FnvHasher};

use super::value::{Value, Binary, TimeStamp};
use super::array::Array;
use super::message_id::MessageId;

#[cfg(feature = "std")]
#[derive(Clone, PartialEq, Eq, Default)]
pub struct Message {
    inner: IndexMap<String, Value>
}

#[cfg(not(feature = "std"))]
#[derive(Clone, PartialEq, Eq, Default)]
pub struct Message {
    inner: IndexMap<String, Value, BuildHasherDefault<FnvHasher>>
}

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    NotPresent,
    UnexpectedType,
}

pub type Result<T> = core::result::Result<T, Error>;

#[cfg(feature = "std")]
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
}


#[cfg(not(feature = "std"))]
impl Message {
    pub fn new() -> Message {
        Message {
            inner: IndexMap::with_hasher(BuildHasherDefault::new())
        }
    }

    pub fn with_capacity(n: usize) -> Message {
        Message {
            inner: IndexMap::with_capacity_and_hasher(n, BuildHasherDefault::new())
        }
    }
}

impl Message {
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

    pub fn get_binary(&self, key: &str) -> Result<&Binary> {
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

    pub fn extend<I: IntoIterator<Item=(String, Value)>>(&mut self, iter: I) {
        self.inner.extend(iter);
    }

    pub fn get_index_of(&self, key: &str) -> Option<usize> {
        self.inner.get_index_of(key)
    }

    pub fn get_index(&self, index: usize) -> Option<(&String, &Value)> {
        self.inner.get_index(index)
    }

    pub fn get_index_mut(&mut self, index: usize) -> Option<(&String, &mut Value)> {
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
        write!(fmt, "Message{:?}", self.inner)
    }
}

impl fmt::Display for Message {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Message{{")?;

        let mut first = true;
        for (k, v) in self.iter() {
            if first {
                first = false;
            } else {
                write!(fmt, ", ")?;
            }

            write!(fmt, "{}: {}", k, v)?;
        }

        write!(fmt, "}}")?;

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

#[cfg(feature = "std")]
impl From<IndexMap<String, Value>> for Message {
    fn from(map: IndexMap<String, Value>) -> Message {
        Message { inner: map }
    }
}

#[cfg(not(feature = "std"))]
impl From<IndexMap<String, Value, BuildHasherDefault<FnvHasher>>> for Message {
    fn from(map: IndexMap<String, Value, BuildHasherDefault<FnvHasher>>) -> Message {
        Message { inner: map }
    }
}
