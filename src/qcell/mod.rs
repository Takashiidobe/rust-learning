#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use qcell::{QCell, QCellOwner};

    // this test panics at runtime
    #[test]
    #[should_panic]
    fn refcell() {
        let item = Rc::new(RefCell::new(Vec::<u8>::new()));
        let mut iref = item.borrow_mut();
        item.borrow_mut().push(2); // panics at runtime here
        iref.push(1);
    }

    // Qcell is used to bring RefCell related runtime errors due to dynamic borrow checking to
    // compile time. Note that this test doesn't compile
    #[test]
    fn qcell_test() {
        let mut owner = QCellOwner::new();

        let item = Rc::new(QCell::new(&owner, Vec::<u8>::new()));
        let iref = owner.rw(&item);
        // owner.rw(&item).push(2); // Compile error
        iref.push(1);
        assert_eq!(*owner.ro(&item), vec![1]);
    }
}
