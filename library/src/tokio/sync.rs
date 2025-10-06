#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use tokio::sync::oneshot;

    async fn some_computation() -> &'static str {
        let _ = tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        "represents the result of the computation"
    }

    // oneshot is a channel that sends a single value from a single producer to a single consumer.
    // This is for sending the result of a computation to a consumer.
    #[tokio::test(start_paused = true)]
    async fn oneshot() {
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            let res = some_computation().await;
            tx.send(res).unwrap();
        });

        let res = rx.await.unwrap();
        assert_eq!(res, "represents the result of the computation");
    }

    use tokio::sync::mpsc;

    async fn some_computation_index(input: u32) -> String {
        format!("the result of computation {}", input)
    }

    // we can also use mpsc channels which allow you to send multiple values to a single consumer
    // this is useful for allowing many workers to do work
    #[tokio::test(start_paused = true)]
    async fn test_mpsc() {
        let (tx, mut rx) = mpsc::channel(100);

        tokio::spawn(async move {
            for i in 0..10 {
                let res = some_computation_index(i).await;
                tx.send(res).await.unwrap();
            }
        });

        let mut v = vec![];

        while let Some(res) = rx.recv().await {
            v.push(res);
        }

        assert_eq!(
            v,
            vec![
                "the result of computation 0",
                "the result of computation 1",
                "the result of computation 2",
                "the result of computation 3",
                "the result of computation 4",
                "the result of computation 5",
                "the result of computation 6",
                "the result of computation 7",
                "the result of computation 8",
                "the result of computation 9",
            ]
        );
    }

    use tokio::sync::broadcast;

    // broadcast allows you to send many values from many producers to many consumers. Each
    // consumer receives each value. As well, if you want to broadcast values from a single
    // producer to many consumers, broadcast works for that use case.
    #[tokio::test(start_paused = true)]
    async fn test_broadcast() {
        let (tx, mut rx1) = broadcast::channel(16);
        let mut rx2 = tx.subscribe();

        tokio::spawn(async move {
            assert_eq!(rx1.recv().await.unwrap(), 10);
            assert_eq!(rx1.recv().await.unwrap(), 20);
        });

        tokio::spawn(async move {
            assert_eq!(rx2.recv().await.unwrap(), 10);
            assert_eq!(rx2.recv().await.unwrap(), 20);
        });

        tx.send(10).unwrap();
        tx.send(20).unwrap();
    }

    use tokio::sync::watch;

    // watch is a channel that supports sending many values from many producers to many consumers.
    // However, it only keeps the most recent value, and consumers are notified when a new value is
    // sent.
    #[tokio::test(start_paused = true)]
    async fn test_watch_late_subscriber() {
        let (tx, mut rx1) = watch::channel(0);

        // the first subscriber is notified of the first update and second update (10, 20)
        // and can verify them both.
        tokio::spawn(async move {
            rx1.changed().await.unwrap();
            assert_eq!(*rx1.borrow(), 10);

            rx1.changed().await.unwrap();
            assert_eq!(*rx1.borrow(), 20);

            rx1.changed().await.unwrap();
            assert_eq!(*rx1.borrow(), 30);
        });

        tx.send(10).unwrap();
        tx.send(20).unwrap();

        // after initializing another subscriber
        let mut rx2 = tx.subscribe();

        // it can only see the last update, 20.
        assert_eq!(*rx2.borrow(), 20);

        tx.send(30).unwrap();

        // but after the update to 30
        rx2.changed().await.unwrap();

        // it can also see that
        assert_eq!(*rx2.borrow(), 30);
    }

    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    use tokio::sync::Barrier;

    #[tokio::test(start_paused = true)]
    async fn test_barrier() {
        let barrier = Arc::new(Barrier::new(3));
        let counter = Arc::new(AtomicUsize::new(0));

        let mut handles = Vec::new();
        for _ in 0..3 {
            let b = barrier.clone();
            let c = counter.clone();
            handles.push(tokio::spawn(async move {
                // Increment before barrier
                c.fetch_add(1, Ordering::SeqCst);
                b.wait().await;
                // At this point, all 3 tasks should have incremented
                assert_eq!(c.load(Ordering::SeqCst), 3);
            }));
        }

        for h in handles {
            h.await.unwrap();
        }
    }

    use tokio::sync::Mutex;

    // Mutexes only allow one thread access to a given resource.
    #[tokio::test]
    async fn test_mutex() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = Vec::new();

        for _ in 0..5 {
            let c = counter.clone();
            handles.push(tokio::spawn(async move {
                let mut lock = c.lock().await;
                *lock += 1;
            }));
        }

        for h in handles {
            h.await.unwrap();
        }

        assert_eq!(*counter.lock().await, 5);
    }

    use std::sync::atomic::AtomicBool;
    use tokio::sync::Notify;

    // notify is used to notify one thread that it can work, without having to pass data.
    #[tokio::test(start_paused = true)]
    async fn test_notify() {
        let notify = Arc::new(Notify::new());
        let flag = Arc::new(AtomicBool::new(false));

        let f = flag.clone();
        let n = notify.clone();
        let handle = tokio::spawn(async move {
            n.notified().await; // Wait for signal
            assert!(f.load(Ordering::SeqCst));
        });

        // Set the flag before notifying
        flag.store(true, Ordering::SeqCst);
        notify.notify_one();

        handle.await.unwrap();
    }

    use tokio::sync::RwLock;

    // RwLock allow as many readers but only one writer to access data.
    #[tokio::test]
    async fn test_rwlock() {
        let data = Arc::new(RwLock::new(vec![1, 2, 3]));

        // Multiple readers
        let d1 = data.clone();
        let r1 = tokio::spawn(async move {
            let read = d1.read().await;
            assert_eq!(read.len(), 3);
        });

        let d2 = data.clone();
        let r2 = tokio::spawn(async move {
            let read = d2.read().await;
            assert_eq!(read[0], 1);
        });

        r1.await.unwrap();
        r2.await.unwrap();

        // One writer (blocks readers until done)
        {
            let mut write = data.write().await;
            write.push(4);
        }

        assert_eq!(data.read().await.len(), 4);
    }

    use tokio::sync::Semaphore;
    use tokio::time::{Duration, sleep};

    // Semaphore allows only N threads into a guarded section of code.
    #[tokio::test(start_paused = true)]
    async fn test_semaphore() {
        let semaphore = Arc::new(Semaphore::new(2)); // only 2 permits
        let active = Arc::new(AtomicUsize::new(0));

        let mut handles = Vec::new();
        for _ in 0..5 {
            let sem = semaphore.clone();
            let act = active.clone();
            handles.push(tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();

                let inside = act.fetch_add(1, Ordering::SeqCst) + 1;
                assert!(inside <= 2); // never exceed 2 tasks at once

                sleep(Duration::from_millis(10)).await;

                act.fetch_sub(1, Ordering::SeqCst);
            }));
        }

        for h in handles {
            h.await.unwrap();
        }

        // After all tasks complete, no one should be inside
        assert_eq!(active.load(Ordering::SeqCst), 0);
    }
}
