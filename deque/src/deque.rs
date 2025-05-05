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
        match self.head.take() {
            None => {
                let new_node = Rc::new(RefCell::new(DequeNode { next: None, prev: None, elem }));
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
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
}


mod tests {
    use crate::deque::Deque;
    #[test]
    fn test_front() {
        let mut deque: Deque<i32> = Deque::new();
        assert_eq!(deque.size, 0);
        for i in 0..10 {
            deque.push(i);
            assert_eq!(deque.size, (i + 1) as usize);
        }
        for i in (0..10).rev() {
            assert_eq!(deque.pop().unwrap(), i);
            assert_eq!(deque.size, i as usize);
        }
    }
}
