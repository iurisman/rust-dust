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
        let mut new_node = Box::new(StackNode{next: None, elem: elem});
        match &mut self.head {
            None => self.head = Some(new_node),
            Some(_) => {
                new_node.next = self.head.take();
                self.head = Some(new_node);
            }
        };
        self.size += 1;
    }

    fn pop(&mut self) -> Option<E> {
        if self.size > 0 {
            let old_head = self.head.take().unwrap();
            self.head = old_head.next;
            self.size -= 1;
            Some(old_head.elem)
        }
        else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::Stack;

    #[test]
    fn int_test() {
        let mut stack: Stack<i32> = Stack::new();
        assert!(stack.pop().is_none());
        assert_eq!(0, stack.size);
        stack.push(1);
        assert_eq!(1, stack.size);
        assert_eq!(1, stack.pop().unwrap());
        assert_eq!(0, stack.size);
        assert!(stack.pop().is_none());
    }

    #[test]
    fn struct_test() {
        #[derive(Debug)]
        struct Struct(i32, String);
        impl PartialEq for Struct {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0 && self.1 == other.1
            }

            fn ne(&self, other: &Self) -> bool {
                !self.eq(other)
            }
        }
        let mut stack: Stack<Struct> = Stack::new();
        for i in 1..10 {
            stack.push(Struct(i, i.to_string()));
            assert_eq!(i, stack.size as i32);
        }

        for i in 10..1 {
            assert_eq!(Struct(i, i.to_string()), stack.pop().unwrap());
            assert_eq!(i-1, stack.size as i32);
        }
    }
}