use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fmt::Result;
pub struct TrieNode(
    pub(crate) HashMap<char, TrieNodeMapValue>
);

impl TrieNode {
    pub(super) fn size(&self) -> usize {
        self.0.iter().fold(
            0,
            |acc, (_, node_val)|
                acc + 1 + node_val.child_map.size()
        )
    }
}

impl Debug for TrieNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "TrieNode (0 = {:?})", self.0)
    }
}
pub(super) struct TrieNodeMapValue {
    pub(super) eow: bool,
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