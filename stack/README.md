
## Singly-linked Stack

We start with a stack (a LIFO singly-linked list) because its memory 
organization is acyclic: each heap-allocated object has a single referer.
(In doubly-linked lists each element is pointed to by the previous and the next
elements, which presents an additional challenge.)

### 1. Naïve Stack (The Java Approach)
Source: `naive.rs`

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

### 2. Naïve Stack (The C Approach)
Source: `stack/src/naive.rs`

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
allow. If we to try with something like this

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

we get the error

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
a Rust reference represents a borrowed value owned by some other value outside our Stack. 
Clearly, this is not what we want: rather, we want the `StackNode` structure and the `elem` 
it contains to be owned by the stack. The allocation details of `StackNode` instances
should be private to the `stack` module, while the element of type `E` should be
allocated by the caller and then moved by value to our stack in the `fn push(&mut self, elem: E)`
function call.

`Rc` is similarly the wrong idea, because shared ownership is not what we're after. The stack should be
the sole owner of the structures in contains.

### 3. Working Stack (The Rust Approach)
Source: `stack.rs`

The way to fix this all is to use `Box`, the simplest way of owned heap allocation. `Box` consists
of two parts: the fixed sized stack object that contains the pointer to the user data alocated
on the heap. Whereas `C`'s `malloc` gives you a raw pointer into heap memory, `Box` hides that
pointer inside the stack-allocated header. Note, that `Box` does not implement `Copy` in order
to prevent multiple ownership of the heap data. The following is a basic implementation 
of stack in Rust.

```rust
struct Stack<E> {
    head: Option<Box<StackNode<E>>>,
    size: usize,
}

struct StackNode<E> {
    next: Option<Box<StackNode<E>>>,
    elem: E,
}

impl<E> Stack<E> {
    fn new() -> Self {
        Stack{head: None, size: 0}
    }
    fn push(&mut self, elem: E) {
        let new_node = Box::new(StackNode{elem, next: self.head.take()});
        self.head = Some(new_node);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<E> {
        match self.head.take() {
            None => None,
            Some(old_head) => {
                self.head = old_head.next;
                self.size -= 1;
                Some(old_head.elem)
            }
        }
    }
}
```

### 4. Iterating 
Source: `stack.rs`

Let's now turn our stack into an iterable collection, so that it can be iterated over as in the
next test case we're going to add:
```rust
#[test]
    fn iter_test() {
        let mut stack: Stack<String> = Stack::new();
        for i in 0..100 {
            stack.push(i.to_string());
        }
        for i in stack {    // Error. Stack is not an interator and does not implement `IntoIterator`.
            println!("{}", i);
        }
    }
```

For now, this fails to compile because `rustc` does not know how to iterate over a collection. The
error message makes the actionable suggestion:
```text
help: the trait `Iterator` is not implemented for `Stack<String>`
note: required for `Stack<String>` to implement `IntoIterator`
```

The `for` expression loops over elements of an iterator. The iterator can be provided explicitly,
when the `for` expression already implements `Itarator` (and thus has the `next()` method) or implicitly,
when the `for` expression implements `std::iter::IntoIterator`. As the error suggests, we only need
to implement `Iterator` for our `Stack` so that the blanket implementation of `IntoIterator` provided
by the standard library for any `Iterator` becomes available to the compiler.

Implementing `Iterator` for our `Stack` is trivial —`next() simply calls `pop()`.
```rust
impl <E> Iterator for Stack<E> {
    type Item = E;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
```
Clearly, this iterator moves the values out of the collection, leaving it empty once the iterator
is exhausted. This is not always what we want. Standard collections implement two more methods which
return iterators over them, `iter()` and `iter_mut()` which borrow, without consuming, the elements of
the collection in an immutable or mutable fashion respectively. The implementations of these methods
is an advanced topic that I am leaving out, but if you're interested, here's 
[an excellent write-up](https://rust-unofficial.github.io/too-many-lists/second-iter.html).

### 5. Cleaning Up
Source: `stack.rs`

Let's now add a new test case
```rust

#[test]
    fn drop_test() {
        let mut stack: Stack<i32> = Stack::new();
        for i in 0..100000 {
            stack.push(i);
        }
    }
```

On my system this crashes with
```text
thread 'stack::tests::drop_test' has overflowed its stack
fatal runtime error: stack overflow
```
(If you're not seeing the error, increase the size of the stack.) The error happens when `stack` goes
out of scope at the end of `drop_test()` function. The default `Drop` implementation for our
`Stack`, as generated by the Rust compiler, is recursive: `i`-th node's `drop()` must call `i+1`st node's
`drop()` before cleaning up itself. We must provide a custom `Drop` implementation that replaces
this recursion with a loop. (For a deeper look at why in this case `rustc` cannot generate an
iterative default implementation, see
[this discussion](https://rust-unofficial.github.io/too-many-lists/first-drop.html)).


The first thing that comes to mind is to simply call `pop()` in a loop:
```rust
 impl <E> Drop for Stack<E> {
    fn drop(&mut self) {
        while let Some(node) = self.pop() {}
    }
}
```

This works fine in the sense that `drop_test()` now passes, but this implementation can still be improved.
The `pop()`function repeatedly disposes of the head node, but it also moves the (potentially large) element
value out of `Box` and returns it, re-wrapped in `Option`. This extra memory copy is unavoidable
if the caller wants the element value, but our `drop()` function does not.

The better solution is to keep taking the `Box` from `StackNode.next` and letting it drop on its
own without unpacking:

```rust
    fn drop(&mut self) {
        let mut curr_head = self.head.take();
        while let Some(boxed_node) = curr_head {
            curr_head = boxed_node.next;
        }
    }
```

### 6. Deficiencies

Our implementation of the stack data structure is for educational purposes only. You should not use 
it in production for the following reasons:
* Not thread safe. This can be addressed in two ways. 1) use `Arc` instead of `Rc`, which uses atomc
  integer to increment and decrement reference counts. Should be a straightforward replacement, but
  has a slight performance cost. 2) `RefCell` cannot be shared between threads. It must be replaced
  with a `Mutex`.
* Lacks many useful methods available on `std::collections::Vec`.
