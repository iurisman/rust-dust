# Rust Dust
### Learn Rust by building simple things.
By Igor Urisman<br>

Also available at https://urisman.net

If, like me, you are coming to Rust from a language that runs on a virtual
machine, like Java or Erlang, your biggest challenge is Rust's memory safety
facility. It requires you to be cognizant of which allocations happen on
the stack and which on the heap — the concepts that are completely abstracted
out for you by VMs. If, on the other hand, you're coming from C/++, your discomfort
comes from the fact that the language does not give you a mechanism
to directly allocate on the heap, but still requires you to understand the implicit
memory organization of your datastructures.

The code I develop in the following sections stops well short of the impolementation of
these data structures found in the standard library. Those use unsafe direct pointer 
manipulations in the name of efficiency and may seem
in contravention of the language's principal ethos of memory safety. For now,
I am leaving both the unsafe Rust and philosophy out of this exercise.

Note that all the modules that fail compilation are intentionally commented out of `main.rs`
to have them ignored by the compiler. If you like to reproduce a compilation error cited
in the following sections, uncomment the corresponding module registration in `main.rs`.

#### 1. [Singly-linked Stack](stack/README.md)
#### 2. [Doubly-linked Deque](deque/README.md)
#### 3. [Rust Dust Library](lib/README.md)
##### 3.1. [How to Build a Simple Rust Library](lib/README.md#1-how-to-build-a-simple-rust-library)
##### 3.2. [Stream Tokenizer Library](lib/README.md#2-stream-tokenizer)
#### 4. [Trie (Prefix Tree)](trie/README.md)
#### 5. [Trie Redux with Error Handling](lib/README_WITH_RESULT.md)


