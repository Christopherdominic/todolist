use dotenvy::dotenv; // 'dotenvy' is the modern, maintained version of 'dotenv'
use postgres::{Client, NoTls};
use std::env;
use std::fs;
use std::net::TcpListener;

fn main() {
    // 1. Load environment variables first
    dotenv().ok();

    // 2. Setup Database using DATABASE_URL from .env or environment
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env or the environment");
    let mut client = Client::connect(&db_url, NoTls)
        .expect("Failed to connect to database. Check DATABASE_URL and database credentials.");

    // Run migrations if needed
    let row = client.query_one("SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'todos')", &[]).unwrap();
    if !row.get::<_, bool>(0) {
        println!("Running database migrations...");
        let sql = fs::read_to_string("src/migrations/001_create_todos.sql")
            .expect("Failed to read migration file");
        client.batch_execute(&sql)
            .expect("Failed to run migration");
        println!("Migrations completed.");
    }

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
