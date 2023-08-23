use std::thread::sleep;
use std::time::Duration;
use tokio::task;
/*
 * Grab a random number, say that you're going to sleep
 * for that number of ms, and say when the sleep is complete
 */
async fn random_print(num: i32) {
    let r = rand::random::<u64>() % 500u64;
    println!("msg{} will sleep for {}ms", num, r);
    sleep(Duration::from_millis(r));
    println!("msg{} complete", num);
}
/*
 * mark main() as the main function to use with the tokio runtime.
 * It's normally ok to just use #[tokio::main], but here, I am
 * explicitly saying I want to configure the runtime to use
 * multiple threads, and I want 8 threads configured.
 */
// #[tokio::main(flavor = "multi_thread", worker_threads = 8)]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    /* create a vector for the join handles. This is a nice to have, but not required */
    let mut handles = vec![];

    /* let's start a function 10 times */
    for j in 0..10000 {
        /* start a task */
        let handle = task::spawn(random_print(j));
        /*
         * save the handle for later use. This handle can be used
         * to check if the task is complete later.
         */
        handles.push(handle);
    }
    /*
     * the next loop is useful if you want to explicitly wait for all tasks to complete.
     * A more complex implementation might require that you only wait for certain
     * tasks to complete
     */
    for handle in handles {
        let _ = handle.await;
    }
}
