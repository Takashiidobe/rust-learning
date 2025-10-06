#[cfg(test)]
mod tests {
    use delegate::delegate;

    #[derive(Clone, Debug)]
    struct Stack<T> {
        inner: Vec<T>,
    }

    impl<T> Stack<T> {
        pub fn new() -> Stack<T> {
            Self { inner: vec![] }
        }

        // delegate allows us to delegate some methods from any member inside a newtype struct.
        // it's possible to rename or delegate to another function or use a specific field to
        // delegate to. However you cannot automatically delegate all public functions which would
        // be nice.
        delegate! {
            to self.inner {
                pub fn is_empty(&self) -> bool;
                pub fn push(&mut self, value: T);
                pub fn pop(&mut self) -> Option<T>;
                pub fn clear(&mut self);

                #[call(len)]
                pub fn size(&self) -> usize;

                #[call(last)]
                pub fn peek(&self) -> Option<&T>;
            }
        }
    }

    // test that all methods are properly delegated
    #[test]
    fn test_stack() {
        let mut s = Stack::new();
        s.push(10);

        assert_eq!(s.inner, vec![10]);
        assert_eq!(s.inner.is_empty(), s.is_empty());
        assert_eq!(s.inner.len(), s.size());
        assert_eq!(s.inner.last(), s.peek());
        s.clear();
        assert_eq!(s.inner.pop(), s.pop());
    }
}
