
## Now with Error Handling

There's a clear problem with the current implementation: it's a happy code path. We've skirted the possibility of 
an error with all the calls to `unwrap()`. This would be perfectly fine in unit tests, where failing fast is
appropriate, the prouct code must handle errors gracefully. For an added perspective, I start with an overview
of exception handling before Rust. If you'd rather continue reading about Rust, skipe to section 2.

### 1. Evolution of Error Handling before Rust
Rust's error handling is, in a way, a throwback to the 1960s, when languages had no support
for exceptions. Back then, each fallible operation (typically involving the operating
system, like file read) returned the status code which had to be acted upon locally. For
example, in Fortran IV (IBM, 1956) an I/O error handler was a line label to which the
program long-jumped in case of an error:
```fortran
READ (unit, format, ERR=100) variable
100 CONTINUE
! Handle error here
```
Things were even iffier when it came to the errors that were thrown entirely by the
user code, like division by zero or integer overflow. In response, PL/I (IBM, 1964)
was the first language to offer exceptions that could be used to handle such conditions:
```pl/i
ON ZERODIVIDE BEGIN;
    PUT SKIP LIST('Error: Division by zero detected!');
    /* Handle error */
END;
```
This advance was made possible by an important development in compiler design; from a mechanical
translator of the high level code to the machine instructions, compilers began inserting
logic that was never written by the programmer, e.g. runtime checks if the denominator is
zero. Moreover, what compilers had to do to implement certain features depended on the
target instruction set and the OS, so compilers began doing different things for
different environments.

By the time the C language was released (Bell Labs, 1974) the concept of exceptions
was well understood by language designers. And yet, Dennis Ritchie left it out entirely
for two reasons:
* Simplicity and performance. The original C compiler did not insert any logic that was not
  written by the programmer.
* Interoperability. This was the time of many new operating systems and processors,
  and in order for a C program behave the same way in different environments, it had to be
  minimalistic.

This omission was becoming increasingly clear, so C++ (1985, AT&T Bell Labs) added exceptions
— to the chagrin of many programmers. Without the support of a VM, exceptions proved unsafe
and expensive. Many teams banned the use of exceptions in C++, forcing programmers to
develop explicit error handling frameworks.

Since then, all new computer languages like Java, C#, Erlang all run on a VM, enabling
safe and efficient exception handling. (Not to mention all the interpreted languages
like Ruby and Python, which _are_ their own VM.)

The price of this high-level convenience is low-level inefficiency. Many use cases, like
systems programming, cannot afford a VM, and some use cases, like embedded systems, cannot 
even support it. Until recently, these use cases have relied squarely on C or C++. The
big change came in 2003 with the release of LLVM (Low Level Virtual Machine), a byte-code
and compiler infrastructure that permitted rapid development of new low-level languages
running directly on the processor, without a runtime VM. By 2010 two such languages were
released: Go (2007, Google) and Rust (2009, Mozilla) — both ditching exceptions in favor
of error propagation via complex return types.

### 2. Living without Exceptions

To reiterate, exceptions, at least in the modern sense of the word, have these drawbacks:
* Runtime overhead. Note, for example that Scala (2004, EPFL), an advanced dual paradigm
  language that compiles to the Java bytecode, uses Java's `try/catch/finally` blocks and
  the `throw` statement, just like Java, because these concepts are built into the JVM.
  (Most Scala programmers prefer to use the `Try` type, which hides these imperative verbs
  behind a more functional flow.)
* A source of potential resource leaks due to early termination.
* Undermines compiler's ability to reason about the source code.

For these reasons Rust does not support exceptions, at least the kind that can be caught.
Instead, Rust offers two error handling mechanisms: _panic_ for non-recoverable
exceptions that typically lead to termination of the panicking thread, and `Result` for
recoverable errors.

#### 2.1. Panic

Panics are meant to be used for systemic unrecoverable errors. It can be triggered
explicitly with the `panic!` macro, or is triggered implicitly by one of the following
shortcuts:
* Calling `unwrap()` on `Result` if it is `Err`. `Result` is the subject of the next
  section.
* Calling `unwrap()` on `Option` if it is `None`.
* Calling `expect()` on either `Result` or `Option`, which is just a variation of `unwrap()'
  that allows the caller to attach last words to the panic.
* Various arithmetic exceptions, such as division by zero and integer overflow.
* Out-of-bounds array index.

Panic is thread-local; a panic in a non-main thread will terminate the thread,
but not the process. There are however errors that are more disruptive than panics,
the out-of-memory error. At this time of this writing it does not cause panic but rather
terminates the process regardless of what thread received it.

On panic, rust compiler attempts to unwind the call stack from the point of panic to the
entry point into the current thread and cleanup all heap allocations owned by stack
values. This is not guaranteed to succeed, because there's no requirement that each struct
overrides the default implementation of `Drop`. Consequently, repeatedly panicking threads
may end up leaking memory.

Even though panic is reserved for non-recoverable errors, the standard library does
provide a way to recover from panic with `std::panic::catch_unwind()` and even to
trigger a custom panic with `std::panic::panic_any()` which takes an arbitrary type that
can be accessed later at the point of recovery. This mechanism however is not meant for
mimicking exception handling a la Java, but for libraries to be able to localize
their panics instead of making the library users to deal with the unexpected panics
coming from 3rd party crates.

#### 2.2. The `Result` Type

All user errors, like trying to read a file that doesn't exist, and recoverable system
errors, like timing out on a network call, are meant to be handled with the `Result` type.
It's the type that is returned by any library, standard or not, so my task as a consumer
of those libraries is to correctly handle the `Result` they return by either recovering 
from the error, like retrying the failed operation, or propagating it up the call stack 
to be handled by a caller.

#### 2.3. Implicit `Error` Propagation

In a well organized codebase, each fallible function returns an
object of type `Result<T,E>`, where `T` is the good result, if the function succeeded,
and `E` is the error type otherwise. `Result` is an enum populated by two instances.
`Ok(T)` wraps the successful return object, while `Err(E)` wraps the error object.
Both `T` and `E` are objects of any type. There's absolutely no expectation on what
user functions can return, although there's a bit of commonly used syntactic sugar,
which makes error propagation ergonomic: `some_result_value?` desugars to
```rust
match some_result_value {
    Ok(val) => val,
    Err(err) => return Err(From::from(err))
}
```
Which is to say that if some value of type `Result<T,E>` is a success, `?` unboxes the `T`,
but if it's a failure, `?` short-circuits out of the function with the possibly converted value
of `E`, boxed in `Err`. If you do nothing, `From::from(err)` returns the `err` value itself, 
thanks to the blanket identity implementation of `From<T>`:
```rust
impl<T> From<T> for T {
    fn from(t: T) -> T { t }
}
```
This nuance is what enables us to implicitly convert from one error type to another, as we propagate it
up the stack. This is important, because most crates use their own error types, exposing data pertinent 
to the kinds of errors the library may encounter. Thus, programmers typically have to deal with several
error types, converting them to some new error type.  We will see how this automatic conversion works
in {todo}.

#### 2.4. Explicit `Error` Propagation (V1)
source: token_with_result_v1.rs

I start by defining our custom tokenizer error would likely want to expose this contextual information:
```rust
#[derive(Debug)]
pub struct TokenizerError {
    // Bad token, if any
    pub token: Option<String>,
    // Error message
    pub message: String
}
```

Let's start with `from_buf_reader()`, whose original implementation was as follows:
```rust
/// Read tokens from a reader
pub fn from_buf_reader<R: Read>(&self, reader: R) -> impl Iterator<Item=String> {
    BufReader::new(reader).lines()
        .map(|res| res.unwrap())
        .map(|str| str.chars().filter(|c| (self.validator)(c)).collect::<String>())
        .flat_map(|line| line.split_whitespace().map(String::from).collect::<Vec<String>>())
}
```
The only fallible call here is `BufReader.lines()`, which returns an iterator over parse results containing either the 
parsed line as a string or an error if the byte array contained non UTF-8 character. We will let the caller process 
the errors by returning  `impl Iterator<Item=Result<String, TokenizerError>>`. 

The name of the game here is to replace the call to `unwrap()` with something that propagates the error up the call
stack, instead of panicking. Because the call to `unwrap()` is inside a closure, we cannot use the `?` syntax
to return from the containing function. Instead, we map the successful result to its filtered version. The 
flat map also receives a `Result` as the argument and maps it to an iterator of `Result`s to be flattended into
the invoking iterator.

```rust
/// Read tokens from a reader
    pub fn from_buf_reader<R: io::Read>(&self, reader: R) -> impl Iterator<Item=Result<String, TokenizerError>> {
        io::BufReader::new(reader).lines()
            .map(|res_line|
                res_line.map(|line|
                    line.chars().filter(|c| (self.validator)(c)).collect::<String>()
                )
            )
            .flat_map(|res_line|
                match res_line {
                    Err(err) =>
                        vec![Err(TokenizerError::from(err))],
                    Ok(line) =>
                        line.split_whitespace()
                            .map(|str| Ok(String::from(str)))
                            .collect::<Vec<Result<String, _>>>()
                }
            )
    }
```

Finally, I fix the `from_file()` method, whose original implementation was as follows:
```rust
    pub fn from_file(&self, filename: &str) -> impl Iterator<Item=String> {
        let file = File::open(filename).unwrap();
        self.from_buf_reader(file)
    }
```
Here, the call to `unwrap()` is not inside a closure so we can take advantage of the `?` syntax:

```rust
/// Read tokens from a file
pub fn from_file(&self, filename: &str)
    -> Result<impl Iterator<Item=Result<String, TokenizerError>>, TokenizerError>
{
    Ok(self.from_buf_reader(fs::File::open(filename)?))
}
```
Note the implicit conversion from `io::Error`, returned by `fs::File::open()`, to `TokenizerError`.
This is possible because we provided an implementation of the `From` trait that covers exactly this use case.:
```rust
impl From<io::Error> for TokenizerError {
    fn from(error: std::io::Error) -> Self {
        TokenizerError {message: format!("{}", error), token: None}
    }
}
```
We can now add a new test case for the file not found error:
```rust
#[test]    
fn test_io_error() {
    let tokenizer = Tokenizer::new_with_validator(validator);
    match tokenizer.from_file("./bad.txt") {
        Ok(_) => assert!(false),
        Err(err) => assert!(
            matches!(err, TokenizerError::Io(foo) if foo.kind() == io::ErrorKind::NotFound)
        ),
    }
}
```

### 3. Further Discussion (V2)
Source: token_with_result_v2.rs

#### 3.1. The Problem

The solution we developed in V1 is already much better than the original tokenizer, because we've replaced panics
with orderly statically typed error handling. The one last wrinkle to smooth out is the unsightly return type 
`Result<impl Iterator<Item=Result<String, TokenizerError>>, TokenizerError>` returned by `from_file()`. If the caller
to be able to tell apart the two `TokenizerError`s, we'd have to expose implementation details that need not be
exposed.

Rather, I want to expose only one error, while keeping the return type as an `impl Iterator`. This means that the
error returned by `fs::File::open()` must be repackaged in a single element iterator. Something like this:
```rust
    /// Read tokens from a file
    pub fn from_file(&self, filename: &str)
        -> impl Iterator<Item=Result<String, TokenizerError>>
    {
        match fs::File::open(filename) {
            Ok(file) => self.from_buf_reader(file),
            Err(error) => vec![Err(TokenizerError::from(error))].into_iter()
        }
    }
```
This would work fine in an OO language, like Scala, where the actual implementation would be determined at runtime.
Rust won't compile this:
```rust
   = note: expected opaque type `impl Iterator<Item = Result<String, token_with_result_v2::TokenizerError>>`
                   found struct `std::vec::IntoIter<Result<_, token_with_result_v2::TokenizerError>>`
help: you could change the return type to be a boxed trait object
   |
34 -         -> impl Iterator<Item=Result<String, TokenizerError>>
34 +         -> Box<dyn Iterator<Item=Result<String, TokenizerError>>>
   |
help: if you change the return type to expect trait objects, box the returned expressions
   |
37 ~             Ok(file) => Box::new(self.from_buf_reader(file)),
38 ~             Err(error) => Box::new(vec![Err(TokenizerError::from(error))].into_iter())
   |
```
The hint suggests that we solve this problem with the familiar technique of `Box`ing the return type. 
We've already encountered this when we implemented recursive `Stack` type. The difference here is that the reason 
compiler can't determine the actual type is one of two possible opaque type. Here again, we could
return trait object `<dyn Iterator<...>>` to make both arms to resolve to the same sized static type. However,
I don't want to change the return type to `Box<dyn Iterator<...>>`. Rather, I'd like to solve what is
likely to be a general problem: how to return one of several opaque types implementing, implementing `Iterator`.

So far, I've found two ways to make the Rust compiler do the work for us: by using enums or by chaining the two
iterators with `Iterator.chain()`.

#### 3.1. Abstracting Over Opaque Types with `enum`s
Enums are additive types which unite arbitrary types in a single type. To unite the two different iterator types,
we create a new enum `TokenizerIter` which one of the two possible arms of the match statement above:
```rust
pub enum TokenizerIter<I1,I2> {
    Iter1(I1),
    Iter2(I2),
}
```

In order to use `TokenizerIter` in place of `impl Iterator<Item=Result<String, TokenizerError>>` it needs to implement
`Iterator` with that item type:
```rust
impl<I1: Iterator<Item=Result<String,TokenizerError>>, I2: Iterator<Item=Result<String,TokenizerError>>>
Iterator for TokenizerIter<I1, I2> {
    type Item = Result<String, TokenizerError>;
    fn next(&mut self) -> Option<Result<String, TokenizerError>> {
        match self {
            Self::Iter1(iter1) => iter1.next(),
            Self::Iter2(iter2) => iter2.next(),
        }
    }
}
```
This is it, really!

#### 3.4. Abstracting Over Opaque Types with `either`s

Except, we just reinvented the `Either` enum, available from the `either` crate. (Will it make it
into the standard library, like in Scala and Haskell?)

```rust
pub enum Either<L, R> {
    /// A value of type `L`.
    Left(L),
    /// A value of type `R`.
    Right(R),
}
```
It's symmetric, has lots of useful methods, and, in particular, implements `Iterator`. We can just use it without
worrying about implementing anything ourselves:
```rust
pub fn from_file_either(&self, filename: &str)
                 -> impl Iterator<Item=Result<String, TokenizerError>>
{
    match fs::File::open(filename) {
        Ok(file) => Either::Left(self.from_buf_reader(file)),
        Err(error) => Either::Right(vec![Err(TokenizerError::from(error))].into_iter())
    }
}
```

Note, that since `Either` is symmetric, I can combine them to create more branches. For example to have three
branches `x,y.z`, I could do `(Left(x), Right(Left(y), Right(z)))`.

#### 3.4. Chaining Iterators
It turns out, we don't even need an explicit sum type to unify the two different opaque implementors of
`Iterator`. We can let the compiler do that for as as well:
```rust
pub fn from_file_chain(&self, filename: &str)
    -> impl Iterator<Item=Result<String, TokenizerError>>
{
    let (iter1_opt, iter2_opt) =
        match fs::File::open(filename) {
            Ok(file) => (Some(self.from_buf_reader(file)), None),
            Err(error) => (None, Some(vec![Err(TokenizerError::from(error))]))
        };
    iter1_opt.into_iter().flatten().chain(iter2_opt.into_iter().flatten())
}
```


### 4. Preserving Backtrace


A look at the docs for `stc::io::Error` reveals the `source()` method,
 which returns the cause of the I/O error. In this case it's `None`, because the underlying error did not
 originate in the user space, but by the OS. However, to be good citizens we should attach
 this I/O error as the source of our error to help clients of our library debug their errors. 