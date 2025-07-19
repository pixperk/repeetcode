use sqlx::{query_as, PgPool};
use bcrypt::{hash, DEFAULT_COST};

use crate::{auth::User, error::{AppError, AppResult}};

pub async fn create_user_if_not_exists(db : &PgPool, username : &str, email : &str,  password : &str) -> AppResult<User>{
    let existing: Option<User> = query_as!(
        User,
        "SELECT * FROM users WHERE username = $1 or email = $2",
        username,
        email
    )
    .fetch_optional(db)
    .await
    .map_err(|err|AppError::Database(err.to_string()))?;

    if let Some(user) = existing {
        return Err(AppError::Database(format!(
            "User with username '{}' or email '{}' already exists",
            user.username, user.email.unwrap())
        ));
    }

    let user = query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password)
        VALUES ($1, $2, $3)
        RETURNING id, username, email, password, created_at, updated_at
        "#,
        username,
        email,
        password
    )
    .fetch_one(db)
    .await
    .map_err(|err|AppError::Database(err.to_string()))?;

    Ok(user)
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST).map_err(|e| AppError::InternalWith(e.to_string()))
}

pub async fn get_user_by_username(db: &PgPool, username: &str) -> AppResult<User> {
    let user = query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        username
    )
    .fetch_one(db)
    .await
    .map_err(|err| AppError::Database(err.to_string()))?;
    Ok(user)
}