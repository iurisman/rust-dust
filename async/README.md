## Input/Output

We would greatly expand the usability of the singly-linked stack if we could turn it into a
doubly-linked list. This will enable us to add methods `push_back()` and `pop_back()`, as well as
implement `DoubleEndedIterator`, supporting traversal in reverse (from back to head).

### 1 `Box` Will Not Do
Source: deque/src/box.rs
