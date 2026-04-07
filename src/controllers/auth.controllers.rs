use hex_literal::hex;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use uuid::Uuid;

pub struct User {
    id: u32,
    username: String,
    password: String,
}
 
pub struct Token {
    user_id: u32,
    token: String,
}

pub fn register_user(username: &str, password: &str) -> Result<u32, &'static str> {
    // Simulated database: a HashMap of users
    let mut users: HashMap<u32, User> = HashMap::new();
    let mut user_id: u32 = 1;
    // Check if the username already exists
    if users.values().any(|user| user.username == username) {
        return Err("Username already exists");
    }
    // Create a new user,hash the user password and add to the database
    let hash = Sha256::digest(b"{}", password.to_string());

    let new_user = User {
        id: user_id,
        username: username.to_string(),
        password: hash,
    };

    users.insert(user_id, new_user);
    user_id += 1;
    Ok(new_user.id)
}

pub fn login_user(username: &str, password: &str) -> Result<Token, &'static str> {
    let mut users: HashMap<u32, User> = HashMap::new();
    // Populate the users HashMap with some sample data
    // ...
    let hash = Sha256::digest(b"{}", password.to_string());

    let user = users
        .values()
        .find(|user| user.username == username && user.password == hash);
    if let Some(user) = user {
        let token = Uuid::new_v4().to_string();
        let token_data = Token {
            user_id: user.id,
            token: token.clone(),
        };
        Ok(token_data)
    } else {
        Err("Invalid credentials")
    }
}

pub fn verify_token(token: &str) -> Result<User, &'static str> {
    let mut users: HashMap<u32, User> = HashMap::new();
    // Populate the users HashMap with sample data
    // ...
    let token_data: Token = serde_json::from_str(token).unwrap();
    let user = users.get(&token_data.user_id);
    if let Some(user) = user {
        Ok(user.clone())
    } else {
        Err("Invalid token")
    }
}


