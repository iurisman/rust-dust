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
`TrieNodeMapValue` is allocated on the stack as part of the `Trie` struct (below), and even then, only 
the child map's
header is stored with the structure. The map's content is managed by `HashMap`'s implementation on the
heap.

Likewise, the destruction is cleanly handled by the default `Drop` implementation, which recursively
calls the destructors of all the map elements. This is fine, because the depth of the recursion is
limited by the longest token, which in a typical use case is under 20. However, if you need to use
this `Trie` for applications where the tokens may be thousands of characters long, an explicit `Drop`
implementation will be needed to avoid recursive stack overflows.

Note as well, that I've implemented `Debug` for all the types to be able to print them. Rust
can derive `Debug` implimentation (if we ask so with `#[derive(Debug)])`) for some custom types,
but there are limitations, in particular it doesn't seem to do it for recursive structures.

The final code for `Trie`:
```rust
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
```

One last note. One of the tests uses [file tokenizer](../lib/src/token.rs). There, we defined the tokenizer
constructor as `pub fn new_with_validator(validator: fn(&char) -> bool)`, which accepts only named functions,
but not anonymous closures. The fundamental difference between the two is that closures close over values
in the containing syntactical context, while functions don't. (Unlike Scala, where a function has access
to values outside its body.) This decision was motivated by a simpler declaration, but now we have to be
more verbose and define the function `validator(c: &char)`:

```rust
const PUNCT_RE:LazyCell<Regex> =
    LazyCell::new(|| Regex::new(r#"[\p{Punct}]"#).unwrap());

/// We care about all chars except punctuation;
fn validator(c: &char) -> bool {
    !PUNCT_RE.is_match(&c.to_string())
}

fn test_big() {
    use rust_dust_lib::token::Tokenizer;
    let mut trie = Trie::new();
    let tokenizer = Tokenizer::new_with_validator(validator);
    let mut word_count = 0;
    for token in tokenizer.from_file("auden.txt") {
        trie.insert(&token);
        word_count += 1;
    }
    ...
}
```

The validator function uses a regular expression to filter out punctuation. I could have defined the
regex inside the validator function, but that would have meant re-instantiating it for every character
in the stream! Instead, we define it statically and only once. Because the size of the resulting regex
can only determinted after it is constructed at run time, I use the `LazyCell` facility for lazy definition
of static values. It gets initialized only once, the first time it is accessed.

Note as well, that we must use `const` with `LazyCell` because `const` implies static lifetime + immutability.
We could have use `static` instead, which implies static lifetime, but potentially mutable value, but the
compiler would not accept it because it knows that `LazyCell` is not thread safe. `LazyLock` is the thread
safe version of `LazyCell`, but its thread safety comes with a runtime overhead which we should only
have to pay if we're designing for a multithreaded environment:
```rust
static PUNCT_RE:LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[\p{Punct}]"#).unwrap());

```
