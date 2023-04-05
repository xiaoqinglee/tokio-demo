fn main() {
}
// What #[tokio::main] expands to
//
// The #[tokio::main] macro is a macro
// that replaces your main function with a non-async main function
// that starts a runtime and then calls your code.
//
// For instance, this:
//
// #[tokio::main]
// async fn main() {
//     println!("Hello world");
// }
//
// is turned into this:
//
// fn main() {
//     tokio::runtime::Builder::new_multi_thread()
//         .enable_all()
//         .build()
//         .unwrap()
//         .block_on(async {
//             println!("Hello world");
//         })
// }
//
// by the macro.
// To use async/await in our own projects,
// we can do something similar
// where we leverage the block_on method to enter the asynchronous context where appropriate.