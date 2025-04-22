struct Stack<E> {
    head: Option<StackNode<E>>,
    size: usize,
}

struct StackNode<E> {
    next: Option<StackNode<E>>,
    elem: E,
}

impl<E> Stack<E> {
    fn new() -> Self {
        Stack { head: None, size: 0 }
    }
    fn push(&mut self, elem: E) {
        todo!()
    }

    fn pop(&mut self) -> Option<E> {
        todo!()
    }
}
#[cfg(test)]
mod tests {
    use crate::naïve_stack::Stack;
    #[test]
    fn test() {
        let mut stack: Stack<i32> = Stack::new();
    }
}