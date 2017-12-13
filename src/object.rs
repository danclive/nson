use std::result;
use std::fmt;

use linked_hash_map::{self, LinkedHashMap};
use chrono::{DateTime, Utc};

use nson::Nson;
use nson::Array;

#[derive(PartialEq)]
pub enum Error {
    NotPresent,
    UnexpectedType,
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, PartialEq)]
pub struct Object {
    inner: LinkedHashMap<String, Nson>
}

impl Object {
    pub fn new() -> Object {
        Object {
            inner: LinkedHashMap::new()
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn get(&self, key: &str) -> Option<&Nson> {
        self.inner.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Nson> {
        self.inner.get_mut(key)
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

    pub fn insert_nson(&mut self, key: String, value: Nson) -> Option<Nson> {
        self.inner.insert(key, value)
    }

    pub fn insert<K: Into<String>, V: Into<Nson>>(&mut self, key: K, value: V) -> Option<Nson> {
        self.insert_nson(key.into(), value.into())
    }

    pub fn remove(&mut self, key: &str) -> Option<Nson> {
        self.inner.remove(key)
    }

    pub fn iter<'a>(&'a self) -> ObjectIterator<'a> {
        self.into_iter()
    }

    pub fn get_f64(&self, key: &str) -> Result<f64> {
        match self.get(key) {
            Some(&Nson::Double(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_i32(&self, key: &str) -> Result<i32> {
        match self.get(key) {
            Some(&Nson::I32(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_i64(&self, key: &str) -> Result<i64> {
        match self.get(key) {
            Some(&Nson::I64(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_u32(&self, key: &str) -> Result<u32> {
        match self.get(key) {
            Some(&Nson::U32(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_u64(&self, key: &str) -> Result<u64> {
        match self.get(key) {
            Some(&Nson::U64(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_str(&self, key: &str) -> Result<&str> {
        match self.get(key) {
            Some(&Nson::String(ref v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_array(&self, key: &str) -> Result<&Array> {
        match self.get(key) {
            Some(&Nson::Array(ref v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_object(&self, key: &str) -> Result<&Object> {
        match self.get(key) {
            Some(&Nson::Object(ref v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_bool(&self, key: &str) -> Result<bool> {
        match self.get(key) {
            Some(&Nson::Boolean(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn is_null(&self, key: &str) -> bool {
        self.get(key) == Some(&Nson::Null)
    }

    pub fn get_binary_generic(&self, key: &str) -> Result<&Vec<u8>> {
        match self.get(key) {
            Some(&Nson::Binary(ref v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_time_stamp(&self, key: &str) -> Result<i64> {
        match self.get(key) {
            Some(&Nson::TimeStamp(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_utc_datetime(&self, key: &str) -> Result<&DateTime<Utc>> {
        match self.get(key) {
            Some(&Nson::UTCDatetime(ref v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Object({:?})", self.inner)
    }
}

impl fmt::Display for Object {
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

pub struct ObjectIntoIterator {
    inner: LinkedHashMap<String, Nson>
}

pub struct ObjectIterator<'a> {
    inner: linked_hash_map::Iter<'a, String, Nson>
}

impl<'a> Iterator for ObjectIntoIterator {
    type Item = (String, Nson);
    fn next(&mut self) -> Option<(String, Nson)> {
        self.inner.pop_front()
    }
}

impl<'a> Iterator for ObjectIterator<'a> {
    type Item = (&'a String, &'a Nson);
    fn next(&mut self) -> Option<(&'a String, &'a Nson)> {
        self.inner.next()
    }
}

impl IntoIterator for Object {
    type Item = (String, Nson);
    type IntoIter = ObjectIntoIterator;

    fn into_iter(self) -> Self:: IntoIter {
        ObjectIntoIterator { inner: self.inner }
    }
}

impl<'a> IntoIterator for &'a Object {
    type Item = (&'a String, &'a Nson);
    type IntoIter = ObjectIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ObjectIterator { inner: self.inner.iter() }
    }
}

impl From<LinkedHashMap<String, Nson>> for Object {
    fn from(map: LinkedHashMap<String, Nson>) -> Object {
        Object { inner: map }
    }
}
