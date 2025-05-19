## Rust Dust Lib
Let's organize some of the exercises into a library with the goal of learning how to build one. Although
all the problems in Rust Dust would be candidates for a library distribution, it's more straightforward
to develop them as simple binaries. Except for those problems which I want to reuse elsewhere in the
Rust Dust project — these are the ones included in Rust Dust Lib.

### 1. How to Build a Simple Rust Library
_Note, that this section applies to Rust 2018 or later._

A Rust library's entry point is `lib.rs` at the top level of the `src` hierarchy. Just like it is
the case with `main.rs` for binary distributions, `lib.rs` declares modules with the `mod`
declaration, optionally with the `pub` qualifier if the module is to be visible to the library
users. And just like in binary distribution, the likely named file must be found at the top level.
Our `lib.rs` declares `pub mod io` and the contents of the `io` module are inside `io.rs`.

This is the simplest possible way to define a Rust library that entirely depends on `rustc`'s defaults.
In general, [Rust's module system](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
) is complex and flexible, supporting a complete decoupling of
the logical structure as seen by the library users and the physical source file organization inside
the crate.

Local projects can now use this library by adding it as a local dependency in their `Cargo.toml`
files:
```toml
rust-dust-lib = { path = "path/to/lib/project" }
```
where `path/to/lib/project` is the path to the library project's top level directory containing
its `Cargo.toml` file. It may be absolute or relative of the client project's `Cargo.toml` file.

### 2. File Tokenizer
Source: token.rs

Problem: iterate over individual words in a file without allocating the entire
file in memory.

The initial intuition is to compose two std library methods `BufReader.lines()`, returning the file's
contents as a `String` `Iterator`, and `String.split_whitespace()`, returning an iterator of string sub-slices:
```rust
fn from_file(filename: &str) -> impl Iterator<Item=String> {
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
 fn read_tokens(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines()
        .map(|res| res.unwrap())
        .flat_map(|line| line.split_whitespace().map(String::from).collect::<Vec<String>>())
}
```
Here's why this works:
* `line.split_whitespace()` returns an iterator over tokens in a single line as `&str` slices;
* `String::from` copies them into instances of `String`s, owned by the enclosing mapping function;
* `collect()` collects them into `Vec<String>` returned by `flat_map()`;
* The Rust compiler implicitly calls `to_iter()` on the `Vec<String>` inside `flat_map()` to turn it into an iterator
that can be flat-mapped into the iterator returned by `lines()`

But we're not done yet. Most applications for a tokenizer call for an alphabet — the set of valid characters. 
Different use cases may call for different alphabets. To support alphabets, we'll let the caller submit a function 
`fn(char) -> boolean` which does custom validation. Another improvement we want to make is to expose a more general
method abstracting over the input source. The good candidate for an abstract source of bytes is the trait 
`std::io::Read`. Any `Read`er can be wrapped into a `BufReader`, which provides an efficient way of reading a stream
of bytes line by line, which is what we do internally.

```rust
struct Tokenizer {
    validator: fn(&char) -> bool,
}

impl Tokenizer {
    
    fn default_validator(_: &char) -> bool {true}
    
    pub fn new() -> Tokenizer {
        Tokenizer { validator: Self::default_validator}
    }
    
    pub fn new_with_validator(validator: fn(&char) -> bool) -> Tokenizer {
        Tokenizer {validator}
    }

    /// Read tokens from a file
    pub fn from_file(&self, filename: &str) -> impl Iterator<Item=String> + use<'_> {
        let file = File::open(filename).unwrap();
        self.from_buf_reader(file)
    }

    /// Read tokens from any `Read`er
    pub fn from_buf_reader<R: Read>(&self, reader: R) -> impl Iterator<Item=String> + use<'_, R> {
        BufReader::new(reader).lines()
            .map(|res| res.unwrap())
            .map(|str| str.chars().filter(|c| (self.validator)(c)).collect::<String>())
            .flat_map(|line| line.split_whitespace().map(String::from).collect::<Vec<String>>())
    }
}
```
A couple of additional observations:

* The type `fn(&char) -> bool` is the simplest form of a function pointer; its size is known at compile time, so it
  can be allocated on the stack. Only named functions have this type, but not closures. In other words, only the 
  name of a named function can be passed to `Tokenizer::new_with_validator()`, but not a closure. If we also want to pass a closure,
  that captures (moves or borrows) values from the environment, `validator`'s type must be boxed, requiring runtime
  allocation on the heap and dynamic dispatch, because the memory size of a closure depends on the types of captured
  values.
```rust
struct Tokenizer {
    validator: Box<dyn Fn(char) -> bool>
}
```
* The ``use<`_,R>`` clause in the return types are the new syntax in Rust 2024. I don't quite understand all the nuances,
  but the general sense is that even though a concrete implementation of `Read` (which is what the type parameter `R`
  stands for) contains internal references, their lifetimes can be elided and will be inferrable at the call site.
