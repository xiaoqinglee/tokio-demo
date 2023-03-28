use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

// #[tokio::main]
// async fn main() {
//     // Bind the listener to the address
//     let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
//
//     loop {
//         // The second item contains the IP and port of the new connection.
//         let (socket, _) = listener.accept().await.unwrap();
//         process(socket).await;
//     }
// }

// async fn process(socket: TcpStream) {
//     // The `Connection` lets us read/write redis **frames** instead of
//     // byte streams. The `Connection` type is defined by mini-redis.
//     let mut connection = Connection::new(socket);
//
//     if let Some(frame) = connection.read_frame().await.unwrap() {
//         println!("GOT: {:?}", frame);
//
//         // Respond with an error
//         let response = Frame::Error("unimplemented".to_string());
//         connection.write_frame(&response).await.unwrap();
//     }
// }

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    // A hashmap is used to store data
    let mut db = HashMap::new();

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);

    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // The value is stored as `Vec<u8>`
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`. This
                    // type will be covered later in the tutorial. For now,
                    // `&Vec<u8>` is converted to `Bytes` using `into()`.
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}

// We want our Redis server to process many concurrent requests.
// To do this, we need to add some concurrency.
//
// To process connections concurrently, a new task is spawned for each inbound connection.
// The connection is processed on this task.
//
// Tasks
//
// A Tokio task is an asynchronous green thread.
// They are created by passing an async block to tokio::spawn.
// The tokio::spawn function returns a JoinHandle,
// which the caller may use to interact with the spawned task.
// The async block may have a return value.
// The caller may obtain the return value using .await on the JoinHandle.
//
// For example:
//
// #[tokio::main]
// async fn main() {
//     let handle = tokio::spawn(async {
//         // Do some async work
//         "return value"
//     });
//
//     // Do some other work
//
//     let out = handle.await.unwrap();
//     println!("GOT {}", out);
// }
//
// Awaiting on JoinHandle returns a Result.
// When a task encounters an error during execution, the JoinHandle will return an Err.
// This happens when the task either panics, or if the task is forcefully cancelled by the runtime shutting down.
//
// Tasks are the unit of execution managed by the scheduler.
// Spawning the task submits it to the Tokio scheduler,
// which then ensures that the task executes when it has work to do.
// The spawned task may be executed on the same thread as where it was spawned,
// or it may execute on a different runtime thread.
// The task can also be moved between threads after being spawned.
//
// Tasks in Tokio are very lightweight.
// Under the hood, they require only a single allocation and 64 bytes of memory.
// Applications should feel free to spawn thousands, if not millions of tasks.

// 'static bound
//
// When you spawn a task on the Tokio runtime, its type's lifetime must be 'static.
// This means that the spawned task must not contain any references to data owned outside the task.
//
// For example, the following will not compile:
//
// use tokio::task;
//
// #[tokio::main]
// async fn main() {
//     let v = vec![1, 2, 3];
//
//     task::spawn(async {
//         println!("Here's a vec: {:?}", v);
//     });
// }

// Send bound
//
// Tasks spawned by tokio::spawn must implement Send.
// This allows the Tokio runtime to move the tasks between threads while they are suspended at an .await.
//
// Tasks are Send when all data that is held across .await calls is Send. This is a bit subtle.
// When .await is called, the task yields back to the scheduler.
// The next time the task is executed, it resumes from the point it last yielded.
// To make this work, all state that is used after .await must be saved by the task.
// If this state is Send, i.e. can be moved across threads, then the task itself can be moved across threads.
// Conversely, if the state is not Send, then neither is the task.
//
// For example, this works:
//
// use tokio::task::yield_now;
// use std::rc::Rc;
//
// #[tokio::main]
// async fn main() {
//     tokio::spawn(async {
//         // The scope forces `rc` to drop before `.await`.
//         {
//             let rc = Rc::new("hello");
//             println!("{}", rc);
//         }
//
//         // `rc` is no longer used. It is **not** persisted when
//         // the task yields to the scheduler
//         yield_now().await;
//     });
// }
//
// This does not:
//
// use tokio::task::yield_now;
// use std::rc::Rc;
//
// #[tokio::main]
// async fn main() {
//     tokio::spawn(async {
//         let rc = Rc::new("hello");
//
//         // `rc` is used after `.await`. It must be persisted to
//         // the task's state.
//         yield_now().await;
//
//         println!("{}", rc);
//     });
// }

