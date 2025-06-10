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

