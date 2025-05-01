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

}

mod tests {
    use crate::r#box::Deque;
    #[test]
    fn test() {
        let mut deque: Deque<i32> = Deque::new();
        deque.push(1);
    }
}
