// popular runtimes include Tokio (the most widely used), async‑std, and smol
// without a runtime, you cannot execute async code

/*
Cargo.toml:
[dependencies]
tokio = { version = "1", features = ["full"] }
*/

use tokio::time::{sleep, Duration};

async fn fetch_data(id: u32, sleep_time: u64) -> String {
    // Simulate network delay
    sleep(Duration::from_millis(sleep_time)).await;
    format!("Data for id {}", id)
}

#[tokio::main]
// ^ a procedural macro provided by tokio, generates a runtime, transforms async main into sync main

async fn main(){
    let data = fetch_data(42, 3000).await;
    println!("Received: {}", data);
}

/*
Avoid blocking calls (like std::fs::File::read) inside async functions. Use the runtime’s async APIs (e.g., tokio::fs) instead
*/


/*
// reading:
  https://rust-lang.github.io/async-book/
  https://tokio.rs/tokio/tutorial/async
  https://thenewstack.io/async-programming-in-rust-understanding-futures-and-tokio/
*/

/*
Pro Tip: Start with single-threaded Tokio (#[tokio::main(flavor = "current_thread")]) 
until you need parallelism—it's simpler and avoids many Send/Sync complexity issues 
*/

// rust async series - powered by Kimi K2.5, ChatGpt 5.2

// cargo run --bin 07b_function_async
