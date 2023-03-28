// Using async/await
//
// Async functions are called like any other Rust function.
// However, calling these functions does not result in the function body executing.
// Instead, calling an async fn returns a value representing the operation.
// This is conceptually analogous to a zero-argument closure.
// To actually run the operation, you should use the .await operator on the return value.

// The return value of an async fn is an anonymous type that implements the Future trait.

async fn say_world() {
    println!("world");
}

#[tokio::main]
async fn main() {
    // Calling `say_world()` does not execute the body of `say_world()`.
    let op = say_world();

    // This println! comes first
    println!("hello");

    // Calling `.await` on `op` starts executing `say_world`.
    op.await;
}

// Async main function
//
// The main function used to launch the application differs from the usual one found in most of Rust's crates.
//
//     It is an async fn
//     It is annotated with #[tokio::main]
//
// An async fn is used as we want to enter an asynchronous context.
// However, asynchronous functions must be executed by a runtime.
// The runtime contains the asynchronous task scheduler, provides evented I/O, timers, etc.
// The runtime does not automatically start, so the main function needs to start it.
//
// The #[tokio::main] function is a macro.
// It transforms the async fn main() into a synchronous fn main() that initializes a runtime instance and executes the async main function.
//
// For example, the following:
//
// #[tokio::main]
// async fn main() {
//     println!("hello");
// }
//
// gets transformed into:
//
// fn main() {
//     let mut rt = tokio::runtime::Runtime::new().unwrap();
//     rt.block_on(async {
//         println!("hello");
//     })
// }