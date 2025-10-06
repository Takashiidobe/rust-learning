#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::sync::Arc;
    use std::thread;
    use thread_local::ThreadLocal;

    #[test]
    fn tls() {
        let tls = Arc::new(ThreadLocal::new());

        for _ in 0..5 {
            let tls2 = tls.clone();
            thread::scope(|s| {
                s.spawn(|| {
                    let cell = tls2.get_or(|| Cell::new(0));
                    cell.set(cell.get() + 1);
                });
                s.spawn(|| {
                    let cell = tls2.get_or(|| Cell::new(0));
                    cell.set(cell.get() + 1);
                });
            });
        }

        let tls = Arc::try_unwrap(tls).unwrap();
        let total = tls.into_iter().fold(0, |x, y| x + y.get());
        assert_eq!(total, 10);
    }
}
