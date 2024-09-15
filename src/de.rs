use std::collections::BTreeSet;

use crate::error::Error;
use crate::value::Node;
use serde::de::{DeserializeSeed, IntoDeserializer, SeqAccess, Visitor};
use serde::{de, forward_to_deserialize_any};

/// Deserialize into struct via env.
///
/// # Examples
///
/// ```
/// use serde::Deserialize;
/// use serde_env::from_env;
///
/// #[derive(Debug, Deserialize)]
/// struct Test {
///     #[cfg(windows)]
///     #[serde(rename = "userprofile")]
///     home: String,
///     #[cfg(not(windows))]
///     home: String,
///     #[serde(rename = "path")]
///     path_renamed: String,
/// }
///
/// let t: Test = from_env().expect("deserialize from env");
/// println!("{:?}", t);
/// ```
pub fn from_env<T>() -> Result<T, Error>
where
    T: de::DeserializeOwned,
{
    T::deserialize(Deserializer(Node::from_env()))
}
/// Deserialize into struct via env with a prefix.
///
/// # Examples
///
/// ```
/// use serde::Deserialize;
/// use serde_env::from_env_with_prefix;
///
/// #[derive(Debug, Deserialize, PartialEq)]
/// struct Test {
///     home: String,
///     path: String,
/// }
/// temp_env::with_vars(
///     [
///         ("TEST_ENV_HOME", Some("/test")),
///         ("TEST_ENV_PATH", Some("foo:bar")),
///     ],
///     || {
///         let t: Test = from_env_with_prefix("TEST_ENV").expect("deserialize from env");
///
///         let result = Test {
///             home: "/test".to_string(),
///             path: "foo:bar".to_string(),
///         };
///         assert_eq!(t, result);
///     },
/// );
/// ```
pub fn from_env_with_prefix<T>(prefix: &str) -> Result<T, Error>
where
    T: de::DeserializeOwned,
{
    T::deserialize(Deserializer(Node::from_env_with_prefix(prefix)))
}

/// Deserialize into struct via an iterable of `(AsRef<str>, AsRef<str>)`
/// representing keys and values.
///
/// # Examples
///
/// ```
/// use serde::Deserialize;
/// use serde_env::from_iter;
///
/// #[derive(Debug, Deserialize, PartialEq)]
/// struct Test {
///     home: String,
///     path: String,
/// }
/// let vars = [
///     ("HOME", "/test"),
///     ("PATH", "foo:bar"),
/// ];
///
/// let actual: Test = from_iter(vars).expect("deserialize from iter");
///
/// let expected = Test {
///     home: "/test".to_string(),
///     path: "foo:bar".to_string(),
/// };
///
/// assert_eq!(actual, expected);
/// ```
pub fn from_iter<Iter, S, T>(iter: Iter) -> Result<T, Error>
where
    Iter: IntoIterator<Item = (S, S)>,
    S: AsRef<str>,
    T: de::DeserializeOwned,
{
    T::deserialize(Deserializer(Node::from_iter(iter)))
}

/// Deserialize into struct via an iterable of `(AsRef<str>, AsRef<str>)`
/// representing keys and values, with a prefix.
///
/// # Examples
///
/// ```
/// use serde::Deserialize;
/// use serde_env::from_iter_with_prefix;
///
/// #[derive(Debug, Deserialize, PartialEq)]
/// struct Test {
///     home: String,
///     path: String,
/// }
/// let vars = ([
///     ("TEST_ENV_HOME", "/test"),
///     ("TEST_ENV_PATH", "foo:bar"),
/// ]);
///
/// let actual: Test = from_iter_with_prefix(vars, "TEST_ENV").expect("deserialize from iter");
///
/// let expected = Test {
///     home: "/test".to_string(),
///     path: "foo:bar".to_string(),
/// };
///
/// assert_eq!(actual, expected);
/// ```
pub fn from_iter_with_prefix<Iter, S, T>(iter: Iter, prefix: &str) -> Result<T, Error>
where
    Iter: IntoIterator<Item = (S, S)>,
    S: AsRef<str>,
    T: de::DeserializeOwned,
{
    T::deserialize(Deserializer(Node::from_iter_with_prefix(iter, prefix)))
}

struct Deserializer(Node);

impl<'de> de::Deserializer<'de> for Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_str(self.0.value())
    }

    fn deserialize_bool<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_bool(self.0.value().parse().map_err(Error::new)?)
    }

    fn deserialize_i8<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_i8(self.0.value().parse().map_err(Error::new)?)
    }

    fn deserialize_i16<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_i16(self.0.value().parse().map_err(Error::new)?)
    }

    fn deserialize_i32<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_i32(self.0.value().parse().map_err(Error::new)?)
    }

    fn deserialize_i64<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_i64(self.0.value().parse().map_err(Error::new)?)
    }

    fn deserialize_u8<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_u8(self.0.value().parse().map_err(Error::new)?)
    }

    fn deserialize_u16<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_u16(self.0.value().parse().map_err(Error::new)?)
    }

    forward_to_deserialize_any! {
        unit unit_struct
        tuple_struct ignored_any
    }

    fn deserialize_u32<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_u32(self.0.value().parse().map_err(Error::new)?)
    }

    fn deserialize_u64<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_u64(self.0.value().parse().map_err(Error::new)?)
    }

    fn deserialize_f32<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_f32(self.0.value().parse().map_err(Error::new)?)
    }

    fn deserialize_f64<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_f64(self.0.value().parse().map_err(Error::new)?)
    }

    fn deserialize_char<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_char(self.0.value().parse().map_err(Error::new)?)
    }

    fn deserialize_str<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_str(self.0.value())
    }

    fn deserialize_string<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_string(self.0.into_value())
    }

    fn deserialize_bytes<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_bytes(self.0.value().as_bytes())
    }

    fn deserialize_byte_buf<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_byte_buf(self.0.into_value().into_bytes())
    }

    fn deserialize_option<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.0.is_empty() {
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
        vis.visit_newtype_struct(Deserializer(self.0))
    }

    fn deserialize_seq<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let elements = self
            .0
            .value()
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
        let elements = self
            .0
            .value()
            .split(',')
            .map(|v| v.trim().to_string())
            .collect();

        vis.visit_seq(SeqAccessor::new(elements))
    }

    fn deserialize_map<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let keys = self.0.flatten("");
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
        let keys = fields.iter().map(|v| v.to_string()).collect();

        vis.visit_map(MapAccessor::new(keys, self.0))
    }

    fn deserialize_identifier<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(vis)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        variants: &'static [&'static str],
        vis: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let keys = variants.iter().map(|v| v.to_string()).collect();

        vis.visit_enum(EnumAccessor::new(keys, self.0))
    }
}

/// KeyDeserializer is used to deserialize key of a map/enum/struct.
///
/// It's similar to `StringDeserializer`, but only used for key deserialization in serde-env where:
///
/// - Key is case insensitive: `PATH`, `Path`, `path`, `PatH` all map to the same key.
/// - Key is prefix based: Key could be adapted based on the deserialize target itself.
struct KeyDeserializer {
    key: String,
}

impl KeyDeserializer {
    fn new(key: String) -> Self {
        Self { key }
    }
}

impl<'de> de::Deserializer<'de> for KeyDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_str(&self.key)
    }

    fn deserialize_str<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_str(&self.key)
    }

    fn deserialize_string<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_string(self.key)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        for v in variants {
            // Return the enum variant if it matches the key.
            if self.key.eq_ignore_ascii_case(v) {
                return visitor.visit_enum(v.into_deserializer());
            }
        }
        Err(de::Error::unknown_variant(&self.key, variants))
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char bytes byte_buf
        option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any
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
            Some(v) => Ok(Some(seed.deserialize(Deserializer(Node::new(v)))?)),
        }
    }
}

struct MapAccessor {
    last_value: Option<Node>,
    keys: std::collections::btree_set::IntoIter<String>,
    node: Node,
}

impl MapAccessor {
    fn new(keys: BTreeSet<String>, node: Node) -> Self {
        Self {
            last_value: None,
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
            self.last_value.is_none(),
            "value for the last entry is not deserialized"
        );

        loop {
            let key = match self.keys.next() {
                None => return Ok(None),
                Some(v) => v,
            };

            match self.node.get(&key) {
                // If key is not found inside node, skip it and continue.
                None => continue,
                Some(v) => {
                    self.last_value = Some(v.clone());
                    return Ok(Some(seed.deserialize(KeyDeserializer::new(key))?));
                }
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = self
            .last_value
            .take()
            .expect("value for current entry is missing");

        seed.deserialize(Deserializer(value))
    }
}

struct EnumAccessor {
    keys: std::vec::IntoIter<String>,
    node: Node,
}

impl EnumAccessor {
    fn new(keys: Vec<String>, node: Node) -> Self {
        Self {
            keys: keys.into_iter(),
            node,
        }
    }
}

impl<'de> de::EnumAccess<'de> for EnumAccessor {
    type Error = Error;
    type Variant = VariantAccessor;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let key = self
            .keys
            .find(|key| self.node.value() == key)
            .ok_or_else(|| de::Error::custom("no variant found"))?;

        let variant = VariantAccessor::new(self.node);
        Ok((seed.deserialize(key.into_deserializer())?, variant))
    }
}

struct VariantAccessor {
    node: Node,
}

impl VariantAccessor {
    fn new(node: Node) -> Self {
        Self { node }
    }
}

impl<'de> de::VariantAccess<'de> for VariantAccessor {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        if self.node.has_children() {
            return Err(de::Error::custom("variant is not unit"));
        }
        Ok(())
    }
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(Deserializer(self.node))
    }
    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("tuple variant is not supported"))
    }
    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let keys = fields.iter().map(|v| v.to_string()).collect();

        visitor.visit_map(MapAccessor::new(keys, self.node))
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use std::collections::HashMap;

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
    #[derive(Deserialize, Debug, PartialEq, Eq)]
    struct Foo {
        bar: String,
        baz: bool,
        zoom: Option<u16>,
        doom: Vec<u64>,
        boom: Vec<String>,
        #[serde(default = "default_kaboom")]
        kaboom: u16,
        #[serde(default)]
        debug_mode: bool,
        provided: Option<String>,
        newtype: CustomNewType,
        boom_zoom: bool,
        #[serde(default = "default_bool")]
        mode_xx: bool,
    }

    fn default_bool() -> bool {
        true
    }

    fn default_kaboom() -> u16 {
        8080
    }

    #[derive(Deserialize, Debug, PartialEq, Eq, Default)]
    struct CustomNewType(u32);

    #[test]
    fn test_ported_from_envy() {
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
                ("mode_xx", Some("false")),
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
                        kaboom: 8080,
                        debug_mode: false,
                        provided: Some(String::from("test")),
                        newtype: CustomNewType(42),
                        boom_zoom: true,
                        mode_xx: false
                    }
                )
            },
        )
    }

    #[derive(Deserialize, PartialEq, Debug)]
    struct TestStructAlias {
        #[serde(alias = "meta_log_level")]
        log_level: String,
    }

    // We are not support alias now.
    #[test]
    #[ignore]
    fn test_from_env_alias() {
        temp_env::with_vars(vec![("meta_log_level", Some("DEBUG"))], || {
            let t: TestStructAlias = from_env().expect("must success");
            assert_eq!(
                t,
                TestStructAlias {
                    log_level: "DEBUG".to_string()
                }
            )
        })
    }

    #[derive(Deserialize, PartialEq, Debug)]
    struct TestStructFlat {
        meta_log_level: String,
    }

    #[test]
    fn test_from_env_flat() {
        temp_env::with_vars(vec![("meta_log_level", Some("DEBUG"))], || {
            let t: TestStructFlat = from_env().expect("must success");
            assert_eq!(
                t,
                TestStructFlat {
                    meta_log_level: "DEBUG".to_string()
                }
            )
        })
    }

    #[test]
    fn test_from_env_flat_upper() {
        temp_env::with_vars(vec![("META_LOG_LEVEL", Some("DEBUG"))], || {
            let t: TestStructFlat = from_env().expect("must success");
            assert_eq!(
                t,
                TestStructFlat {
                    meta_log_level: "DEBUG".to_string()
                }
            )
        })
    }

    #[derive(Deserialize, PartialEq, Debug)]
    struct TestStructFlatWithDefault {
        meta_log_level: String,
    }

    impl Default for TestStructFlatWithDefault {
        fn default() -> Self {
            Self {
                meta_log_level: "INFO".to_string(),
            }
        }
    }

    #[test]
    fn test_from_env_flat_with_default() {
        temp_env::with_vars(vec![("meta_log_level", Some("DEBUG"))], || {
            let t: TestStructFlatWithDefault = from_env().expect("must success");
            assert_eq!(
                t,
                TestStructFlatWithDefault {
                    meta_log_level: "DEBUG".to_string()
                }
            )
        })
    }

    #[test]
    fn test_from_env_flat_upper_with_default() {
        temp_env::with_vars(vec![("META_LOG_LEVEL", Some("DEBUG"))], || {
            let t: TestStructFlatWithDefault = from_env().expect("must success");
            assert_eq!(
                t,
                TestStructFlatWithDefault {
                    meta_log_level: "DEBUG".to_string()
                }
            )
        })
    }

    #[test]
    fn test_from_env_as_map() {
        temp_env::with_vars(vec![("METASRV_LOG_LEVEL", Some("DEBUG"))], || {
            let t: HashMap<String, String> = from_env().expect("must success");
            assert_eq!(t["metasrv_log_level"], "DEBUG".to_string())
        })
    }

    #[derive(Deserialize, PartialEq, Debug)]
    struct EnumNewtype {
        bar: String,
    }

    #[derive(Deserialize, PartialEq, Debug)]
    struct ExternallyEnumStruct {
        foo: ExternallyEnum,
    }

    #[derive(Deserialize, PartialEq, Debug)]
    enum ExternallyEnum {
        X,
        Y(EnumNewtype),
        Z { a: i32 },
    }

    #[test]
    fn test_from_env_externally_enum() {
        temp_env::with_vars(vec![("FOO", Some("X"))], || {
            let t: ExternallyEnumStruct = from_env().expect("must success");
            assert_eq!(t.foo, ExternallyEnum::X)
        });

        temp_env::with_vars(vec![("FOO", Some("Y")), ("FOO_BAR", Some("xxx"))], || {
            let t: ExternallyEnumStruct = from_env().expect("must success");
            assert_eq!(
                t.foo,
                ExternallyEnum::Y(EnumNewtype {
                    bar: "xxx".to_string()
                })
            )
        });

        temp_env::with_vars(vec![("FOO", Some("Z")), ("FOO_A", Some("1"))], || {
            let t: ExternallyEnumStruct = from_env().expect("must success");
            assert_eq!(t.foo, ExternallyEnum::Z { a: 1 })
        });
    }

    #[derive(Deserialize, PartialEq, Debug)]
    struct TriggerDeserializeAny {
        foo: TriggerDeserializeAnyEnum,
    }

    #[serde_with::serde_as]
    #[derive(Deserialize, PartialEq, Debug)]
    #[serde(untagged)]
    enum TriggerDeserializeAnyEnum {
        Json(#[serde_as(as = "serde_with::json::JsonString")] Vec<String>),
        CommaSeparated(
            #[serde_as(
                deserialize_as = "serde_with::StringWithSeparator<serde_with::formats::CommaSeparator, String>"
            )]
            Vec<String>,
        ),
        NonJSON(Vec<String>),
    }

    #[test]
    fn test_from_env_deserialize_any() {
        temp_env::with_vars(vec![("FOO", Some("X"))], || {
            let t: TriggerDeserializeAny = from_env().expect("must success");
            assert_eq!(
                t.foo,
                TriggerDeserializeAnyEnum::CommaSeparated(vec!["X".to_string()])
            )
        });

        temp_env::with_vars(vec![("FOO", Some("X,Y"))], || {
            let t: TriggerDeserializeAny = from_env().expect("must success");
            assert_eq!(
                t.foo,
                TriggerDeserializeAnyEnum::CommaSeparated(vec!["X".to_string(), "Y".to_string()])
            )
        });

        temp_env::with_vars(vec![("FOO", Some("[\"X\",\"Y\"]"))], || {
            let t: TriggerDeserializeAny = from_env().expect("must success");
            assert_eq!(
                t.foo,
                TriggerDeserializeAnyEnum::Json(vec!["X".to_string(), "Y".to_string()])
            )
        });
    }

    #[derive(Deserialize, PartialEq, Debug)]
    struct InternallyEnumStruct {
        foo: InternallyEnum,
    }

    #[derive(Deserialize, PartialEq, Debug)]
    #[serde(tag = "type")]
    enum InternallyEnum {
        X,
        Y(EnumNewtype),
        Z { a: i32 },
    }

    // Currently Internally / Adjacently / Untagged enum is not support by the following issues
    // https://github.com/serde-rs/serde/issues/2187
    #[test]
    #[ignore]
    fn test_from_env_internally_enum() {
        temp_env::with_vars(vec![("FOO_TYPE", Some("X"))], || {
            let t: InternallyEnumStruct = from_env().expect("must success");
            assert_eq!(t.foo, InternallyEnum::X)
        });

        temp_env::with_vars(
            vec![("FOO_TYPE", Some("Y")), ("FOO_BAR", Some("xxx"))],
            || {
                let t: InternallyEnumStruct = from_env().expect("must success");
                assert_eq!(
                    t.foo,
                    InternallyEnum::Y(EnumNewtype {
                        bar: "xxx".to_string()
                    })
                )
            },
        );

        temp_env::with_vars(vec![("FOO_TYPE", Some("Z")), ("FOO_A", Some("1"))], || {
            let t: InternallyEnumStruct = from_env().expect("must success");
            assert_eq!(t.foo, InternallyEnum::Z { a: 1 })
        });
    }

    #[derive(Deserialize, PartialEq, Debug, Eq)]
    struct DoubleOptionOuter {
        inner: Option<DoubleOptionInner>,
    }

    #[derive(Deserialize, PartialEq, Debug, Eq)]
    struct DoubleOptionInner {
        val: Option<u8>,
    }

    #[test]
    fn double_option() {
        temp_env::with_var("INNER_VAL", Some("2"), || {
            let t: DoubleOptionOuter = from_env().expect("must success");
            assert_eq!(
                t,
                DoubleOptionOuter {
                    inner: Some(DoubleOptionInner { val: Some(2) })
                }
            )
        })
    }

    #[test]
    fn inner_mapping_with_enum_keys() {
        #[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
        enum MappingKey {
            Option1,
            Option2,
        }
        #[derive(Debug, Deserialize, Eq, PartialEq)]
        struct Mapping {
            val: HashMap<MappingKey, String>,
        }

        let env = vec![("VAL_OPTION1", "FOO"), ("VAL_OPTION2", "BAR")];
        let t: Mapping = from_iter(env).expect("must succeed");
        assert_eq!(
            t,
            Mapping {
                val: HashMap::from_iter(vec![
                    (MappingKey::Option1, "FOO".to_string()),
                    (MappingKey::Option2, "BAR".to_string())
                ])
            }
        );
    }

    // TODO: not supported yet, refer to https://github.com/Xuanwo/serde-env/issues/49
    //
    // #[test]
    // fn double_inner_mapping_with_enum_keys() {
    //     #[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
    //     #[serde(rename_all = "lowercase")]
    //     enum MappingKey {
    //         Option1,
    //         Option2,
    //     }
    //
    //     #[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
    //     #[serde(rename_all = "lowercase")]
    //     enum MappingKey2 {
    //         Inner1,
    //         Inner2,
    //     }
    //
    //     #[derive(Debug, Deserialize, Eq, PartialEq)]
    //     struct Mapping {
    //         val: HashMap<MappingKey, HashMap<MappingKey2, String>>,
    //     }
    //
    //     let env = vec![("VAL_OPTION1_INNER2", "FOO"), ("VAL_OPTION2_INNER1", "BAR")];
    //     let t: Mapping = from_iter(env).expect("must succeed");
    //     assert_eq!(
    //         t,
    //         Mapping {
    //             val: HashMap::from_iter(vec![
    //                 (
    //                     MappingKey::Option1,
    //                     HashMap::from_iter(vec![(MappingKey2::Inner2, "FOO".to_string())])
    //                 ),
    //                 (
    //                     MappingKey::Option2,
    //                     HashMap::from_iter(vec![(MappingKey2::Inner1, "BAR".to_string())])
    //                 ),
    //             ])
    //         }
    //     );
    // }
}
