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

    /// Pop off the head of the deque.
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
}

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

mod tests {
    use crate::deque::Deque;
    #[test]
    fn test_front() {
        let mut deque: Deque<i32> = Deque::new();
        assert_eq!(deque.size, 0);
        assert_eq!(deque.pop(), None);
        assert_eq!(deque.size, 0);

        for i in 0..10 {
            deque.push(i);
            assert_eq!(deque.size, (i + 1) as usize);
        }
        for i in (0..10).rev() {
            assert_eq!(deque.pop().unwrap(), i);
            assert_eq!(deque.size, i as usize);
        }
        assert_eq!(deque.size, 0);
        assert_eq!(deque.pop(), None);
        assert_eq!(deque.size, 0);
    }

    #[test]
    fn test_back() {
        let mut deque: Deque<i32> = Deque::new();
        assert_eq!(deque.size, 0);
        assert_eq!(deque.pop_back(), None);
        assert_eq!(deque.size, 0);

        for i in 0..10 {
            deque.push_back(i);
            assert_eq!(deque.size, (i + 1) as usize);
        }
        for i in (0..10).rev() {
            assert_eq!(deque.pop_back().unwrap(), i);
            assert_eq!(deque.size, i as usize);
        }
        assert_eq!(deque.size, 0);
        assert_eq!(deque.pop(), None);
        assert_eq!(deque.size, 0);
    }

    #[test]
    fn test_mixed() {
        let mut deque: Deque<i32> = Deque::new();
        assert_eq!(deque.size, 0);
        assert_eq!(deque.pop_back(), None);
        assert_eq!(deque.size, 0);

        for i in 0..10 {
            if i % 2 == 0 {
                deque.push(i);
            } else {
                deque.push_back(i);
            }
            assert_eq!(deque.size, (i + 1) as usize);
        }
        assert_eq!(deque.pop().unwrap(), 8);
        assert_eq!(deque.size, 9);
        assert_eq!(deque.pop().unwrap(), 6);
        assert_eq!(deque.size, 8);
        assert_eq!(deque.pop().unwrap(), 4);
        assert_eq!(deque.size, 7);
        assert_eq!(deque.pop().unwrap(), 2);
        assert_eq!(deque.size, 6);
        assert_eq!(deque.pop().unwrap(), 0);
        assert_eq!(deque.size, 5);
        assert_eq!(deque.pop().unwrap(), 1);
        assert_eq!(deque.size, 4);
        assert_eq!(deque.pop().unwrap(), 3);
        assert_eq!(deque.size, 3);
        assert_eq!(deque.pop().unwrap(), 5);
        assert_eq!(deque.size, 2);
        assert_eq!(deque.pop().unwrap(), 7);
        assert_eq!(deque.size, 1);
        assert_eq!(deque.pop().unwrap(), 9);
        assert_eq!(deque.size, 0);
        assert_eq!(deque.pop(), None);
        assert_eq!(deque.size, 0);
    }

    #[test]
    fn test_iterator() {
        let mut deque: Deque<String> = Deque::new();
        for i in 0..10 {
            deque.push(i.to_string());
        }
        for (s, ix) in deque.zip((0..10).rev()) {
            assert_eq!(ix, s.parse::<i32>().unwrap());
        }
    }

    #[test]
    fn drop_test() {
        let mut deque: Deque<i32> = Deque::new();
        let size = 1000000;
        for i in 0..size {
            deque.push(i);
        }
        assert_eq!(size, deque.size as i32)
    }
}
