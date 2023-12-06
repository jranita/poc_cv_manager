use anyhow::Result;
use argon2::{
    password_hash::{Error, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use rand::{self, distributions::Alphanumeric, rngs::OsRng, Rng};
use salvo::basic_auth::{BasicAuth, BasicAuthValidator};
use salvo::prelude::*;
use serde::Deserialize;

use crate::{models::user::User, utils::app_error::AppError};

#[derive(Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

pub fn authorize_user(user: &User, credentials: &Credentials) -> Result<String, Error> {
    let db_hash = PasswordHash::new(&user.pass)?;
    let argon = Argon2::default();

    argon.verify_password(&credentials.password.as_bytes(), &db_hash)?;
    Ok(rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect())
}

pub fn hash_password(new_password: String) -> Result<String> {
    let salt = SaltString::generate(OsRng);
    let argon = argon2::Argon2::default();
    let hashed_password = argon
        .hash_password(&new_password.as_bytes(), &salt)
        .map_err(|_| AppError::PasswordHashingError)?;

    Ok(hashed_password.to_string())
}

#[derive(Deserialize)]
pub struct Validator {
    pub username: String,
    pub password: String,
}

#[async_trait]
impl BasicAuthValidator for Validator {
    async fn validate(&self, username: &str, password: &str, _depot: &mut Depot) -> bool {
        username == "root" && password == "pwd"
    }
}

pub fn auth_handler() -> BasicAuth<Validator> {
    return BasicAuth::new(Validator {
        username: String::new(),
        password: String::new(),
    })
}

// #[cfg(test)]
// mod tests {
//     use salvo::prelude::*;
//     use salvo::test::{ResponseExt, TestClient};

//     #[tokio::test]
//     async fn test_basic_auth() {
//         let service = Service::new(super::route());

//         let content = TestClient::get("http://127.0.0.1:5801/")
//             .basic_auth("root", Some("pwd"))
//             .send(&service)
//             .await
//             .take_string()
//             .await
//             .unwrap();
//         assert!(content.contains("Hello"));

//         let content = TestClient::get("http://127.0.0.1:5801/")
//             .basic_auth("root", Some("pwd2"))
//             .send(&service)
//             .await
//             .take_string()
//             .await
//             .unwrap();
//         assert!(content.contains("Unauthorized"));
//     }
// }
