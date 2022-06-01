use std::time::{Duration, Instant};

use future_combinators::executor::new_executor_and_spawner;
use future_combinators::timer::TimerFuture;
use future_combinators::FutureExt;

#[test]
fn flat_map_works() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        let future = TimerFuture::new(Duration::from_secs(2)).map(|_| 3);
        let future2 = future
            .flat_map(|seconds| TimerFuture::new(Duration::from_secs(seconds)))
            .map(|_| 42);
        assert_eq!(42, future2.await);
    });

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    let now = Instant::now();
    // Run the executor until the task queue is empty.
    executor.run();

    assert!(now.elapsed() >= Duration::from_secs(5));
    assert!(now.elapsed() <= Duration::from_secs(6));
}
