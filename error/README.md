
## Error Handling in Rust

### Evolution of Error Handling before Rust
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
systems programming, cannot afford a VM, and some use cases cannot even support it, like
embedded systems. Until recently, these use cases have relied squarely on C or C++. The
big change came in 2003 with the release of LLVM (Low Level Virtual Machine), a byte-code
and compiler infrastructure that permitted rapid development of new low-level languages
running directly on the processor, without a runtime VM. By 2010 two such languages were 
released: Go (2007, Google) and Rust (2009, Mozilla) — both ditching exceptions in favor
of error propagation via complex return types.

To reiterate, exceptions, at least in the modern sense of the word, have these drawbacks:
* Runtime overhead. Note, for example that Scala (2004, EPFL), an advanced dual paradigm
  language that compiles to the Java bytecode, uses Java's `try/catch/finally` blocks and
  the `throw` statement, just like Java, because these concepts are built into the JVM.
  (Most Scala programmers prefer to use the `Try` type, which hides these imperative verbs
  behind a more functional flow.)
* Potentially a source of resource leaks due to early termination.
* Undermines compiler's ability to reason about the source code.

### Living without Exceptions — Again

When it comes to exceptions, Rust takes the high road: if it can't do them well, it
won't do them at all. (So does Go, but — curiously — not Swift (2014, Apple), another
LLVM based language.) Instead, Rust offers two error handling mechanisms: panic and `Result`.

#### Panic

Panics are meant to be used for systemic unrecoverable errors. It can be triggered
explicitly with the `panic!` macro, or is triggered implicitly by
* Calling `unwrap()` on `Result` if it is `Err`. `Result` is the subject of the next
  section.
* Calling `unwrap()` on `Option` if it is `None`.
* Calling `expect()` on either `Result` or `Option`, which is just a variation of `unwrap()'
  that allows the caller to attach last words to the panic.
* Various arithmetic exceptions, such as division by zero and integer overflow.
* Out-of-bounds array index.

It is possible to recover from panic with `std::panic::catch_unwind` and even to
trigger a custom panic with `std::panic::panic_any()` which takes an arbitrary type that
can be accessed later at the point of recovery. This mechanism however is not meant for
mimicking exception handling a la Java. 

Note that panic is thread-local; panic in a non-main thread will terminate the thread,
but not the process. There are however errors that are more disruptive than panics,
most importantly the out-of-memory condition. At this time of this writing it 
does not cause panic but rather terminates the process regardless what thread received it.