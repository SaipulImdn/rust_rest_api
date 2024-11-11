use bcrypt::verify;
use uuid::Uuid;

use crate::models::user::User;
use crate::utils::hash::{hash_password, verify_password};

pub async fn edit_account(
    user_id: Uuid,
    edit_data: EditAccountRequest,
) -> Result<User, &'static str> {
    let mut user = get_user_by_id(user_id).await?;

    if let Some(username) = edit_data.username {
        user.username = username;
    }

    if let Some(email) = edit_data.email {
        user.email = email;
    }

    if let Some(password) = edit_data.password {
        user.password = hash_password(&password).map_err(|_| "Password hash error")?;
    }

    update_user_in_db(user.clone()).await?;
    Ok(user)
}

pub async fn delete_account(
    user_id: Uuid,
    password: &str,
) -> Result<(), &'static str> {
    let user = get_user_by_id(user_id).await?;

    if verify_password(password, &user.password).map_err(|_| "Password verification failed")? {
        delete_user_from_db(user_id).await?;
        Ok(())
    } else {
        Err("Incorrect password")
    }
}

async fn get_user_by_id(user_id: Uuid) -> Result<User, &'static str> {
    Ok(User {
        id: user_id,
        username: "example_user".to_string(),
        email: "example@example.com".to_string(),
        password: "$2y$12$examplehashedpassword".to_string(),
    })
}

async fn update_user_in_db(user: User) -> Result<(), &'static str> {
    Ok(())
}

async fn delete_user_from_db(user_id: Uuid) -> Result<(), &'static str> {
    Ok(())
}
