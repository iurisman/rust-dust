use std::collections::HashMap;
use std::fmt::Debug;
use crate::trie_node::*;

pub struct Trie {
    root: TrieNode
}

impl Debug for Trie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Trie (root: {:?})", self.root)
    }
}
impl Trie {

    pub fn new() -> Self {
        Trie{root: TrieNode(HashMap::new())}
    }

    pub fn insert(&mut self, token: &str) {
        let mut curr_node_map = &mut self.root.0;
        let token_size = token.chars().count();
        for (char, ix) in token.chars().zip(1..=token_size) {
            let map_value = curr_node_map.entry(char).or_insert(TrieNodeMapValue::new());
            curr_node_map = &mut map_value.child_map.0;
            // If this is the last character, set current node's eow to true
            if ix == token_size {
                map_value.eow = true;
            }
        }
    }

    pub fn size(&self) -> usize {
        self.root.size()
    }

    pub fn contains(&mut self, token: &str) -> bool {
        let mut curr_node_map = &self.root.0;
        let token_size = token.chars().count();
        for (char, ix) in token.chars().zip(1..=token_size) {
            match curr_node_map.get(&char) {
                None => return false,
                Some(next_map_value) =>
                    if ix == token_size && next_map_value.eow == true {
                        return true;
                    } else {
                    curr_node_map = &next_map_value.child_map.0
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert() {
        let mut trie = Trie::new();
        assert_eq!(trie.size(), 0);
        assert!(!trie.contains(&""));
        assert!(!trie.contains(&"a"));
        assert!(!trie.contains(&"apple"));
        trie.insert("apple");
        assert_eq!(trie.size(), 5);
        assert!(!trie.contains(&""));
        assert!(!trie.contains(&"a"));
        assert!(!trie.contains(&"ap"));
        assert!(!trie.contains(&"app"));
        assert!(!trie.contains(&"appl"));
        assert!(trie.contains(&"apple"));
        assert!(!trie.contains(&"apples"));

        trie.insert("orange");
        assert_eq!(trie.size(), 11);
        assert!(!trie.contains(&""));
        assert!(!trie.contains(&"a"));
        assert!(!trie.contains(&"ap"));
        assert!(!trie.contains(&"app"));
        assert!(!trie.contains(&"appl"));
        assert!(trie.contains(&"apple"));
        assert!(!trie.contains(&"apples"));
        assert!(!trie.contains(&"o"));
        assert!(!trie.contains(&"or"));
        assert!(!trie.contains(&"ora"));
        assert!(!trie.contains(&"oran"));
        assert!(!trie.contains(&"orang"));
        assert!(trie.contains(&"orange"));
        assert!(!trie.contains(&"oranges"));

        assert!(!trie.contains(&"pear"));

        trie.insert("oranges");
        assert!(!trie.contains(&"o"));
        assert!(!trie.contains(&"or"));
        assert!(!trie.contains(&"ora"));
        assert!(!trie.contains(&"oran"));
        assert!(!trie.contains(&"orang"));
        assert!(trie.contains(&"orange"));
        assert!(trie.contains(&"oranges"));

    }
}