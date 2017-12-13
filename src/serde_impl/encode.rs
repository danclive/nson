use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeTuple, SerializeTupleStruct,
                 SerializeTupleVariant, SerializeMap, SerializeStruct, SerializeStructVariant};

use object::Object;
use nson::Nson;
use nson::Array;
use nson::UTCDateTime;
use encode::to_nson;
use encode::EncodeError;
use encode::EncodeResult;

impl Serialize for Object {
     #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self {
            map.serialize_key(k)?;
            map.serialize_value(v)?;
        }
        map.end()
    }
}

impl Serialize for Nson {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            Nson::Double(v) => serializer.serialize_f64(v),
            Nson::I32(v) => serializer.serialize_i32(v),
            Nson::I64(v) => serializer.serialize_i64(v),
            Nson::U32(v) => serializer.serialize_u32(v),
            Nson::U64(v) => serializer.serialize_u64(v),
            Nson::String(ref v) => serializer.serialize_str(v),
            Nson::Array(ref v) => v.serialize(serializer),
            Nson::Object(ref v) => v.serialize(serializer),
            Nson::Boolean(v) => serializer.serialize_bool(v),
            Nson::Null => serializer.serialize_unit(),
            _ => {
                let object = self.to_extended_object();
                object.serialize(serializer)
            }
        }
    }
}

pub struct Encoder;

impl Encoder {
    pub fn new() -> Encoder {
        Encoder
    }
}

impl Serializer for Encoder {
    type Ok = Nson;
    type Error = EncodeError;

    type SerializeSeq = ArraySerializer;
    type SerializeTuple = TupleSerializer;
    type SerializeTupleStruct = TupleStructSerializer;
    type SerializeTupleVariant = TupleVariantSerializer;
    type SerializeMap = MapSerializer;
    type SerializeStruct = StructSerializer;
    type SerializeStructVariant = StructVariantSerializer;

    #[inline]
    fn serialize_bool(self, value: bool) -> EncodeResult<Nson> {
        Ok(Nson::Boolean(value))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> EncodeResult<Nson> {
        self.serialize_i32(value as i32)
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> EncodeResult<Nson> {
        self.serialize_u32(value as u32)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> EncodeResult<Nson> {
        self.serialize_i32(value as i32)
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> EncodeResult<Nson> {
        self.serialize_u32(value as u32)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> EncodeResult<Nson> {
        Ok(Nson::I32(value))
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> EncodeResult<Nson> {
        Ok(Nson::U32(value))
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> EncodeResult<Nson> {
        Ok(Nson::I64(value))
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> EncodeResult<Nson> {
        Ok(Nson::U64(value))
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> EncodeResult<Nson> {
        self.serialize_f64(value as f64)
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> EncodeResult<Nson> {
        Ok(Nson::Double(value))
    }

    #[inline]
    fn serialize_char(self, value: char) -> EncodeResult<Nson> {
        let mut s = String::new();
        s.push(value);
        self.serialize_str(&s)
    }

    #[inline]
    fn serialize_str(self, value: &str) -> EncodeResult<Nson> {
        Ok(Nson::String(value.to_string()))
    }

    fn serialize_bytes(self, value: &[u8]) -> EncodeResult<Nson> {
        Ok(Nson::Binary(value.into()))
    }

    #[inline]
    fn serialize_none(self) -> EncodeResult<Nson> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<V: ?Sized>(self, value: &V) -> EncodeResult<Nson>
        where V: Serialize
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> EncodeResult<Nson> {
        Ok(Nson::Null)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> EncodeResult<Nson> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str
    ) -> EncodeResult<Nson> {
        Ok(Nson::String(variant.to_string()))
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T
    ) -> EncodeResult<Nson>
        where T: Serialize
    {
        let mut ser = TupleStructSerializer { inner: Array::new() };
        ser.serialize_field(value)?;
        ser.end()
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T
    ) -> EncodeResult<Nson>
        where T: Serialize
    {
        let mut ser = TupleVariantSerializer {
            inner: Array::new(),
            name: variant,
        };
        ser.serialize_field(value)?;
        ser.end()
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> EncodeResult<Self::SerializeSeq> {
        Ok(ArraySerializer { inner: Array::with_capacity(len.unwrap_or(0)) })
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> EncodeResult<Self::SerializeTuple> {
        Ok(TupleSerializer { inner: Array::with_capacity(len) })
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize
    ) -> EncodeResult<Self::SerializeTupleStruct> {
        Ok(TupleStructSerializer { inner: Array::with_capacity(len) })
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize
    ) -> EncodeResult<Self::SerializeTupleVariant> {
        Ok(TupleVariantSerializer {
            inner: Array::with_capacity(len),
            name: variant,
        })
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> EncodeResult<Self::SerializeMap> {
        Ok(MapSerializer {
            inner: Object::new(),
            next_key: None,
        })
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize
    ) -> EncodeResult<Self::SerializeStruct> {
        Ok(StructSerializer { inner: Object::new() })
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize
    ) -> EncodeResult<Self::SerializeStructVariant> {
        Ok(StructVariantSerializer {
            name: variant,
            inner: Object::new(),
        })
    }
}


pub struct ArraySerializer {
    inner: Array
}

impl SerializeSeq for ArraySerializer {
    type Ok = Nson;
    type Error = EncodeError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<()> {
        self.inner.push(to_nson(value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Nson> {
        Ok(Nson::Array(self.inner))
    }
}

pub struct TupleSerializer {
    inner: Array
}

impl SerializeTuple for TupleSerializer {
    type Ok = Nson;
    type Error = EncodeError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<()> {
        self.inner.push(to_nson(value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Nson> {
        Ok(Nson::Array(self.inner))
    }
}

pub struct TupleStructSerializer {
    inner: Array
}

impl SerializeTupleStruct for TupleStructSerializer {
    type Ok = Nson;
    type Error = EncodeError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<()> {
        self.inner.push(to_nson(value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Nson> {
        Ok(Nson::Array(self.inner))
    }
}

pub struct TupleVariantSerializer {
    inner: Array,
    name: &'static str
}

impl SerializeTupleVariant for TupleVariantSerializer {
    type Ok = Nson;
    type Error = EncodeError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<()> {
        self.inner.push(to_nson(value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Nson> {
        let mut tuple_variant = Object::new();
        if self.inner.len() == 1 {
            tuple_variant.insert(self.name, self.inner.into_iter().next().unwrap());
        } else {
            tuple_variant.insert(self.name, Nson::Array(self.inner));
        }

        Ok(Nson::Object(tuple_variant))
    }
}

pub struct MapSerializer {
    inner: Object,
    next_key: Option<String>
}

impl SerializeMap for MapSerializer {
    type Ok = Nson;
    type Error = EncodeError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> EncodeResult<()> {
        self.next_key = match to_nson(&key)? {
            Nson::String(s) => Some(s),
            other => return Err(EncodeError::InvalidMapKeyType(other)),
        };
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<()> {
        let key = self.next_key.take().unwrap_or_else(|| "".to_string());
        self.inner.insert(key, to_nson(&value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Nson> {
        Ok(Nson::from_extended_object(self.inner))
    }
}

pub struct StructSerializer {
    inner: Object
}

impl SerializeStruct for StructSerializer {
    type Ok = Nson;
    type Error = EncodeError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T
    ) -> EncodeResult<()> {
        self.inner.insert(key, to_nson(value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Nson> {
        Ok(Nson::from_extended_object(self.inner))
    }
}

pub struct StructVariantSerializer {
    inner: Object,
    name: &'static str
}

impl SerializeStructVariant for StructVariantSerializer {
    type Ok = Nson;
    type Error = EncodeError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T
    ) -> EncodeResult<()> {
        self.inner.insert(key, to_nson(value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Nson> {
        let var = Nson::from_extended_object(self.inner);

        let mut struct_variant = Object::new();
        struct_variant.insert(self.name, var);

        Ok(Nson::Object(struct_variant))
    }
}

impl Serialize for UTCDateTime {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        // Cloning a `DateTime` is extremely cheap
        let object = Nson::UTCDatetime(self.0);
        object.serialize(serializer)
    }
}

