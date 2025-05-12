## Input/Output

### 1. File Tokenizer
Source: read.rs

Problem: iterate over individual words in a file without allocating the entire
file in memory.

The initial intuition is to compose two std library methods `BufReader.lines()`, returning the file's
contents as a `String` `Iterator`, and `String.split_whitespace()`, returning an iterator of string sub-slices:
```rust
// 
fn read_tokens(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines()
        .map(|res| res.unwrap())
        .flat_map(|line| line.split_whitespace())
}
```

This fails with 
```rust
error[E0271]: expected `FlatMap<Map<Lines<BufReader<File>>, {closure@read.rs:7:14}>, SplitWhitespace<'_>, {closure@read.rs:8:19}>` to be an iterator that yields `String`, but it yields `&str`
 --> src/read.rs:4:35
  |
4 | fn read_tokens(filename: &str) -> impl Iterator<Item=String> {
  |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `String`, found `&str`
```

The error, as I understand it, is saying that we're trying to flatten an borrowing iterator of `&str` into an
owning iterator of `String`. Changing the function's return type to iterator of `&str` is a bad idea because
each line is a string owned by this function, so we can't return references to it. Rather, we need to convert
the string slices returned by `split_whitespace()` into `String`s owned by the iterator we're building to return:

```rust
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines()
        .map(|res| res.unwrap())
        .flat_map(|line| line.split_whitespace().map(String::from).collect::<Vec<String>>())
```
Here's why this works:
* `line.split_whitespace()` returns an iterator over tokens in a single line as `&str` slices;
* `String::from` copies them into instances of `String`s, owned by the enclosing mapping function;
* `collect()` collects them into `Vec<String>` returned by `flat_map()`;
* The Rust compiler implicitly calls `to_iter()` on the `Vec<String>` inside `flat_map()` to turn it into an iterator
that can be flat-mapped into the iterator returned by `lines()`

Note, however, that some of the tokens contain punctuation -- likely not what a caller would want. We need therefore
to add an alphabet. It contains the characters that are considered meaningful, while the characters not in alphabet
will be considered as whitespace.