struct Stack<E> {
    head: Option<Box<StackNode<E>>>,
    size: usize,
}

struct StackNode<E> {
    elem: E,
    next: Option<Box<StackNode<E>>>,
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
            Some(head) => {
                self.head = head.next;
                self.size -= 1;
                Some(head.elem)
            }
        }
    }

    // fn peek(&mut self) -> Option<&E> {
    //     self.head.as_ref().map(|node| &node.elem)
    // }
}


impl <E> Drop for Stack<E> {

    // Works but inefficient
    // fn drop(&mut self) {
    //     while let Some(node) = self.pop() {}
    // }

    // This is better
    fn drop(&mut self) {
        let mut curr_head = self.head.take();
        while let Some(boxed_node) = curr_head {
            curr_head = boxed_node.next;
        }
    }
}

impl <E> Iterator for Stack<E> {
    type Item = E;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
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

    #[test]
    fn drop_test() {
        let mut stack: Stack<i32> = Stack::new();
        for i in 0..100000 {
            stack.push(i);
        }
    }

    #[test]
    fn iter_test() {
        let mut stack: Stack<String> = Stack::new();
        for i in 0..100 {
            stack.push(i.to_string());
        }

        for i in stack {
            println!("{}", i);
        }
    }

    // #[test]
    // fn peek_test() {
    //     let mut stack: Stack<String> = Stack::new();
    //     assert!(stack.pop().is_none());
    //     assert_eq!(0, stack.size);
    //     assert!(stack.peek().is_none());
    //     stack.push(String::from("Hello"));
    //     assert_eq!(1, stack.size);
    //     assert_eq!("Hello", stack.peek().unwrap());
    //     assert_eq!("Hello", stack.peek().unwrap());
    //     stack.push(String::from("World"));
    //     assert_eq!(2, stack.size);
    //     let peek = stack.peek().unwrap();
    //     assert_eq!(peek, &stack.pop().unwrap());
    // }

}