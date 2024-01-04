use std::{io, string::FromUtf8Error};

use salvo::{async_trait, writing::Text, Depot, Request, Response, Writer};
use thiserror::Error;

use crate::models::user::User;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("io: `{0}`")]
    Io(#[from] io::Error),
    #[error("utf8: `{0}`")]
    FromUtf8(#[from] FromUtf8Error),
    #[error("sqlx: `{0}`")]
    Sqlx(#[from] sqlx::Error),
    #[error("salvo: `{0}`")]
    Salvo(#[from] salvo::Error),
    #[error("serde::Serializer::Error")]
    SerdeSerialize,
    #[error("serde::Deserializer::Error")]
    SerdeDeserialize,
    // #[error("serde: `{0}`")]
    // Serde(#[from] serde::Error),
    #[error("json: `{0}`")]
    SerdeJson(#[from] serde_json::Error),
    // #[error("argon2: `{0}`")]
    // Argon2(#[from] argon2::Error),
    // #[error(transparent)]
    // Other(#[from] Anyhow::Error),
    #[error("Password hashing error")]
    PasswordHashingError,
}

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, depot: &mut Depot, res: &mut Response) {
        let user = depot.obtain::<User>();
        if user.unwrap().role == "admin" {
            res.render(Text::Plain("e".to_string()));
        } else {
            res.render(Text::Plain("I'm a error, hahaha!"));
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
