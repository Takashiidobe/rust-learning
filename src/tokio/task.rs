#[cfg(test)]
mod tests {
    use tokio::task;

    // spawn a task here
    // tasks should not perform syscalls or other operations than can block -- run them in an
    // asynchronous context instead
    #[tokio::test]
    async fn spawn() {
        let join = task::spawn(async { "hello world!" });

        let result = join.await.unwrap();
        assert_eq!(result, "hello world!");
    }

    #[tokio::test]
    async fn spawn_panic() {
        let join = task::spawn(async { panic!("oh no") });

        let result = join.await.is_err();
        assert!(result);
    }

    // you can spawn a blocking thread which allows it to call syscalls or other compute heavy
    // tasks
    #[tokio::test]
    async fn spawn_blocking() {
        let join = task::spawn_blocking(|| {
            // do some compute-heavy work or call synchronous code
            "blocking completed"
        });

        let result = join.await.unwrap();
        assert_eq!(result, "blocking completed");
    }

    // you can yield, which makes the current task yield to allow other tasks to be scheduled.
    // after yielding, after some time the task will be polled again for execution.
    #[tokio::test]
    async fn yielding() {
        let res = task::spawn(async { "first task" });

        task::yield_now().await;

        assert_eq!(res.await.unwrap(), "first task");
    }
}
