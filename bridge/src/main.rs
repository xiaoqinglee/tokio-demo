use tokio::runtime::Builder;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

pub struct Task {
    name: String,
    // info that describes the task
}

async fn handle_task(task: Task) {
    println!("Running task {}", task.name);
    sleep(Duration::from_millis(5)).await;
    println!("Task {} exits", task.name);
}

pub struct TaskSpawner {
    task_sender: mpsc::Sender<Task>,
    std_thread_join_handle: std::thread::JoinHandle<()>,
}

impl TaskSpawner {
    pub fn new() -> TaskSpawner {
        // Set up a channel for communicating.
        let (send, mut recv) = mpsc::channel(16);

        // Build the runtime for the new thread.
        //
        // The runtime is created before spawning the thread
        // to more cleanly forward errors if the `unwrap()`
        // panics.
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let std_thread_join_handle = std::thread::spawn(move || {
            runtime.block_on(async move {
                let mut tokio_task_spawn_join_handles = vec![];
                while let Some(task) = recv.recv().await {
                    let handle = tokio::spawn(handle_task(task));
                    tokio_task_spawn_join_handles.push(handle)
                }
                println!("have left while loop");
                for handle in tokio_task_spawn_join_handles{
                    handle.await.unwrap();
                }
                println!("all tasks have exit");
            });
            println!("std thread exits");
        });

        TaskSpawner {
            task_sender: send,
            std_thread_join_handle,
        }
    }

    pub fn spawn_task(&self, task: Task) {
        match self.task_sender.blocking_send(task) {
            Ok(()) => {},
            Err(_) => panic!("The shared runtime has shut down."),
        }
    }
}

fn main() {
    let spawner = TaskSpawner::new();
    spawner.spawn_task(Task{name: String::from("foo")});
    spawner.spawn_task(Task{name: String::from("bar")});
    std::thread::sleep(Duration::from_millis(1));
    drop(spawner.task_sender);
    spawner.std_thread_join_handle.join().unwrap();
    println!("main std thread exits");
}

// Running task foo
// Running task bar
// have left while loop
// Task foo exits
// Task bar exits
// all tasks have exit
// std thread exits
// main std thread exits
