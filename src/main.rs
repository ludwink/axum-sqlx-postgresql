use std::env;

use dotenvy::dotenv;
use tokio::net::TcpListener;

mod router;

use router::router;

#[tokio::main]
async fn main() {
    // Load .env file (development mode)
    dotenv().ok();

    // TcpListener
    let port: u16 = env::var("PORT")
        .expect("PORT undefined")
        .parse()
        .expect("PORT value must be u16");

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to create TcpListener to specified address");

    // Start server
    println!("Server running on port: {}", listener.local_addr().unwrap());

    axum::serve(listener, router())
        .await
        .expect("Failed to start Axum server")
}
