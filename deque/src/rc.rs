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
            let new_node = Rc::new(DequeNode{next: self.head.take(), prev: None, elem});
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
            self.size += 1;
        } else {
            todo!()
        }
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
