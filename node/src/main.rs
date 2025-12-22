// BlazeEVM Node Binary

use blazeevm_node::server;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8545".parse().unwrap();
    println!("Starting BlazeEVM Node...");

    if let Err(e) = server::start(addr).await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}
