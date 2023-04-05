use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        println!("running in task");
        sleep(Duration::from_millis(10)).await;
        println!("continuing in task");
        "return value"
    });

    println!("running in main");
    sleep(Duration::from_millis(5)).await;
    println!("continuing in main");

    let out = handle.await.unwrap();
    println!("GOT {}", out);
}

// running in main
// running in task
// continuing in main
// continuing in task
// GOT return value

// 如果注释掉 handle.await.unwrap() 那么会得到:
// running in main
// running in task
// continuing in main