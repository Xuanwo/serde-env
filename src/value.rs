use std::collections::BTreeMap;
use std::env;

use log::debug;

/// Node represents a tree of env values.
///
/// Every env will be seperated by `_` in key to construct this tree.
///
/// - `ABC=123` => `Node("123", {})`
/// - `ABC_DEF=123` => `Node("", { "DEF": Node("123", {}) })`
/// - `ABC=123,ABC_DEF=456` => `Node("123", { "DEF": Node("456", {}) })`
#[derive(Debug, PartialEq, Clone)]
pub struct Node(String, BTreeMap<String, Node>);

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

    /// Get children from node.
    pub fn children(&self) -> &BTreeMap<String, Node> {
        &self.1
    }

    /// Get node value full key name
    ///
    /// `node.get("abc_def")` => `node.get("abc").get("def")`
    pub fn get(&self, k: &str) -> Option<&Node> {
        debug!("get key: {}", k);

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
        debug!("try to push value: {}, {}", k, v);

        match k.split_once('_') {
            None => {
                self.1.insert(k.to_string(), Self::new(v));
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

        assert_eq!(
            root,
            Node(
                "".to_string(),
                BTreeMap::from([(
                    "a".to_string(),
                    Node(
                        "".to_string(),
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
}
