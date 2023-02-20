use tokio::task;

async fn our_async_program()  -> Result<String> {
    future::ok("Hello world".to_string()).await
}

fn fib_cpu_intensive(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib_cpu_intensive(n - 1) + fib_cpu_intensive(n - 2),
    }
}

#[tokio::main]
async fn main() {
    let concurrent_future = task::spawn(our_async_program());
    let threadpool_future = task::spawn_blocking(||fib_cpu_intensive(30));
    todo!()
}

