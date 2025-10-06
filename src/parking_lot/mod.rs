#[cfg(test)]
mod tests {
    use parking_lot::{Mutex, deadlock};
    use std::{sync::Arc, thread, time::Duration};

    #[test]
    fn detects_deadlock_without_sleep() {
        let a = Arc::new(Mutex::new(()));
        let b = Arc::new(Mutex::new(()));

        let ready = Arc::new(std::sync::Barrier::new(2));

        let a1 = a.clone();
        let b1 = b.clone();
        let ready1 = ready.clone();
        thread::spawn(move || {
            let _l1 = a1.lock();
            ready1.wait();
            let _l2 = b1.lock();
        });

        let a2 = a.clone();
        let b2 = b.clone();
        let ready2 = ready.clone();
        thread::spawn(move || {
            let _l1 = b2.lock();
            ready2.wait();
            let _l2 = a2.lock();
        });

        let mut deadlock_found = false;
        for _ in 0..100 {
            thread::sleep(Duration::from_millis(10));
            let deadlocks = deadlock::check_deadlock();
            if !deadlocks.is_empty() {
                deadlock_found = true;
                break;
            }
        }

        assert!(deadlock_found);
    }
}
