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
    fn push(&mut self, elem: E) {
        if self.size == 0 {
            let new_node = Rc::new(RefCell::new(DequeNode{next: None, prev: None, elem}));
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        } else {
            let old_head = self.head.take();
            let new_head = Rc::new(RefCell::new(DequeNode{next: old_head.clone(), prev: None, elem}));
            self.head = Some(new_head.clone());
            old_head.unwrap().borrow_mut().prev = Some(new_head);
        }
        self.size += 1;
    }
}

mod tests {
    use crate::deque::Deque;
    #[test]
    fn test() {
        let mut deque: Deque<i32> = Deque::new();
        assert_eq!(deque.size, 0);
        deque.push(1);
        assert_eq!(deque.size, 1);
    }
}
