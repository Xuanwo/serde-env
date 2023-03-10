use std::collections::{BTreeMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::{env, fmt};

use crate::cond_log::trace;

/// Node represents a tree of env values.
///
/// Every env will be seperated by `_` in key to construct this tree.
///
/// - `ABC=123` => `Node("123", {})`
/// - `ABC_DEF=123` => `Node("", { "DEF": Node("123", {}) })`
/// - `ABC=123,ABC_DEF=456` => `Node("123", { "DEF": Node("456", {}) })`
#[derive(PartialEq, Clone)]
pub struct Node(String, BTreeMap<String, Node>);

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
    pub fn new(v: &str) -> Self {
        Node(v.to_string(), BTreeMap::new())
    }

    /// Get value from node.
    pub fn value(&self) -> &str {
        &self.0
    }

    /// Into value to get ownership.
    pub fn into_value(self) -> String {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty() && self.1.is_empty()
    }

    pub fn has_children(&self) -> bool {
        !self.1.is_empty()
    }

    pub fn flatten(&self, prefix: &str) -> HashSet<String> {
        let mut m = HashSet::new();

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
    pub fn get(&self, k: &str) -> Option<&Node> {
        trace!("get key: {}", k);

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
        trace!("try to push value: {}, {}", k, v);

        match k.split_once('_') {
            None => {
                self.1
                    .entry(k.to_string())
                    .or_insert_with(|| Node::new(""))
                    .0 = v.to_string();
            }
            Some((k, remain)) => match self.1.get_mut(k) {
                None => {
                    let mut node = Self::new("");
                    node.push(remain, v);
                    self.1.insert(k.to_string(), node);
                }
                Some(node) => {
                    node.push(remain, v);
                }
            },
        };
    }

    /// Construct full tree from env.
    pub fn from_env() -> Self {
        let mut root = Node::new("");

        let vars = env::vars()
            .map(|(k, v)| (k.to_lowercase(), v))
            .filter(|(_, v)| !v.is_empty());
        for (k, v) in vars {
            root.push(&k, &v)
        }

        root
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

        let mut expected = HashSet::<String>::new();
        expected.insert("a".to_owned());
        expected.insert("a_b".to_owned());
        expected.insert("a_b_c".to_owned());
        expected.insert("a_b_c_d".to_owned());
        expected.insert("a_b_c_e".to_owned());
        expected.insert("a_b_f".to_owned());

        assert_eq!(root.flatten(""), expected);
    }
}
