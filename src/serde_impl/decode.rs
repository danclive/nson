use std::fmt;
use std::vec;
use std::result;
use std::marker::PhantomData;

use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess, SeqAccess, VariantAccess,
                DeserializeSeed, EnumAccess};
use serde::de::{Error, Expected, Unexpected};

use linked_hash_map::LinkedHashMap;

use nson::Nson;
use nson::UTCDateTime;
use object::Object;
use object::ObjectIntoIterator;
use decode::DecodeError;
use decode::DecodeResult;

impl de::Error for DecodeError {
    fn custom<T: fmt::Display>(msg: T) -> DecodeError {
        DecodeError::Unknown(msg.to_string())
    }

    fn invalid_type(_unexp: Unexpected, exp: &Expected) -> DecodeError {
        DecodeError::InvalidType(exp.to_string())
    }

    fn invalid_value(_unexp: Unexpected, exp: &Expected) -> DecodeError {
        DecodeError::InvalidValue(exp.to_string())
    }

    fn invalid_length(len: usize, exp: &Expected) -> DecodeError {
        DecodeError::InvalidLength(len, exp.to_string())
    }

    fn unknown_variant(variant: &str, _expected: &'static [&'static str]) -> DecodeError {
        DecodeError::UnknownVariant(variant.to_string())
    }

    fn unknown_field(field: &str, _expected: &'static [&'static str]) -> DecodeError {
        DecodeError::UnknownField(field.to_string())
    }

    fn missing_field(field: &'static str) -> DecodeError {
        DecodeError::ExpectedField(field)
    }

    fn duplicate_field(field: &'static str) -> DecodeError {
        DecodeError::DuplicatedField(field)
    }
}

impl<'de> Deserialize<'de> for Object {
    /// Deserialize this value given this `Deserializer`.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer
            .deserialize_map(NsonVisitor)
            .and_then(|bson|
                if let Nson::Object(object) = bson {
                    Ok(object)
                } else {
                    let err = format!("expected object, found extended JSON data type: {}", bson);
                    Err(de::Error::invalid_type(Unexpected::Map, &&*err))
            })
    }
}

impl<'de> Deserialize<'de> for Nson {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Nson, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_any(NsonVisitor)
    }
}

pub struct NsonVisitor;

impl<'de> Visitor<'de> for NsonVisitor {
    type Value = Nson;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "expecting a Nson")
    }

    #[inline]
    fn visit_bool<E>(self, value: bool) -> Result<Nson, E>
        where E: Error
    {
        Ok(Nson::Boolean(value))
    }

    #[inline]
    fn visit_i8<E>(self, value: i8) -> Result<Nson, E>
        where E: Error
    {
        Ok(Nson::I32(value as i32))
    }

    #[inline]
    fn visit_u8<E>(self, value: u8) -> Result<Nson, E>
        where E: Error
    {
        Ok(Nson::U32(value as u32))
    }

    #[inline]
    fn visit_i16<E>(self, value: i16) -> Result<Nson, E>
        where E: Error
    {
        Ok(Nson::I32(value as i32))
    }

    #[inline]
    fn visit_u16<E>(self, value: u16) -> Result<Nson, E>
        where E: Error
    {
        Ok(Nson::U32(value as u32))
    }

    #[inline]
    fn visit_i32<E>(self, value: i32) -> Result<Nson, E>
        where E: Error
    {
        Ok(Nson::I32(value))
    }

    #[inline]
    fn visit_u32<E>(self, value: u32) -> Result<Nson, E>
        where E: Error
    {
        Ok(Nson::U32(value))
    }

    #[inline]
    fn visit_i64<E>(self, value: i64) -> Result<Nson, E>
        where E: Error
    {
        Ok(Nson::I64(value))
    }

    #[inline]
    fn visit_u64<E>(self, value: u64) -> Result<Nson, E>
        where E: Error
    {
        Ok(Nson::U64(value))
    }

    #[inline]
    fn visit_f64<E>(self, value: f64) -> Result<Nson, E> {
        Ok(Nson::Double(value))
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Nson, E>
        where E: de::Error
    {
        self.visit_string(value.to_string())
    }

    #[inline]
    fn visit_string<E>(self, value: String) -> Result<Nson, E> {
        Ok(Nson::String(value))
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Nson, E> {
        Ok(Nson::Null)
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Nson, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_any(self)
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Nson, E> {
        Ok(Nson::Null)
    }

    #[inline]
    fn visit_seq<V>(self, mut visitor: V) -> Result<Nson, V::Error>
        where V: SeqAccess<'de>
    {
        let mut values = Vec::new();

        while let Some(elem) = visitor.next_element()? {
            values.push(elem);
        }

        Ok(Nson::Array(values))
    }

    #[inline]
    fn visit_map<V>(self, visitor: V) -> Result<Nson, V::Error>
        where V: MapAccess<'de>
    {
        let values = ObjectVisitor::new().visit_map(visitor)?;
        Ok(Nson::from_extended_object(values.into()))
    }
}

pub struct ObjectVisitor {
    marker: PhantomData<Object>
}

impl ObjectVisitor {
    pub fn new() -> ObjectVisitor {
        ObjectVisitor { marker: PhantomData }
    }
}

impl<'de> Visitor<'de> for ObjectVisitor {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "expecting ordered object")
    }

    #[inline]
    fn visit_unit<E>(self) -> result::Result<Object, E>
        where E: de::Error
    {
        Ok(Object::new())
    }

    #[inline]
    fn visit_map<V>(self, mut visitor: V) -> result::Result<Object, V::Error>
        where V: MapAccess<'de>
    {
        let mut inner = match visitor.size_hint() {
            Some(size) => LinkedHashMap::with_capacity(size),
            None => LinkedHashMap::new(),
        };

        while let Some((key, value)) = visitor.next_entry()? {
            inner.insert(key, value);
        }

        Ok(inner.into())
    }
}

/// Serde Decoder
pub struct Decoder {
    value: Option<Nson>,
}

impl Decoder {
    pub fn new(value: Nson) -> Decoder {
        Decoder { value: Some(value) }
    }
}

macro_rules! forward_to_deserialize {
    ($(
        $name:ident ( $( $arg:ident : $ty:ty ),* );
    )*) => {
        $(
            forward_to_deserialize!{
                func: $name ( $( $arg: $ty ),* );
            }
        )*
    };

    (func: deserialize_enum ( $( $arg:ident : $ty:ty ),* );) => {
        fn deserialize_enum<V>(
            self,
            $(_: $ty,)*
            _visitor: V,
        ) -> ::std::result::Result<V::Value, Self::Error>
            where V: ::serde::de::Visitor<'de>
        {
            Err(::serde::de::Error::custom("unexpected Enum"))
        }
    };

    (func: $name:ident ( $( $arg:ident : $ty:ty ),* );) => {
        #[inline]
        fn $name<V>(
            self,
            $(_: $ty,)*
            visitor: V,
        ) -> ::std::result::Result<V::Value, Self::Error>
            where V: ::serde::de::Visitor<'de>
        {
            self.deserialize_any(visitor)
        }
    };
}

impl<'de> Deserializer<'de> for Decoder {
    type Error = DecodeError;

    #[inline]
    fn deserialize_any<V>(mut self, visitor: V) -> DecodeResult<V::Value>
        where V: Visitor<'de>
    {
        let value = match self.value.take() {
            Some(value) => value,
            None => return Err(DecodeError::EndOfStream),
        };

        match value {
            Nson::Double(v) => visitor.visit_f64(v),
            Nson::I32(v) => visitor.visit_i32(v),
            Nson::I64(v) => visitor.visit_i64(v),
            Nson::U32(v) => visitor.visit_u32(v),
            Nson::U64(v) => visitor.visit_u64(v),
            Nson::String(v) => visitor.visit_string(v),
            Nson::Array(v) => {
                let len = v.len();
                visitor.visit_seq(
                    SeqDecoder {
                        iter: v.into_iter(),
                        len: len,
                    }
                )
            }
            Nson::Object(v) => {
                let len = v.len();
                visitor.visit_map(
                    MapDecoder {
                        iter: v.into_iter(),
                        value: None,
                        len: len,
                    }
                )
            }
            Nson::Boolean(v) => visitor.visit_bool(v),
            Nson::Null => visitor.visit_unit(),
            Nson::Binary(v) => visitor.visit_bytes(&v),
            _ => {
                let object = value.to_extended_object();
                let len = object.len();
                visitor.visit_map(
                    MapDecoder {
                        iter: object.into_iter(),
                        value: None,
                        len: len,
                    }
                )
            }
        }
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> DecodeResult<V::Value>
        where V: Visitor<'de>
    {
        match self.value {
            Some(Nson::Null) => visitor.visit_none(),
            Some(_) => visitor.visit_some(self),
            None => Err(DecodeError::EndOfStream),
        }
    }

    #[inline]
    fn deserialize_enum<V>(
        mut self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V
    ) -> DecodeResult<V::Value>
        where V: Visitor<'de>
    {
        let value = match self.value.take() {
            Some(Nson::Object(value)) => value,
            Some(Nson::String(variant)) => {
                return visitor.visit_enum(
                    EnumDecoder {
                        val: Nson::String(variant),
                        decoder: VariantDecoder { val: None },
                    }
                );
            }
            Some(_) => {
                return Err(DecodeError::InvalidType("expected an enum".to_string()));
            }
            None => {
                return Err(DecodeError::EndOfStream);
            }
        };

        let mut iter = value.into_iter();

        let (variant, value) = match iter.next() {
            Some(v) => v,
            None => return Err(DecodeError::SyntaxError("expected a variant name".to_string())),
        };

        // enums are encoded in json as maps with a single key:value pair
        match iter.next() {
            Some(_) => {
                Err(DecodeError::InvalidType("expected a single key:value pair".to_string()))
            }
            None => {
                visitor.visit_enum(
                    EnumDecoder {
                        val: Nson::String(variant),
                        decoder: VariantDecoder { val: Some(value) },
                    }
                )
            }
        }
    }

    #[inline]
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V
    ) -> DecodeResult<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_newtype_struct(self)
    }

    forward_to_deserialize!{
        deserialize_bool();
        deserialize_u8();
        deserialize_u16();
        deserialize_u32();
        deserialize_u64();
        deserialize_i8();
        deserialize_i16();
        deserialize_i32();
        deserialize_i64();
        deserialize_f32();
        deserialize_f64();
        deserialize_char();
        deserialize_str();
        deserialize_string();
        deserialize_unit();
        deserialize_seq();
        deserialize_bytes();
        deserialize_map();
        deserialize_unit_struct(name: &'static str);
        deserialize_tuple_struct(name: &'static str, len: usize);
        deserialize_struct(name: &'static str, fields: &'static [&'static str]);
        deserialize_tuple(len: usize);
        deserialize_identifier();
        deserialize_ignored_any();
        deserialize_byte_buf();
    }
}

struct EnumDecoder {
    val: Nson,
    decoder: VariantDecoder,
}

impl<'de> EnumAccess<'de> for EnumDecoder {
    type Error = DecodeError;
    type Variant = VariantDecoder;
    fn variant_seed<V>(self, seed: V) -> DecodeResult<(V::Value, Self::Variant)>
        where V: DeserializeSeed<'de>
    {
        let dec = Decoder::new(self.val);
        let value = seed.deserialize(dec)?;
        Ok((value, self.decoder))
    }
}

struct VariantDecoder {
    val: Option<Nson>,
}

impl<'de> VariantAccess<'de> for VariantDecoder {
    type Error = DecodeError;

    fn unit_variant(mut self) -> DecodeResult<()> {
        match self.val.take() {
            None => Ok(()),
            Some(val) => {
                Nson::deserialize(Decoder::new(val)).map(|_| ())
            }
        }
    }

    fn newtype_variant_seed<T>(mut self, seed: T) -> DecodeResult<T::Value>
        where T: DeserializeSeed<'de>
    {
        let dec = Decoder::new(self.val.take().ok_or(DecodeError::EndOfStream)?);
        seed.deserialize(dec)
    }

    fn tuple_variant<V>(mut self, _len: usize, visitor: V) -> DecodeResult<V::Value>
        where V: Visitor<'de>
    {
        if let Nson::Array(fields) = self.val.take().ok_or(DecodeError::EndOfStream)? {

            let de = SeqDecoder {
                len: fields.len(),
                iter: fields.into_iter(),
            };
            de.deserialize_any(visitor)
        } else {
            return Err(DecodeError::InvalidType("expected a tuple".to_string()));
        }
    }

    fn struct_variant<V>(
        mut self,
        _fields: &'static [&'static str],
        visitor: V
    ) -> DecodeResult<V::Value>
        where V: Visitor<'de>
    {
        if let Nson::Object(fields) = self.val.take().ok_or(DecodeError::EndOfStream)? {
            let de = MapDecoder {
                len: fields.len(),
                iter: fields.into_iter(),
                value: None,
            };
            de.deserialize_any(visitor)
        } else {
            return Err(DecodeError::InvalidType("expected a struct".to_string()));
        }
    }
}

struct SeqDecoder {
    iter: vec::IntoIter<Nson>,
    len: usize,
}

impl<'de> Deserializer<'de> for SeqDecoder {
    type Error = DecodeError;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> DecodeResult<V::Value>
        where V: Visitor<'de>
    {
        if self.len == 0 {
            visitor.visit_unit()
        } else {
            visitor.visit_seq(self)
        }
    }

    forward_to_deserialize!{
        deserialize_bool();
        deserialize_u8();
        deserialize_u16();
        deserialize_u32();
        deserialize_u64();
        deserialize_i8();
        deserialize_i16();
        deserialize_i32();
        deserialize_i64();
        deserialize_f32();
        deserialize_f64();
        deserialize_char();
        deserialize_str();
        deserialize_string();
        deserialize_unit();
        deserialize_option();
        deserialize_seq();
        deserialize_bytes();
        deserialize_map();
        deserialize_unit_struct(name: &'static str);
        deserialize_newtype_struct(name: &'static str);
        deserialize_tuple_struct(name: &'static str, len: usize);
        deserialize_struct(name: &'static str, fields: &'static [&'static str]);
        deserialize_tuple(len: usize);
        deserialize_enum(name: &'static str, variants: &'static [&'static str]);
        deserialize_identifier();
        deserialize_ignored_any();
        deserialize_byte_buf();
    }
}

impl<'de> SeqAccess<'de> for SeqDecoder {
    type Error = DecodeError;

    fn next_element_seed<T>(&mut self, seed: T) -> DecodeResult<Option<T::Value>>
        where T: DeserializeSeed<'de>
    {
        match self.iter.next() {
            None => Ok(None),
            Some(value) => {
                self.len -= 1;
                let de = Decoder::new(value);
                match seed.deserialize(de) {
                    Ok(value) => Ok(Some(value)),
                    Err(err) => Err(err),
                }
            }
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

struct MapDecoder {
    iter: ObjectIntoIterator,
    value: Option<Nson>,
    len: usize,
}

impl<'de> MapAccess<'de> for MapDecoder {
    type Error = DecodeError;

    fn next_key_seed<K>(&mut self, seed: K) -> DecodeResult<Option<K::Value>>
        where K: DeserializeSeed<'de>
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.len -= 1;
                self.value = Some(value);

                let de = Decoder::new(Nson::String(key));
                match seed.deserialize(de) {
                    Ok(val) => Ok(Some(val)),
                    Err(DecodeError::UnknownField(_)) => Ok(None),
                    Err(e) => Err(e),
                }
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> DecodeResult<V::Value>
        where V: DeserializeSeed<'de>
    {
        let value = self.value.take().ok_or(DecodeError::EndOfStream)?;
        let de = Decoder::new(value);
        seed.deserialize(de)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

impl<'de> Deserializer<'de> for MapDecoder {
    type Error = DecodeError;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> DecodeResult<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_map(self)
    }

    forward_to_deserialize!{
        deserialize_bool();
        deserialize_u8();
        deserialize_u16();
        deserialize_u32();
        deserialize_u64();
        deserialize_i8();
        deserialize_i16();
        deserialize_i32();
        deserialize_i64();
        deserialize_f32();
        deserialize_f64();
        deserialize_char();
        deserialize_str();
        deserialize_string();
        deserialize_unit();
        deserialize_option();
        deserialize_seq();
        deserialize_bytes();
        deserialize_map();
        deserialize_unit_struct(name: &'static str);
        deserialize_newtype_struct(name: &'static str);
        deserialize_tuple_struct(name: &'static str, len: usize);
        deserialize_struct(name: &'static str, fields: &'static [&'static str]);
        deserialize_tuple(len: usize);
        deserialize_enum(name: &'static str, variants: &'static [&'static str]);
        deserialize_identifier();
        deserialize_ignored_any();
        deserialize_byte_buf();
    }
}

impl<'de> Deserialize<'de> for UTCDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;

        match Nson::deserialize(deserializer)? {
            Nson::UTCDatetime(dt) => Ok(UTCDateTime(dt)),
            _ => Err(D::Error::custom("expecting UtcDateTime")),
        }
    }
}
