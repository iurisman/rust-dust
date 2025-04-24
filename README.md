# Rust Cookbook
### Learn Rust by building simple things.
By Igor Urisman 2024-25

### 1. Audience
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

### 2. Stack
We start with a stack (a LIFO singly-linked list) because its memory 
organization is acyclic: each heap-allocated object has a single referer.
(In doubly-linked lists each element is pointed to by the previous and the next
elements, which presents an additional challenge.)

#### 2.1. Naïve Stack (The Java Approach)
Source: `src/naive.rs`

In the naïve stack, one can see what happens if we ditch the concepts of stack and heap
and hope that the language will handle memory management details for us. In the listing below,
the struct `Stack` holds the pointer to the head node, if any, and the current stack size.
The struct `StackNode` holds the value of the element of type `E` and the pointer to the next node,
if any.

```rust
struct Stack<E> {
    head: Option<StackNode<E>>,
    size: usize,
}

struct StackNode<E> {
    next: Option<StackNode<E>>,
    elem: E,
}
```
 Something like this would work fine in Java but Rust gives a compilation error:

```text
error[E0072]: recursive type `naive_stack::StackNode` has infinite size
 --> src/naive_stack.rs:6:1
  |
6 | struct StackNode<E> {
  | ^^^^^^^^^^^^^^^^^^^
7 |     next: Option<StackNode<E>>,
  |                  ------------ recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
7 |     next: Option<Box<StackNode<E>>>,
  |                  ++++            +
```

Recursive structures do not have a size known at compile time because the amount of memory needed
to allocate one depends on the depth of recursion. This works in Java because the JVM allocates 
everything on the heap, deferring the size computation until runtime. 
Rust does not have runtime, so the compiler must know how much memory each type requires.

Next, we consider compiler's (helpful) suggestion "_insert some indirection 
(e.g., a `Box`, `Rc`, or `&`) to break the cycle._"

#### 2.2. Naïve Stack (The C Approach)
Source: `src/naive.rs`

C too requires that all structures' sizes be known at compile time. There, the problem is resolved by
(as suggested by the Rust compiler) indirection, replacing the inline inner structure with a pointer 
to it:

```C
template <typename T>
struct Stack {
    StackNode* head;
    int size;
};

template <typename T>
struct StackNode {
    T elem;
    StackNode* next;
};
```

In C, the programmer is responsible for "manually" allocating each instance of `StackNode` and
storing its pointer in the parent node. Pointers have a known size (that of the machine word),
but direct pointer manipulation is exactly the brittleness that Rust intentionally does not
allow. If we were to try with something like this

```rust
struct Stack<E> {
    head: Option<&StackNode<E>>,
    size: usize,
}

struct StackNode<E> {
    next: Option<&StackNode<E>>,
    elem: E,
}
```

we would get the error

```text
error[E0106]: missing lifetime specifier
 --> src/naive.rs:2:18
  |
2 |     head: Option<&StackNode<E>>,
  |                  ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct Stack<'a, E> {
2 ~     head: Option<&'a StackNode<E>>,
  |

error[E0106]: missing lifetime specifier
 --> src/naive.rs:7:18
  |
7 |     next: Option<&StackNode<E>>,
  |                  ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
6 ~ struct StackNode<'a, E> {
7 ~     next: Option<&'a StackNode<E>>,
  |
```

The error demonstrates the fundamental difference between a C pointer and a Rust reference: 
a Rust reference represents a borrowed value owned by some other structure outside our Stack. 
Clearly, this is not what we want: rather, we want the `StackNode` structure and the `elem` 
it contains to be owned by the stack. The allocation details of `StackNode` instances
should be private to the `stack` module, while the element of type `E` should be
allocated by the caller and then transferred by value to our stack in the `fn push(&mut self, elem: E)`
function call.

`Rc` is similarly the wrong idea, because shared ownership is not what we're after. The stack should be
the sole owner of the structures in contains.

#### 2.3. Working Stack (The Rust Approach)