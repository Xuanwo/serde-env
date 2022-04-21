use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::forward_to_deserialize_any;


use crate::error::Error;

pub struct EmptyStr;

impl<'de> serde::de::Deserializer<'de> for EmptyStr {
    type Error = Error;

    fn deserialize_any<V>(self, _vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_bool(false)
    }

    fn deserialize_i8<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_i8(0)
    }

    fn deserialize_i16<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_i16(0)
    }

    fn deserialize_i32<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_i32(0)
    }

    fn deserialize_i64<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_i64(0)
    }

    fn deserialize_u8<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_u8(0)
    }

    fn deserialize_u16<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_u16(0)
    }

    fn deserialize_u32<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_u32(0)
    }

    fn deserialize_u64<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_u64(0)
    }

    fn deserialize_f32<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_f32(0.0)
    }

    fn deserialize_f64<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_f64(0.0)
    }

    fn deserialize_char<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_char('\0')
    }

    fn deserialize_str<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_str("")
    }

    fn deserialize_string<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_string("".to_string())
    }

    fn deserialize_bytes<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_bytes(&[])
    }

    fn deserialize_byte_buf<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_byte_buf(Vec::new())
    }

    fn deserialize_option<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_none()
    }

    fn deserialize_unit<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        vis: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_newtype_struct(EmptyStr)
    }

    fn deserialize_seq<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_seq(EmptyStr)
    }

    fn deserialize_tuple<V>(self, _len: usize, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_seq(EmptyStr)
    }

    forward_to_deserialize_any! { identifier tuple_struct enum ignored_any }

    fn deserialize_map<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_map(EmptyStr)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        vis: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_map(EmptyStr)
    }
}

impl<'de> SeqAccess<'de> for EmptyStr {
    type Error = Error;

    fn next_element_seed<T>(&mut self, _seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        Ok(None)
    }
}

impl<'de> MapAccess<'de> for EmptyStr {
    type Error = Error;

    fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        Ok(None)
    }

    fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        todo!()
    }
}
