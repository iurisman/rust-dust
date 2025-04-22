# Rust Cookbook
### Learn Rust by building simple things.
By Igor Urisman 2024-25

### 1. Audience
If, like me, you are coming to Rust from a language that runs on a virtual
machine, like Java or Erlang, your biggest challenge is Rust's memory safety
mechanism. It requires you to be cognizant of which allocations happen on
the stack and which on the heap — the concepts that are completely abstracted
out by VMs. If, on the other hand, you're coming from C/++, your discomfort
comes from the fact that the language does not give you a mechanism
to directly allocate on the heap, but still requires you to understand the 
memory organization of your datastructures.

### 2. Stack
We start with a stack (a LIFO singly-linked list) because its memory 
organization is acyclic: each heap-allocated object has a single owner.

#### 2.1. Naïve Stack


