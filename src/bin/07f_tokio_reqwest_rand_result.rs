// Error Handling - Async Rust integrates naturally with Result
// reqwest crate provides async HTTP client functionality
// rand crate provides random number generation

use rand::Rng; //use rand::rngs::OsRng; -> more secure for passwords/tokens etc

async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    // ^ url: &str vs String
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    let base_url = String::from("https://jsonplaceholder.typicode.com/users");
    
    let mut rng = rand::thread_rng();
    // random id between 1 and 10 (inclusive)
    let user_id = rng.gen_range(1..=10);
    println!("user id: {}", user_id);
    let url = format!("{}/{}", base_url, user_id);
    match fetch_data(&url).await {
        Ok(data) => println!("Fetched: \n{}", data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
// cargo run --bin test

/*
add to Cargo.toml,
[dependencies]
rand = "0.8"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
*/
