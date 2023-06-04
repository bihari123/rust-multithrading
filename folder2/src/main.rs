use async_std::task;
use futures::executor::block_on;
use std::time::Duration;

fn main() {
    let future = async_main();
    block_on(future);
}

async fn async_main() {
    print_for_five("await 1").await;

    let async_one = print_for_five("async 1");
    let async_two = print_for_five("async 2");

    futures::join!(async_one, async_two);
    task::sleep(Duration::from_secs(500)).await;
}

async fn print_for_five(msg: &str) {
    for _ in 0..5 {
        task::sleep(Duration::from_secs(1)).await;
        println!("one second has passed: {}", msg)
    }
}
