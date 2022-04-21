use std::collections::BTreeMap;

use log::debug;
use serde::de::{DeserializeSeed, IntoDeserializer, SeqAccess, Visitor};
use serde::{de, forward_to_deserialize_any};

use crate::emtpy_str::EmptyStr;
use crate::error::Error;
use crate::value::Node;

pub fn from_env<T>() -> Result<T, Error>
where
    T: de::DeserializeOwned,
{
    T::deserialize(Deserializer(Node::from_env()))
}

pub fn from_prefixed_env<T>(_prefix: &str) -> Result<T, Error>
where
    T: de::DeserializeOwned,
{
    todo!()
}

struct Deserializer(Node);

impl<'de> de::Deserializer<'de> for Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(vis)
    }

    fn deserialize_bool<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize bool: {:?}", &self.0 .0);
        vis.visit_bool(self.0 .0.parse().map_err(Error::new)?)
    }

    fn deserialize_i8<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize i8: {:?}", &self.0 .0);
        vis.visit_i8(self.0 .0.parse().map_err(Error::new)?)
    }

    fn deserialize_i16<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize i16: {:?}", &self.0 .0);
        vis.visit_i16(self.0 .0.parse().map_err(Error::new)?)
    }

    fn deserialize_i32<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize i32: {:?}", &self.0 .0);
        vis.visit_i32(self.0 .0.parse().map_err(Error::new)?)
    }

    fn deserialize_i64<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize i64: {:?}", &self.0 .0);
        vis.visit_i64(self.0 .0.parse().map_err(Error::new)?)
    }

    fn deserialize_u8<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize u8: {:?}", &self.0 .0);

        vis.visit_u8(self.0 .0.parse().map_err(Error::new)?)
    }

    fn deserialize_u16<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize u16: {:?}", &self.0 .0);

        vis.visit_u16(self.0 .0.parse().map_err(Error::new)?)
    }

    forward_to_deserialize_any! {
        unit unit_struct
        tuple_struct enum ignored_any
    }

    fn deserialize_u32<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize u32: {:?}", &self.0 .0);

        vis.visit_u32(self.0 .0.parse().map_err(Error::new)?)
    }

    fn deserialize_u64<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize u64: {:?}", &self.0 .0);

        vis.visit_u64(self.0 .0.parse().map_err(Error::new)?)
    }

    fn deserialize_f32<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize f32: {:?}", &self.0 .0);

        vis.visit_f32(self.0 .0.parse().map_err(Error::new)?)
    }

    fn deserialize_f64<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize f64: {:?}", &self.0 .0);

        vis.visit_f64(self.0 .0.parse().map_err(Error::new)?)
    }

    fn deserialize_char<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize char: {:?}", &self.0 .0);

        vis.visit_char(self.0 .0.parse().map_err(Error::new)?)
    }

    fn deserialize_str<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize str: {:?}", &self.0 .0);

        vis.visit_str(&self.0 .0)
    }

    fn deserialize_string<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize string: {:?}", &self.0 .0);

        vis.visit_string(self.0 .0)
    }

    fn deserialize_bytes<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize bytes: {:?}", &self.0 .0);

        vis.visit_bytes(self.0 .0.as_bytes())
    }

    fn deserialize_byte_buf<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize byte_buf: {:?}", &self.0 .0);

        vis.visit_byte_buf(self.0 .0.into_bytes())
    }

    fn deserialize_option<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize option: {:?}", &self.0 .0);

        if self.0 .0.is_empty() {
            vis.visit_none()
        } else {
            vis.visit_some(Deserializer(self.0))
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        vis: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize newtype struct: {:?}", &self.0 .0);

        vis.visit_newtype_struct(Deserializer(self.0))
    }

    fn deserialize_seq<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize seq: {:?}", &self.0 .0);

        let elements = self
            .0
             .0
            .split(',')
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
            .collect();

        vis.visit_seq(SeqAccessor::new(elements))
    }

    fn deserialize_tuple<V>(self, _len: usize, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize tuple: {:?}", &self.0 .0);

        let elements = self.0 .0.split(',').map(|v| v.trim().to_string()).collect();

        vis.visit_seq(SeqAccessor::new(elements))
    }

    fn deserialize_map<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize map: {:?}, {:?}", &self.0 .0, &self.0 .1);

        let keys = self.0 .1.keys().map(|v| v.to_string()).collect();
        vis.visit_map(MapAccessor::new(keys, self.0))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        vis: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize struct: {:?}, {:?}", &self.0 .0, &self.0 .1);

        vis.visit_map(MapAccessor::new(
            fields.iter().map(|v| v.to_string()).collect(),
            self.0,
        ))
    }

    fn deserialize_identifier<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        debug!("deserialize identifier: {:?}", &self.0 .0);

        self.deserialize_string(vis)
    }
}

struct SeqAccessor {
    elements: std::vec::IntoIter<String>,
}

impl SeqAccessor {
    fn new(keys: Vec<String>) -> Self {
        Self {
            elements: keys.into_iter(),
        }
    }
}

impl<'de> SeqAccess<'de> for SeqAccessor {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.elements.next() {
            None => Ok(None),
            Some(v) => Ok(Some(
                seed.deserialize(Deserializer(Node(v, BTreeMap::new())))?,
            )),
        }
    }
}

struct MapAccessor {
    last_key: Option<String>,
    keys: std::vec::IntoIter<String>,
    node: Node,
}

impl MapAccessor {
    fn new(keys: Vec<String>, node: Node) -> Self {
        debug!("access keys {:?} from {:?}", keys, node);

        Self {
            last_key: None,
            keys: keys.into_iter(),
            node,
        }
    }
}

impl<'de> de::MapAccess<'de> for MapAccessor {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        debug_assert!(
            self.last_key.is_none(),
            "value for the last entry is not deserialized"
        );

        match self.keys.next() {
            None => Ok(None),
            Some(k) => {
                self.last_key = Some(k.clone());
                Ok(Some(seed.deserialize(k.into_deserializer())?))
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let key = self
            .last_key
            .take()
            .expect("value for current entry is missing");

        match self.node.get(&key) {
            None => {
                debug!("key {} not found, use empty str instead", key);
                seed.deserialize(EmptyStr)
            }
            Some(v) => seed.deserialize(Deserializer(v.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Deserialize, Default, PartialEq, Debug)]
    #[serde(default)]
    struct TestStruct {
        a: i64,
        b: bool,
        c: String,
        d: EmbedStruct,
    }

    #[derive(Deserialize, Default, PartialEq, Debug)]
    #[serde(default)]
    struct EmbedStruct {
        aa: f32,
        bb: String,
    }

    #[test]
    fn test_from_env() {
        let _ = env_logger::try_init();

        temp_env::with_vars(
            vec![
                ("A", Some("123")),
                ("B", Some("true")),
                ("C", Some("Hello, test")),
                ("D_AA", Some("1.2")),
                ("D_BB", Some("Hello, embed")),
            ],
            || {
                let t: TestStruct = from_env().expect("must success");
                assert_eq!(
                    t,
                    TestStruct {
                        a: 123,
                        b: true,
                        c: "Hello, test".to_string(),
                        d: EmbedStruct {
                            aa: 1.2,
                            bb: "Hello, embed".to_string()
                        }
                    }
                )
            },
        )
    }

    /// This test is ported from [softprops/envy](https://github.com/softprops/envy/blob/801d81e7c3e443470e110bf4e34460acba113476/src/lib.rs#L410)
    /// FIXME: default not works correctly.
    #[derive(Deserialize, Debug, PartialEq)]
    // #[serde(default)]
    pub struct Foo {
        bar: String,
        baz: bool,
        zoom: Option<u16>,
        doom: Vec<u64>,
        boom: Vec<String>,
        // #[serde(default = "default_kaboom")]
        // kaboom: u16,
        #[serde(default)]
        debug_mode: bool,
        provided: Option<String>,
        newtype: CustomNewType,
        boom_zoom: bool,
    }

    /// FIXME: default not works correctly.
    // pub fn default_kaboom() -> u16 {
    //     8080
    // }

    #[derive(Deserialize, Debug, PartialEq, Default)]
    pub struct CustomNewType(u32);

    #[test]
    fn test_ported_from_envy() {
        let _ = env_logger::try_init();

        temp_env::with_vars(
            vec![
                ("BAR", Some("test")),
                ("BAZ", Some("true")),
                ("DOOM", Some("1, 2, 3 ")),
                // Empty string should result in empty vector.
                ("BOOM", Some("")),
                ("SIZE", Some("small")),
                ("PROVIDED", Some("test")),
                ("NEWTYPE", Some("42")),
                ("boom_zoom", Some("true")),
            ],
            || {
                let actual: Foo = from_env().expect("must success");
                assert_eq!(
                    actual,
                    Foo {
                        bar: String::from("test"),
                        baz: true,
                        zoom: None,
                        doom: vec![1, 2, 3],
                        boom: vec![],
                        // kaboom: 8080,
                        debug_mode: false,
                        provided: Some(String::from("test")),
                        newtype: CustomNewType(42),
                        boom_zoom: true,
                    }
                )
            },
        )
    }
}
