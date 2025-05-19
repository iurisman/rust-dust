## Trie Tree

Trie (prefix tree) is a common, space-efficient way of storing a set of valid strings (e.g. english words). 
Although a hash table would offer a better time complexity, a trie is typically preferred when the number
of words is large, because trie stores all common prefixes only once. Thus, `achtung` and `achilles` will
share the `ach`.

The top level type `Trie` contains the root level `TrieNode` and the `size` field, which keeps track of
the overall size (number of words contained) of the tree. Each `TrieNode` is simply a `HashMap`,
mapping the current character to the next character. Thus, to find out if the word `Hephaestos` is contained
in the trie, we'll need to traverse the tree one character per node, and if the map value for the last `s`
has its `eow` (end of word) is set to true.

The code for `TrieNode`:
```rust
pub struct TrieNode(
    pub HashMap<char, TrieNodeMapValue>
);

impl Debug for TrieNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "TrieNode (0 = {:?})", self.0)
    }
}
pub(super) struct TrieNodeMapValue {
    // Is the char mapping to this value end of a valid word?
    pub(super) eow: bool,
    // Possible continuations.
    pub(super) child_map: TrieNode
}

impl TrieNodeMapValue {
    pub(super) fn new() -> TrieNodeMapValue {
        TrieNodeMapValue{eow: false, child_map: TrieNode(HashMap::new())}
    }
}

impl Debug for TrieNodeMapValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "TrieNodeMapValue (eow = {}, child_map = {:?})", self.eow, self.child_map)
    }
}
```
The critical thing to note, is that although `TrieNode` is a recursive structure, we did not have to 
mess around with `Box`ing things up, as we had to with the linked list. (Perhaps I should have started
the series with trie, instead of list!) That is because `HashMap` is doing it for us. Only the root level
`TrieNode` is allocated on the stack as part of the `Trie` struct (below), and eventhen, only the map's
header is stored with the structure. The map's content is managed, by `HashMap`'s implementation, on the
heap.

Note as well, that I've implemented `Debug` for all the types to be able to print them for debugging. Rust
can derive `Debug` implimentation (if we ask so with `#[derive(Debug)])`) but there are limitations,
in particular it can't do it for recursive structures.

The final code for `Trie`:
```rust
pub struct Trie {
    root: TrieNode,
    size: usize,
}

impl Debug for Trie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Trie (root: {:?}, size: {})", self.root, self.size)
    }
}
impl Trie {

    pub fn new() -> Self {
        Trie{root: TrieNode(HashMap::new()), size: 0}
    }

    pub fn insert(&mut self, token: &str) {
        let mut curr_child_map = &mut self.root.0;
        let token_size = token.chars().count();
        for (char, ix) in token.chars().zip(1..=token_size) {
            let map_value = curr_child_map.entry(char).or_insert(TrieNodeMapValue::new());
            curr_child_map = &mut map_value.child_map.0;
            // If this is the last character, set current node's eow to true
            if ix == token_size {
                map_value.eow = true;
            }
        }
        self.size += 1;
    }

    pub fn size(&self) -> usize {
        self.size
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
```
