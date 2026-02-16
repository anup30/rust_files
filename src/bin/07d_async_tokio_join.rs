// tokio::join! - Run Multiple Futures Concurrently

use tokio::time::{sleep, Duration};

async fn fetch_data(id: u32, sleep_sec: u64) -> String {
    // wait time
    sleep(Duration::from_secs(sleep_sec)).await; // from_millis
    format!("Data for id {}", id)
}

async fn fetch_user(id: u32) -> String {
    sleep(Duration::from_secs(3)).await;
    format!("User {}", id)
}

async fn fetch_orders(user_id: u32) -> Vec<String> {
    sleep(Duration::from_secs(5)).await;
    vec![format!("Order for user {}", user_id)]
}

#[tokio::main]

async fn main(){
    println!("tokio::join!() example");

    let data42 = fetch_data(42, 10);
    let data43 = fetch_data(43, 5);

    println!("Waiting for data...");

    let (r42, r43) = tokio::join!(data42, data43); // tokio::join! - Run Multiple Futures Concurrently

    println!("Received: {}", r42); // prints at 10 sec
    println!("Received: {}", r43); // prints at 10 sec
/*
// Timeline:
| Time | What happens       |
| ---- | ------------------ |
| 0s   | both futures start |
| 5s   | id 43 ready        |
| 10s  | id 42 ready        |
| 10s  | both printed       |

*/

    // All three run concurrently:
    let (user, orders, prefs) = tokio::join!(
        fetch_user(44),
        fetch_orders(44),
        async {
            sleep(Duration::from_millis(100)).await;
            "Preferences".to_string()
        }
    );
    println!("User: {}, Orders: {:?}, Prefs: {}", user, orders, prefs); // User: User 44, Orders: ["Order for user 44"], Prefs: Preferences
}

// cargo run --bin 07b_function_async3

