//

use tokio::time::{sleep, Duration};

async fn fetch_data(id: u32, sleep_sec: u64) -> String {
    // Simulate network delay
    sleep(Duration::from_secs(sleep_sec)).await;
    format!("Data for id {}", id)
}

#[tokio::main]

async fn main(){
    println!("async function example!");
    let data42 = fetch_data(42, 20); // fetch_data(42, 5).await vs fetch_data(42, 5)
    // fetch_data(...) does NOT start running immediately. It only creates a Future. rust Async functions are lazy. Nothing runs until you .await them.
    let data43 = fetch_data(43, 10);
    println!("Waiting for data...");
    println!("Received: {}", data42.await); // prints at 20 sec
    println!("Received: {}", data43.await); // prints at 30 sec
}

/*
for this program, after starting:
Received: Data for id 42, printed after 20 sec
Received: Data for id 43, printed after 30 sec
why so, if thet are non blocking 
shouldn't id 43 be printed after 10 sec and 42 after 20 sec,
why 43 waits for 42 to be finished, then "start its task" ?
*/

/*
// Timeline: 
| Time | What happens                    |
| ---- | ------------------------------- |
| 0s   | data42 future created           |
| 0s   | data43 future created           |
| 0s   | await data42 → starts 20s sleep |
| 20s  | id 42 printed                   |
| 20s  | await data43 → starts 10s sleep |
| 30s  | id 43 printed                   |

*/

// cargo run --bin 07b_function_async2