use async_std;

async fn hello() {
    let answer = 1_234_5;
    println!("Hello, {}", answer);
}

#[async_std::main]
async fn main() {
    hello().await;
}
