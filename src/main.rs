use dotenvy::dotenv; // 'dotenvy' is the modern, maintained version of 'dotenv'
use postgres::{Client, NoTls};
use std::env;
use std::net::TcpListener;

fn main() {
    // 1. Load environment variables first
    dotenv().ok();

    // 2. Setup Database (Ensure the string is complete)
    let db_url = "postgresql://postgres:postgres@localhost:5432/postgres";
    let mut _client = Client::connect(db_url, NoTls).expect("Failed to connect to database");

    // 3. Setup Port (Use {} instead of {:?})
    let port = env::var("PORT").unwrap_or_else(|_| "5000".to_string());
    let addr = format!("127.0.0.1:{}", port);

    // 4. Bind the listener
    let listener = TcpListener::bind(&addr).expect("Failed to bind to address");

    // 5. Correctly print the local address
    println!(
        "Server started on http://{}",
        listener.local_addr().unwrap()
    );

    // 6. we need a loop to actually handle connections!
    for stream in listener.incoming() {
        match stream {
            Ok(_s) => println!("New connection established!"),
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}
