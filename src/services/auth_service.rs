use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

use crate::models::user::User;

pub async fn register(username: &str, email: &str, password: &str) -> Result<User, &'static str> {
    let hashed_password = hash(password, DEFAULT_COST).map_err(|_| "Password hash error")?;
    let new_user = User {
        id: Uuid::new_v4(),
        username: username.to_string(),
        email: email.to_string(),
        password: hashed_password,
    };
    Ok(new_user)
}

pub async fn login(email: &str, password: &str) -> Result<String, &'static str> {
    let user = User {
        id: Uuid::new_v4(),
        username: "user1".to_string(),
        email: email.to_string(),
        password: "$2y$12$examplehashedpassword".to_string(),
    };
    if verify(password, &user.password).map_err(|_| "Hash verification error")? {
        Ok("Generated JWT token".to_string()) 
    } else {
        Err("Invalid credentials")
    }
}
