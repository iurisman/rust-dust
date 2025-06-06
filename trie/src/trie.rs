use std::fmt::Debug;
use crate::trie_node::*;

pub struct Trie {
    root: TrieNodeMapValue,
    size: usize,
}

impl Debug for Trie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Trie (root: {:?}, size: {})", self.root, self.size)
    }
}
impl Trie {

    pub fn new() -> Self {
        Trie{root: TrieNodeMapValue::new(), size: 0}
    }

    pub fn insert(&mut self, token: &str) {
        let mut curr_map_value = &mut self.root;
        for char in token.chars() {
            let next_map_value = curr_map_value.child_map.0.entry(char).or_insert(TrieNodeMapValue::new());
            curr_map_value = next_map_value;
        }
        curr_map_value.eow = true;
        self.size += 1;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn contains(&mut self, token: &str) -> bool {
        let mut curr_map_value = &self.root;
        for char in token.chars() {
            match curr_map_value.child_map.0.get(&char) {
                None => return false,
                Some(next_map_value) => {
                    curr_map_value = next_map_value;
                }
            }
        }
        curr_map_value.eow
    }
}

#[cfg(test)]
mod tests {
    use std::cell::LazyCell;
    use std::sync::LazyLock;
    use regex::Regex;
    use super::*;
    #[test]
    fn test_small() {
        let mut trie = Trie::new();
        assert_eq!(trie.size(), 0);
        assert!(!trie.contains(&""));
        assert!(!trie.contains(&"a"));
        assert!(!trie.contains(&"apple"));
        trie.insert("apple");
        assert_eq!(trie.size(), 1);
        assert!(!trie.contains(&""));
        assert!(!trie.contains(&"a"));
        assert!(!trie.contains(&"ap"));
        assert!(!trie.contains(&"app"));
        assert!(!trie.contains(&"appl"));
        assert!(trie.contains(&"apple"));
        assert!(!trie.contains(&"apples"));

        trie.insert("orange");
        assert_eq!(trie.size(), 2);
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
        assert_eq!(trie.size(), 3);
        assert!(!trie.contains(&"o"));
        assert!(!trie.contains(&"or"));
        assert!(!trie.contains(&"ora"));
        assert!(!trie.contains(&"oran"));
        assert!(!trie.contains(&"orang"));
        assert!(trie.contains(&"orange"));
        assert!(trie.contains(&"oranges"));
    }

    const PUNCT_RE:LazyCell<Regex> =
        LazyCell::new(|| Regex::new(r#"[\p{Punct}]"#).unwrap());

    /// We care about all chars except punctuation;
    fn validator(c: &char) -> bool {
        !PUNCT_RE.is_match(&c.to_string())
    }

    #[test]
    fn test_big() {
        use rust_dust_lib::token::Tokenizer;
        let mut trie = Trie::new();
        let tokenizer = Tokenizer::new_with_validator(validator);
        let mut word_count = 0;
        // tokenize this file
        for token in tokenizer.from_file("auden.txt") {
            trie.insert(&token);
            word_count += 1;
        }
        for token in tokenizer.from_file("auden.txt") {
            assert!(trie.contains(&token));
        }
        assert_eq!(trie.size(), word_count);
        assert!(trie.contains(&"WH"));
        assert!(!trie.contains(&"wh"));
        assert!(trie.contains(&"Auden"));
        assert!(!trie.contains(&"Pound"));
        assert!(trie.contains(&"Hephaestos"));
        assert!(!trie.contains(&"hephaestos"));
    }
}