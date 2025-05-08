## Doubly-linked Dequeue

We would greatly expand the usability of the singly-linked stack if we could turn it into a
doubly-linked list. This will enable us to add methods `push_back()` and `pop_back()`, as well as
implement `DoubleEndedIterator`, supporting traversal in reverse (from back to head).

### 1 `Box` Will Not Do
Source: deque/src/box.rs

We can improve on the singly-linked stack in the previous section by adding the necessary pointers:

```rust
struct Deque<E> {
    head: Option<Box<DequeNode<E>>>,
    tail: Option<Box<DequeNode<E>>>,
    size: usize,
}

struct DequeNode<E> {
    next: Option<Box<DequeNode<E>>>,
    prev: Option<Box<DequeNode<E>>>,
    elem: E,
}

impl<E> Deque<E> {
    fn new() -> Self {
        Deque { head: None, tail: None, size: 0 }
    }
}

mod tests {
    use crate::r#box::Deque;
    #[test]
    fn test() {
        let mut stack: Deque<i32> = Deque::new();
    }
}
```
This scaffolding of a Deque seems fine, so let's try to implement `push()`. Both `head` and `tail` 
should point to the first element:

```rust
fn push(&mut self, elem: E) {
    if self.size == 0 {
        let new_node = Box::new(DequeNode{next: self.head.take(), prev: None, elem});
        self.head = Some(new_node);
        self.tail = Some(new_node);  // Error: new_node has been moved.
        self.size += 1;
    } else {
        todo!()
    }
}
```
This does not compile because we're attempting to use `new_node` twice and `Box` is not a copy type.
This is intentional, because the same heap allocation can only
have one owner. If we are to have multiple borrowers of the same heap allocation,
something else, has to own it, deferring the ownership concern to runtime — 
a micro garbage collector that the compiler injects into the executable.

### 2. Runtime Garbage Collection with `Rc`
Source: deque/src/rc.rs

`Rc` (_reference counting_) and `Arc` (_atomic reference counting_) are such micro-garbage collectors.
they take ownership of a piece of heap and give out shared references to it freely, while
counting (atomically, in the case of `Arc`, for thread safety) the number of referrers, including
the original `Rc`. 
When none are left, `Rc` drops its owned heap allocation.  Like `Box`, `Rc` is not a copy type.
However, it implements `Clone`, such that calling the `clone()` method produces another 
instance of `Rc` pointing to the same heap allocation.

Our dequeue scaffolding now runs:
```rust
use std::rc::Rc;
struct Deque<E> {
    head: Option<Rc<DequeNode<E>>>,
    tail: Option<Rc<DequeNode<E>>>,
    size: usize,
}

struct DequeNode<E> {
    next: Option<Rc<DequeNode<E>>>,
    prev: Option<Rc<DequeNode<E>>>,
    elem: E,
}

impl<E> Deque<E> {
    fn new() -> Self {
        Deque { head: None, tail: None, size: 0 }
    }
    fn push(&mut self, elem: E) {
        if self.size == 0 {
            let new_node = Rc::new(DequeNode{next: None, prev: None, elem});
            self.head = Some(new_node.clone()); // Clone before new_node gets moved on the next line.
            self.tail = Some(new_node);
        } else {
            todo!()
        }
        self.size += 1;
    }
}

mod tests {
    use crate::rc::Deque;
    #[test]
    fn test() {
        let mut deque: Deque<i32> = Deque::new();
        deque.push(1);
    }
}
```

Note that we must clone the `Rc` value before moving it.  

Now, let's implement the case when there's already one or more nodes in the list. 
This will require mutating nodes already inside 'Rc's:
```rust
    fn push(&mut self, elem: E) {
        if self.size == 0 {
            let new_node = Rc::new(DequeNode{next: None, prev: None, elem});
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        } else {
            let old_head = self.head.take();
            let new_head = Rc::new(DequeNode{next: old_head.clone(), prev: None, elem});
            self.head = Some(new_head.clone());
            old_head.unwrap().prev = Some(new_head);  // Error: `Rc` is immutable.
        }
        self.size += 1;
    }
```

This does not compile:
```text
error[E0594]: cannot assign to data in an `Rc`
  --> src/rc.rs:26:13
   |
26 |             old_head.unwrap().prev = Some(new_node.clone());
   |             ^^^^^^^^^^^^^^^^^^^^^^ cannot assign
   |
   = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Rc<rc::DequeNode<E>>`
```
The reason for the error is that the `Rc` type follows Rust's general principle that shared references
are immutable. Conveniently, the docs for `Rc` anticipate this hurdle and offer this suggestion:

> Shared references in Rust disallow mutation by default, and Rc is no exception: 
> you cannot generally obtain a mutable reference to something inside an Rc. 
> If you need mutability, put a Cell or RefCell inside the Rc.

### 3 Inner Mutability with `RefCell`
Source: deque/src/deque.rs

`Cell` is typically used for copy types where the cell's contents aren't actually mutated, but
replaced. The non-copy types or larger copy types are advised to prefer `RefCopy`, which enables
direct access to the cell's content with the following methods:
```rust
borrow(&self) -> &T 
borrow_ref(&self) -> &mut T
```
Both methods perform a runtime check, ensuring that at most one mutable borrower exists at a time, 
and that at no time an immutable borrower co-exists with a mutable borrower. If these invariants
are violated, the calling thread will panic. 

Inserting `RefCell` between `Rc` and `DequeNode` yields working implementation of the `push()` method.
We can now update the old head's `prev` link by borrowing a mutable reference to it from `RefCell`.
The borrowed reference lives until its value exits scope, i.e. just that the expression 
`old_head.borrow_mut().prev = Some(new_head);`.

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct Deque<E> {
    head: Option<Rc<RefCell<DequeNode<E>>>>,
    tail: Option<Rc<RefCell<DequeNode<E>>>>,
    size: usize,
}

struct DequeNode<E> {
    next: Option<Rc<RefCell<DequeNode<E>>>>,
    prev: Option<Rc<RefCell<DequeNode<E>>>>,
    elem: E,
}

impl<E> Deque<E> {
    fn new() -> Self {
        Deque { head: None, tail: None, size: 0 }
    }

    /// Insert at the head of the deque.
    fn push(&mut self, elem: E) {
        match self.head.take() {
            None => {
                let new_head = Rc::new(RefCell::new(DequeNode { next: None, prev: None, elem }));
                self.head = Some(new_head.clone());
                self.tail = Some(new_head);
            }
            Some(old_head) => {
                let new_head =
                    Rc::new(RefCell::new(DequeNode { next: Some(old_head.clone()), prev: None, elem }));
                self.head = Some(new_head.clone());
                old_head.borrow_mut().prev = Some(new_head);
            }
        }
        self.size += 1;
    }
}
```

Likewise, the implementation of the `pop()` method borrows mutable references to the head of the que
and its `prev` links in order to mutate via the interior mutability mechanism provided by `RefCell`. 

The implementation of the `pop()` method is also straightforward, except perhaps for the last line
where we move the value of type `E` to return from the method. In order to consume the `elem`,
we need to consume the entire node `Rc<RefCell<DequeNode<E>>>` from left to right. The 
`Rc::try_unwrap()` associated function returns the object inside the `Rc` instances re-wrapped in 
`Result::Ok` if the passed reference is the only strong reference to the shared object, or the
unchanged `Rc` instance if the strong reference count is > 1.  

```rust
impl<E> Deque<E> {

    ...
    
    /// Pop off the head of the deque.
    fn pop(&mut self) -> Option<E> {
        self.head.take().map(|old_head| {
            self.head = old_head.borrow_mut().next.take();
            match &self.head {
                Some(new_head) => {
                    // New head's prev link is now null;
                    new_head.borrow_mut().prev.take();
                }
                None => {
                    // No more nodes left.
                    self.tail = None
                }
            }
            self.size -= 1;
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
}
```

The `pop()` method pops the head element of the deque.

```rust
fn pop(&mut self) -> Option<E> {
    self.head.take().map (|old_head| {
        self.head = old_head.borrow_mut().next.take();
        match &self.head {
            Some(new_head) => {
                // New head's prev link is now null;
                new_head.borrow_mut().prev.take();
            }
            None => {
                // No more nodes left.
                self.tail = None
            }
        }
        self.size -= 1;
        Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
    })
}
```

And, finally the `push_back()` and `pop_back()` methods below are the counterparts to the above `push()`
and `pop()` methods, that operate on the back end of the queue. 

```rust
    ...

    /// Insert at the back of the queue
    fn push_back(&mut self, elem: E) {
        match self.tail.take() {
            None => {
                let new_node = Rc::new(RefCell::new(DequeNode { next: None, prev: None, elem }));
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            Some(old_tail) => {
                let new_tail =
                    Rc::new(RefCell::new(DequeNode { next: None, prev: Some(old_tail.clone()), elem }));
                self.tail = Some(new_tail.clone());
                old_tail.borrow_mut().next = Some(new_tail);
            }
        }
        self.size += 1;
    }

    /// Pop off the back of the queue.
    fn pop_back(&mut self) -> Option<E> {
        self.tail.take().map (|old_tail| {
            self.tail = old_tail.borrow_mut().prev.take();
            match &self.tail {
                Some(new_tail) => {
                    // New tail's next link is now null;
                    new_tail.borrow_mut().next.take();
                }
                None => {
                    // No more nodes left.
                    self.head = None
                }
            }
            self.size -= 1;
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }
    ...
```

### 4. Iterating
Because our deque is double ended, we want to implement the `DoubleEndedIterator` trait, in addition
to the `Iterator` trait to be able to iterate in both directions. The implementations simply wrap
calls to `pop()` and `pop_back()` respectively.

```rust
impl<E> Iterator for Deque<E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

impl <E> DoubleEndedIterator for Deque<E> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.pop_back()
    }
}
```

If you're coming from an object oriented language, you may wonder why we implemented the two traits
separately, geven `DoubleEndedIterator`'s definition:
```rust
pub trait DoubleEndedIterator: Iterator {
    // Required method
    fn next_back(&mut self) -> Option<Self::Item>;

    // Provided methods
    ...
}
```

In, e.g. Scala, when a trait extends another trait, a class implementing the descendent trait
implements methods from both traits. In Rust, however, `trait DoubleEndedIterator: Iterator` 
means that implementing structures must also implement, i.e. `Iterator` is not a supertype, but
an (upper) type bound, with the implication that `Self` is already an `Iterator` and that 
`DoubleEndedIterator`'s element type is `Self::Item`. Rust does not support any traditional notion
of subtyping.

### 5. Cleaning Up

Let's now see if we need to add a custom `Drop` implementation. Following what we did in for stack
in section 2.5, we add the test case that drop a large deque:
```rust
    fn drop_test() {
        let mut deque: Deque<i32> = Deque::new();
        for i in 0..100000 {
            deque.push(i);
        }
    }
```
The test passes without an error. The reason for that is that the default `Drop` implementation will
not attempt to drop any of the nodes because each of them is referenced by two other nodes. More
precisely, the default `Drop` will drop the `Deque` structure, but leak the nodes because each of them
has two outstanding strong references. (In the trivial case of one element, the compiler will be
able to drop it becase both references come from the same structure.)

We could verify this by downgrading one of the links, e.g, `prev` to a weak reference with 
`Rc::downgrade(&strong)`, where `strong` is the reference we currently obtain from `Rc::new()`. This
will make the entire graph acyclic for the purposes of `Drop` and `rustc` will be able to generate
the code to drop all the nodes recursively, as it was the case for stack. This is not what we want 
though, because this default  `Drop` implementation will blow the stack.

Instead, we write a custom `Drop` implementation which loops through the entire list, emptying out
all the pointers. This decrements the ref count on all the links to 0, allowing them to be dropped
by the compiler-generated "garbage collector":
```rust
impl<E> Drop for Deque<E> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(curr_head) = head {
            curr_head.borrow_mut().prev.take();
            head = curr_head.borrow_mut().next.take();
        }
    }
}
```

### 6. Deficiencies

Our implementation of the deque data structure is for educational purposes only. You should not use
it in production for the following reasons:
* Not thread safe. This can be addressed in two ways. 1) use `Arc` instead of `Rc`, which uses atomc
  integer to increment and decrement reference counts. Should be a straightforward replacement, but
  has a slight performance cost. 2) `RefCell` cannot be shared between threads. It must be replaced
  with a `Mutex`.
* Lacks many useful methods available on `std::collections::VecDeque`.