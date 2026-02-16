// tokio::spawn(future) // Run in background task

use tokio::time::{Duration, sleep};

async fn background_job(name: &'static str) {
    for i in 1..=3 {
        println!("{}: iteration {}", name, i);
        sleep(Duration::from_millis(100)).await;
    }
}

#[tokio::main]
async fn main() {
    // Spawn independent tasks
    let handle1 = tokio::spawn(background_job("Task A"));
    let handle2 = tokio::spawn(background_job("Task B"));

    println!("Main: doing other work...");

    // Wait for completion
    let _ = tokio::join!(handle1, handle2);
}

// cargo run --bin 07b_function_async4_tokio_spawn

/*
// output:
Main: doing other work...
Task B: iteration 1
Task A: iteration 1
Task A: iteration 2
Task B: iteration 2
Task B: iteration 3
Task A: iteration 3
*/

/*
// MPSC (Multiple Producer, Single Consumer)
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);  // Buffer size 32

    // Spawn producer
    tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(format!("Message {}", i)).await.unwrap();
        }
    });

    // Consume messages
    while let Some(msg) = rx.recv().await {
        println!("Received: {}", msg);
    }
}

// output:
// Received: Message 1
// Received: Message 2
// Received: Message 3
// Received: Message 4
// Received: Message 5

*/

/*
// Oneshot (Single Response)
use tokio::sync::oneshot;

async fn compute(tx: oneshot::Sender<i32>) {
    // Do work...
    let _ = tx.send(42);
}

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(compute(tx));

    match rx.await {
        Ok(value) => println!("Got: {}", value),
        Err(_) => println!("Sender dropped"),
    }
}
*/

/*
// Advanced Control Flow - select! - Race Between Futures
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(1);

    tokio::spawn(async move {
        sleep(Duration::from_millis(200)).await;
        let _ = tx.send("Data arrived").await;
    });

    tokio::select! {
        Some(msg) = rx.recv() => {
            println!("Received: {}", msg);
        }
        _ = sleep(Duration::from_millis(100)) => {
            println!("Timeout!");
        }
    }
}
*/

/*
// Shared State with Async Mutex
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            for _ in 0..100 {
                let mut num = counter.lock().await;
                *num += 1;
                // Lock auto-released when `num` drops
            }
        }));
    }

    for h in handles { h.await.unwrap(); }
    println!("Final: {}", *counter.lock().await);  // 1000
}
// Note: Use tokio::sync::Mutex, not std::sync::Mutex, to avoid blocking the async runtime .

*/

/*
// Async I/O Example: TCP Echo Server
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handle_client(mut socket: TcpStream) {
    let mut buf = [0u8; 1024];

    loop {
        match socket.read(&mut buf).await {
            Ok(0) => return,  // Connection closed
            Ok(n) => {
                if socket.write_all(&buf[..n]).await.is_err() {
                    return;
                }
            }
            Err(_) => return,
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New client: {}", addr);
        tokio::spawn(handle_client(socket));  // Handle concurrently
    }
}
*/

/*
| Practice                         | Why It Matters                                                            |
| -------------------------------- | ------------------------------------------------------------------------- |
| **Don't block the runtime**      | Use `tokio::task::spawn_blocking` for CPU-intensive or blocking I/O work  |
| **Prefer `tokio::sync::Mutex`**  | Standard `Mutex` blocks threads, killing concurrency                      |
| **Use bounded channels**         | Prevents memory exhaustion under load                                     |
| **Clone senders, not receivers** | Multiple producers, single consumer pattern                               |
| **Handle cancellation**          | Tasks may be dropped; use `select!` for cleanup                           |

Pro Tip: Start with single-threaded Tokio (#[tokio::main(flavor = "current_thread")])
until you need parallelism—it's simpler and avoids many Send/Sync complexity issues
*/
