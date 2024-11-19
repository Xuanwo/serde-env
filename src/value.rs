use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Formatter};
use std::{env, fmt};

/// Node represents a tree of env values.
///
/// Every env will be separated by `_` in key to construct this tree.
///
/// - `ABC=123` => `Node("123", {})`
/// - `ABC_DEF=123` => `Node("", { "DEF": Node("123", {}) })`
/// - `ABC=123,ABC_DEF=456` => `Node("123", { "DEF": Node("456", {}) })`
#[derive(PartialEq, Clone)]
pub(crate) struct Node(String, BTreeMap<String, Node>);

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            // Value is empty, print inner map instead.
            f.debug_map().entries(&self.1).finish()
        } else if self.1.is_empty() {
            // inner map is empty too, print value instead.
            f.write_str(&self.0)
        } else {
            // Print as list.
            // The first entry is value, and the second is inner map.
            f.debug_list().entry(&self.0).entry(&self.1).finish()
        }
    }
}

impl Node {
    /// Create a new node without children
    pub(crate) fn new(v: impl Into<String>) -> Self {
        Node(v.into(), BTreeMap::new())
    }

    /// Get value from node.
    pub(crate) fn value(&self) -> &str {
        &self.0
    }

    /// Into value to get ownership.
    pub(crate) fn into_value(self) -> String {
        self.0
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty() && self.1.is_empty()
    }

    pub(crate) fn has_children(&self) -> bool {
        self.1.contains_key(&self.0)
    }

    pub(crate) fn flatten(&self, prefix: &str) -> BTreeSet<String> {
        let mut m = BTreeSet::new();

        for (key, value) in self.1.iter() {
            let prefix_key = if prefix.is_empty() {
                key.to_string()
            } else {
                format!("{prefix}_{key}")
            };

            if !value.0.is_empty() {
                m.insert(prefix_key.clone());
            }
            if !value.1.is_empty() {
                m.insert(prefix_key.clone());
                m.extend(value.flatten(&prefix_key))
            }
        }

        m
    }

    /// Get node value full key name
    ///
    /// `node.get("abc_def")` => `node.get("abc").get("def")`
    pub(crate) fn get(&self, k: &str) -> Option<&Node> {
        match k.split_once('_') {
            None => self.1.get(k),
            Some((k, remain)) => match self.1.get(k) {
                None => None,
                Some(node) => node.get(remain),
            },
        }
    }

    /// Push into node with full key name.
    ///
    /// `node.push("abc_def", v)` => `node.push("abc", "").push("def", v)`
    fn push(&mut self, k: &str, v: &str) {
        match k.split_once('_') {
            None => {
                self.1
                    .entry(k.to_string())
                    .or_insert_with(|| Node::new(String::default()))
                    .0 = v.to_string();
            }
            Some((k, remain)) => match self.1.get_mut(k) {
                None => {
                    let mut node = Self::new(String::default());
                    node.push(remain, v);
                    self.1.insert(k.to_string(), node);
                }
                Some(node) => {
                    node.push(remain, v);
                }
            },
        };
    }

    /// Construct full tree from an iterator.
    pub(crate) fn from_iter<Iter, S>(iter: Iter) -> Self
    where
        S: AsRef<str>,
        Iter: IntoIterator<Item = (S, S)>,
    {
        let mut root = Node::new(String::default());

        let vars = iter
            .into_iter()
            .map(|(k, v)| (k.as_ref().to_lowercase(), v))
            .filter(|(_, v)| !v.as_ref().is_empty());

        for (k, v) in vars {
            root.push(&k, v.as_ref())
        }

        root
    }

    /// Construct full tree from an iterator with prefix.
    pub(crate) fn from_iter_with_prefix<Iter, S>(iter: Iter, prefix: &str) -> Self
    where
        S: AsRef<str>,
        Iter: IntoIterator<Item = (S, S)>,
    {
        let prefix = format!("{}_", prefix);
        let mut root = Node::new(String::default());

        let vars = iter.into_iter().filter_map(|(k, v)| {
            if v.as_ref().is_empty() {
                None
            } else {
                k.as_ref()
                    .strip_prefix(&prefix)
                    .map(|k| (k.to_lowercase(), v))
            }
        });

        for (k, v) in vars {
            root.push(&k, v.as_ref())
        }

        root
    }

    /// Construct full tree from env.
    pub(crate) fn from_env() -> Self {
        Node::from_iter(env::vars())
    }

    /// Construct full tree from env with prefix.
    pub(crate) fn from_env_with_prefix(prefix: &str) -> Self {
        Node::from_iter_with_prefix(env::vars(), prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut root = Node::new("");

        root.push("a_b_c_d", "Hello, World!");
        root.push("a_b_c_e", "Hello, Mars!");
        root.push("a_b_f", "Hello, Moon!");

        assert_eq!(root.get("a_b_c_d"), Some(&Node::new("Hello, World!")));
        assert_eq!(root.get("a_b_c_e"), Some(&Node::new("Hello, Mars!")));
        assert_eq!(root.get("a_b_f"), Some(&Node::new("Hello, Moon!")));
        assert_eq!(
            root.get("a_b_c"),
            Some(&Node(
                "".to_string(),
                BTreeMap::from([
                    ("d".to_string(), Node::new("Hello, World!")),
                    ("e".to_string(), Node::new("Hello, Mars!"))
                ])
            ))
        );
    }

    #[test]
    fn test_push() {
        let mut root = Node::new("");

        root.push("a_b_c_d", "Hello, World!");
        root.push("a_b_c_e", "Hello, Mars!");
        root.push("a_b_f", "Hello, Moon!");
        root.push("a", "Hello, Earth!");

        assert_eq!(
            root,
            Node(
                "".to_string(),
                BTreeMap::from([(
                    "a".to_string(),
                    Node(
                        "Hello, Earth!".to_string(),
                        BTreeMap::from([(
                            "b".to_string(),
                            Node(
                                "".to_string(),
                                BTreeMap::from([
                                    (
                                        "c".to_string(),
                                        Node(
                                            "".to_string(),
                                            BTreeMap::from([
                                                ("d".to_string(), Node::new("Hello, World!")),
                                                ("e".to_string(), Node::new("Hello, Mars!"))
                                            ])
                                        )
                                    ),
                                    ("f".to_string(), Node::new("Hello, Moon!"))
                                ])
                            )
                        )])
                    )
                )])
            )
        )
    }

    #[test]
    fn test_flatten() {
        let mut root = Node::new("");

        root.push("a", "Hello, World!");
        root.push("a_b_c_d", "Hello, World!");
        root.push("a_b_c_e", "Hello, Mars!");
        root.push("a_b_f", "Hello, Moon!");

        let mut expected = BTreeSet::<String>::new();
        expected.insert("a".to_owned());
        expected.insert("a_b".to_owned());
        expected.insert("a_b_c".to_owned());
        expected.insert("a_b_c_d".to_owned());
        expected.insert("a_b_c_e".to_owned());
        expected.insert("a_b_f".to_owned());

        assert_eq!(root.flatten(""), expected);
    }
    #[test]
    fn test_prefix() {
        std::env::set_var("TEST_ENV_VAR", "Hello, World!");
        let root = Node::from_env_with_prefix("TEST_ENV");
        assert_eq!(root.get("var"), Some(&Node::new("Hello, World!")));
    }
}
