use tokio::time::{sleep, Duration};

async fn greet(name: &str, num: u64) {
    println!("Starting greeting for {name}");
    sleep(Duration::from_secs(num)).await;
    println!("Hello, {name}!");
}

#[tokio::main]
async fn main() {
    greet("Ishaq", 5).await;
    greet("Bre", 2).await;
}