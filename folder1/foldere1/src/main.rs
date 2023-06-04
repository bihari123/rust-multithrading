use std::{thread, time::Duration};

async fn printing() {
    println!("hi");
}

#[tokio::main]
async fn main() {
    printing().await;
    thread::sleep(Duration::from_secs(500))
}
