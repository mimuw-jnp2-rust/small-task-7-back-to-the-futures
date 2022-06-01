use std::time::{Duration, Instant};

use future_combinators::executor::new_executor_and_spawner;
use future_combinators::timer::TimerFuture;
use future_combinators::FutureExt;

#[test]
fn join_works() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        println!("howdy!");
        let future1 = TimerFuture::new(Duration::from_secs(2))
            .map(|_| println!("goodbye!"))
            .map(|_| 42);
        let future2 =
            TimerFuture::new(Duration::from_secs(2)).map(|_| println!("goodbye for real!"));
        let future = future1.join(future2);
        assert_eq!((42, ()), future.await);
    });

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    let now = Instant::now();
    // Run the executor until the task queue is empty.
    executor.run();

    assert!(now.elapsed() <= Duration::from_secs(3));
}
