use alloc::string::{String, ToString};

use serde::ser::{
    Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant, Serializer,
};

use crate::array::Array;
use crate::id::Id;
use crate::map::Map;
use crate::value::{Binary, TimeStamp, Value};

use super::to_nson;
use super::EncodeError;
use super::EncodeResult;

impl Serialize for Map {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl Serialize for Value {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Value::F32(v) => serializer.serialize_f32(v),
            Value::F64(v) => serializer.serialize_f64(v),
            Value::I32(v) => serializer.serialize_i32(v),
            Value::I64(v) => serializer.serialize_i64(v),
            Value::U32(v) => serializer.serialize_u32(v),
            Value::U64(v) => serializer.serialize_u64(v),
            Value::I8(v) => serializer.serialize_i8(v),
            Value::U8(v) => serializer.serialize_u8(v),
            Value::I16(v) => serializer.serialize_i16(v),
            Value::U16(v) => serializer.serialize_u16(v),
            Value::String(ref v) => serializer.serialize_str(v),
            Value::Array(ref v) => v.serialize(serializer),
            Value::Map(ref v) => v.serialize(serializer),
            Value::Bool(v) => serializer.serialize_bool(v),
            Value::Null => serializer.serialize_unit(),
            Value::Binary(ref bytes) => serializer.serialize_bytes(&bytes.0),
            _ => {
                let msg = self.to_extended_map();
                msg.serialize(serializer)
            }
        }
    }
}

#[derive(Default)]
pub struct Encoder;

impl Encoder {
    pub fn new() -> Encoder {
        Encoder
    }
}

impl Serializer for Encoder {
    type Ok = Value;
    type Error = EncodeError;

    type SerializeSeq = ArraySerializer;
    type SerializeTuple = TupleSerializer;
    type SerializeTupleStruct = TupleStructSerializer;
    type SerializeTupleVariant = TupleVariantSerializer;
    type SerializeMap = MapSerializer;
    type SerializeStruct = StructSerializer;
    type SerializeStructVariant = StructVariantSerializer;

    #[inline]
    fn serialize_bool(self, value: bool) -> EncodeResult<Value> {
        Ok(Value::Bool(value))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> EncodeResult<Value> {
        Ok(Value::I8(value))
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> EncodeResult<Value> {
        Ok(Value::U8(value))
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> EncodeResult<Value> {
        Ok(Value::I16(value))
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> EncodeResult<Value> {
        Ok(Value::U16(value))
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> EncodeResult<Value> {
        Ok(Value::I32(value))
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> EncodeResult<Value> {
        Ok(Value::U32(value))
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> EncodeResult<Value> {
        Ok(Value::I64(value))
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> EncodeResult<Value> {
        Ok(Value::U64(value))
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> EncodeResult<Value> {
        Ok(Value::F32(value))
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> EncodeResult<Value> {
        Ok(Value::F64(value))
    }

    #[inline]
    fn serialize_char(self, value: char) -> EncodeResult<Value> {
        let mut s = String::new();
        s.push(value);
        self.serialize_str(&s)
    }

    #[inline]
    fn serialize_str(self, value: &str) -> EncodeResult<Value> {
        Ok(Value::String(value.to_string()))
    }

    fn serialize_bytes(self, value: &[u8]) -> EncodeResult<Value> {
        Ok(Value::Binary(Binary(value.into())))
    }

    #[inline]
    fn serialize_none(self) -> EncodeResult<Value> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<V: Serialize + ?Sized>(self, value: &V) -> EncodeResult<Value> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> EncodeResult<Value> {
        Ok(Value::Null)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> EncodeResult<Value> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> EncodeResult<Value> {
        Ok(Value::String(variant.to_string()))
    }

    #[inline]
    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> EncodeResult<Value> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> EncodeResult<Value> {
        let mut newtype_variant = Map::new();
        newtype_variant.insert(variant, to_nson(value)?);
        Ok(Value::Map(newtype_variant))
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> EncodeResult<Self::SerializeSeq> {
        Ok(ArraySerializer {
            inner: Array::with_capacity(len.unwrap_or(0)),
        })
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> EncodeResult<Self::SerializeTuple> {
        Ok(TupleSerializer {
            inner: Array::with_capacity(len),
        })
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> EncodeResult<Self::SerializeTupleStruct> {
        Ok(TupleStructSerializer {
            inner: Array::with_capacity(len),
        })
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> EncodeResult<Self::SerializeTupleVariant> {
        Ok(TupleVariantSerializer {
            inner: Array::with_capacity(len),
            name: variant,
        })
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> EncodeResult<Self::SerializeMap> {
        Ok(MapSerializer {
            inner: Map::new(),
            next_key: None,
        })
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> EncodeResult<Self::SerializeStruct> {
        Ok(StructSerializer { inner: Map::new() })
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> EncodeResult<Self::SerializeStructVariant> {
        Ok(StructVariantSerializer {
            name: variant,
            inner: Map::new(),
        })
    }
}

pub struct ArraySerializer {
    inner: Array,
}

impl SerializeSeq for ArraySerializer {
    type Ok = Value;
    type Error = EncodeError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<()> {
        self.inner.push(to_nson(value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Value> {
        Ok(Value::Array(self.inner))
    }
}

pub struct TupleSerializer {
    inner: Array,
}

impl SerializeTuple for TupleSerializer {
    type Ok = Value;
    type Error = EncodeError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<()> {
        self.inner.push(to_nson(value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Value> {
        Ok(Value::Array(self.inner))
    }
}

pub struct TupleStructSerializer {
    inner: Array,
}

impl SerializeTupleStruct for TupleStructSerializer {
    type Ok = Value;
    type Error = EncodeError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<()> {
        self.inner.push(to_nson(value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Value> {
        Ok(Value::Array(self.inner))
    }
}

pub struct TupleVariantSerializer {
    inner: Array,
    name: &'static str,
}

impl SerializeTupleVariant for TupleVariantSerializer {
    type Ok = Value;
    type Error = EncodeError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<()> {
        self.inner.push(to_nson(value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Value> {
        let mut tuple_variant = Map::new();
        tuple_variant.insert(self.name, self.inner);
        Ok(Value::Map(tuple_variant))
    }
}

pub struct MapSerializer {
    inner: Map,
    next_key: Option<String>,
}

impl SerializeMap for MapSerializer {
    type Ok = Value;
    type Error = EncodeError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> EncodeResult<()> {
        self.next_key = match to_nson(&key)? {
            Value::String(s) => Some(s),
            other => return Err(EncodeError::InvalidMapKeyType(other)),
        };
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<()> {
        let key = self.next_key.take().unwrap_or_default();
        self.inner.insert(key, to_nson(&value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Value> {
        Ok(Value::from_extended_map(self.inner))
    }
}

pub struct StructSerializer {
    inner: Map,
}

impl SerializeStruct for StructSerializer {
    type Ok = Value;
    type Error = EncodeError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> EncodeResult<()> {
        self.inner.insert(key, to_nson(value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Value> {
        Ok(Value::from_extended_map(self.inner))
    }
}

pub struct StructVariantSerializer {
    inner: Map,
    name: &'static str,
}

impl SerializeStructVariant for StructVariantSerializer {
    type Ok = Value;
    type Error = EncodeError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> EncodeResult<()> {
        self.inner.insert(key, to_nson(value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Value> {
        let var = Value::from_extended_map(self.inner);

        let mut struct_variant = Map::new();
        struct_variant.insert(self.name, var);

        Ok(Value::Map(struct_variant))
    }
}

impl Serialize for TimeStamp {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = Value::TimeStamp(*self);
        value.serialize(serializer)
    }
}

impl Serialize for Id {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = Value::Id(*self);
        value.serialize(serializer)
    }
}

impl Serialize for Binary {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = Value::Binary(self.clone());
        value.serialize(serializer)
    }
}
