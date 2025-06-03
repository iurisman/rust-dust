
## Now with Error Handling
source: src/token_v2.rs

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

### Living without Exceptions

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

#### Panic

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

#### The `Result` Type
All user errors, like trying to read a file that doesn't exist, and recoverable system
errors, like timing out on a network call, are meant to be handled with the `Result` type.
It's the type that is returned by any library, standard or not, so my task as a consumer
of those libraries is to correctly handle the `Result` they return by
either recovering from the error, like retrying the failed operation, or propagating it
up the call stack to be handled there.

In a well organized codebase, each fallible function returns an
object of type `Result<T,E>`, where `T` is the good result, if the function succeeded,
and `E` is the error type otherwise. `Result` is an enum populated by two instances.
`Ok(T)` wraps the successful return object, while `Err(E)` wraps the error object.
Both `T` and `E` are objects of any type; there's no reason on the part of the language
designers to limit return types of my functions or what constitutes an error.
If you're used to thinking in object-oriented languages, this seems strange: exceptions
are frequently handled up the call stack and across an abstraction boundary from where
they were thrown. In object-oriented languages, this type of behavior transparency is
handled with inheritance, when different error types have a common abstract supertype,
which compels the concrete error types to implement methods that can be used across
abstraction boundary.

[[ Aside to be moved elsewhere ]]

Rust has no type inheritance, but it expresses a similar capability with trait
bounds, which constrain generic parameters to only those types that implement the named
traits. For example, here's the declaration of the `std::boxed::Box` type:
```rust
pub struct Box<T, A = Global>(/* private fields */)
where
    A: Allocator,
    T: ?Sized;
```
It takes two type parameters, both of which are constrained by the trait bounds. Here,
`A = Global` defines the default value for the type param `A`. The question mark in
`?Sized` means somewhat of an inverse idea: it relaxes the implicit trait bound `Sized`,
which otherwise would have been applied. All `struct`s in rust
implement the `Sized` trait, which is to say have a known size at compile time. As we
already saw in the implementation of stack, `Box` provides the way of deferring the
heap allocation of `T`, such that the size of `Box` itself is known, even though the
size of `T` may not be. Thus `?Sized` means that `T` is opted out of the trait `Sized`;
it may but doesn't have to implement it.

[[ End aside]]

In rust, error propagation up the call stack is achieved by either explicit conversion
from one error type to another, or implicitly.

##### Explicit `Error` Propagation without Closures

The reason the issue arises is that almost any library uses its own error type exposing methods pertinent 
to the kinds of errors the library may encounter. Thus, our custom tokenizer error would likely want to 
expose this contextual information:
```rust
#[derive(Debug)]
pub struct TokenizerError {
    // Bad token, if any
    pub token: Option<String>,
    // Error message
    pub message: String
}
```
By the Rust convention, error types are always named `Error`. I'm using `TokenizerError` just for
convenience of reference. Now, our code is responsible for converting the errors we get from other libraries, like
`std::io` and `std::fs` and converting them to `TokenizerError` Let's start with the 
following V1 method:

```rust
    pub fn from_file(&self, filename: &str) -> impl Iterator<Item=String> {
        let file = File::open(filename).unwrap();
        self.from_buf_reader(file)
    }
```
I only need to make two changes: the return type changes to `Result<...>` and the call to
`unwrap()` must be replaced with a `match` that does explicit conversion from `std::io::error::Error` 
to `TokenizerError`:

```rust
    pub fn from_file(&self, filename: &str)
        -> Result<impl Iterator<Item=String>, TokenizerError>
    {
        match File::open(filename) {
            Ok(file) => {
                Ok(self.from_buf_reader(file))
            }
            Err(err) => {  // This is std::io::error::Error
                Err(TokenizerError {message: format!("{}", err), token: None})
            }
        }
    }
```

This is better but not yet right. For one this is quite verbose: each time we have to convert from `io::Error`
to `TokenizerError` we'd have to cut and past the same code. We can avoid this tedium by factoring the
comon code out into an interpretation of the `From` trait, designed just for this use case:
```rust
impl From<std::io::Error> for TokenizerError {
    fn from(error: std::io::Error) -> Self {
        TokenizerError {message: format!("{}", error), token: None}
    }
}
```
Now we can rewrite the `from_file()` method more consicely:
```rust
    match File::open(filename) {
        Ok(file) => { Ok(self.from_buf_reader(file)) }
        Err(error) => { Err(From::from(error)) }
```

Conveniently, there's syntactic sugar for this exact construct: `?`. `expr?` desugars to
```rust
match expr {
    Ok(value) => val,
    Err(error) => return Err(From::from(error)),
}
```
Thus, we can simplify `from_file()` even further:
```rust
pub fn from_file(&self, filename: &str)
    -> Result<impl Iterator<Item=String>, TokenizerError> 
{
    Ok(self.from_buf_reader(File::open(filename)?)) 
}
```

We won't even need the `Ok(...)` once we convert `from_buf_reader()` to also return `Result`, which
I'll do next.

#### Explicit `Error` Propagation with Closures










A look at the docs for `stc::io::Error` reveals the `source()` method,
 which returns the cause of the I/O error. In this case it's `None`, because the underlying error did not
 originate in the user space, but by the OS. However, to be good citizens we should attach
 this I/O error as the source of our error to help clients of our library debug their errors. 