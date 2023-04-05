use tokio::runtime::Builder;
use tokio::time::{sleep, Duration};

fn main() {
    let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        handles.push(runtime.spawn(my_bg_task(i)));
        println!("task {i} created.");
    }
    println!("10 tasks created.");

    // Do something time-consuming while the background tasks execute.
    std::thread::sleep(Duration::from_millis(750));
    println!("Finished time-consuming task.");

    // Wait for all of them to complete.
    for handle in handles {
        // The `spawn` method returns a `JoinHandle`. A `JoinHandle` is
        // a future, so we can wait for it using `block_on`.
        runtime.block_on(handle).unwrap();
    }
}

async fn my_bg_task(i: u64) {
    // By subtracting, the tasks with larger values of i sleep for a
    // shorter duration.
    let millis = 1000 - 50 * i;
    println!("Task {} running. Going to sleep for {} ms.", i, millis);

    sleep(Duration::from_millis(millis)).await;

    println!("Task {} stopping.", i);
}

// task 0 created.
// task 1 created.
// Task 0 running. Going to sleep for 1000 ms.
// task 2 created.
// task 3 created.
// task 4 created.
// task 5 created.
// task 6 created.
// task 7 created.
// Task 1 running. Going to sleep for 950 ms.
// task 8 created.
// task 9 created.
// 10 tasks created.
// Task 2 running. Going to sleep for 900 ms.
// Task 3 running. Going to sleep for 850 ms.
// Task 4 running. Going to sleep for 800 ms.
// Task 5 running. Going to sleep for 750 ms.
// Task 6 running. Going to sleep for 700 ms.
// Task 7 running. Going to sleep for 650 ms.
// Task 8 running. Going to sleep for 600 ms.
// Task 9 running. Going to sleep for 550 ms.
// Task 9 stopping.
// Task 8 stopping.
// Task 7 stopping.
// Task 6 stopping.
// Finished time-consuming task.
// Task 5 stopping.
// Task 4 stopping.
// Task 3 stopping.
// Task 2 stopping.
// Task 1 stopping.
// Task 0 stopping.
