#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    // a tokio runtime keepsa set of tasks that need to run. It will remove a task and schedule it
    // with `poll`. If the collection is empty, the thread will sleep until it gets more tasks.
    // However, this leads to task starvation -- tokio has a fairness guarantee, where if the
    // number of tasks is bounded and no task is blocking the thread, then it will be scheduled
    // fairly. Tokio uses `MAX_TASKS` which configures the total number of tasks on the runtime,
    // and `MAX_SCHEDULE` where each task returns within that amount of time, and `MAX_DELAY`,
    // where when a task is woken up, it will be scheduled by that time. Thus, fairness means that
    // tasks will run within their `MAX_DELAY` time.
    //
    // The runtime also schedules IO resources and timers. It checks if any IO resources or timers
    // are ready and then waking tasks that use those to be scheduled.
    //
    // For the current thread runtime: There are two FIFO queues of tasks to be scheduled, a global
    // and local queue. The local queue is preferred unless it is empty or it has picked a task
    // `global_queue_interval` times, which defaults to 31. The runtime will check if there's a new
    // IO or timer event when there are no tasks to be scheduled or when it has schedule
    // `event_interval` tasks in a row, which defaults to 61.
    //
    // For the multithreaded runtime, there is a global queue and a local queue for each worker
    // thread. The local queue can fit at most 256 tasks. If more are added, they are added to the
    // global queue. If a worker thread has no tasks and the global queue is empty, then the worker
    // thread will steal half its tasks from the local queue of another worker thread.

    // tokio::test allows you to configure the runtime that's spawned for the test.
    // in this case, we spawn 4 worker threaders, and then run a task that returns its thread index
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn test_multi_thread_runtime_configuration() {
        let threads = 4;

        let mut handles = vec![];
        for i in 0..threads {
            handles.push(tokio::spawn(async move { i }));
        }

        let mut values = HashSet::new();
        for handle in handles {
            values.insert(handle.await.unwrap());
        }

        // these items can come in any order, so we use a set
        assert_eq!(values, HashSet::from([0, 1, 2, 3]));
    }

    // you can use one thread for a runtime, in which case all tasks created will be on the same
    // thread.
    #[tokio::test(flavor = "current_thread")]
    async fn test_current_thread_runtime_configuration() {
        let handle = tokio::spawn(async { std::thread::current().id() });
        let tid1 = handle.await.unwrap();

        let handle2 = tokio::spawn(async { std::thread::current().id() });
        let tid2 = handle2.await.unwrap();

        assert_eq!(tid1, tid2);
    }
}
