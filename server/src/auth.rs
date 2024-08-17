use axum::{async_trait, extract::FromRef};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, MySqlPool};
use thiserror::Error;
use uuid::Uuid;

use crate::app_state::AppState;

mod handler;
// UserId wraped uuid
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, FromRow)]
pub struct UserId(String);

impl UserId {
    pub fn get_id_txt(&self) -> &str {
        &self.0
    }
}

// indicate user mail
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, FromRow)]
pub struct UserMail(String);

impl UserMail {
    pub fn new(user_mail: &str) -> Self {
        Self(user_mail.to_owned())
    }

    pub fn get_mail_txt(&self) -> &str {
        &self.0
    }
}

// indicate user password that hashed
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, FromRow)]
pub struct UserPass(String);

impl UserPass {
    pub fn new(user_pass: &str) -> Self {
        Self(user_pass.to_owned())
    }

    pub fn get_pass_txt(&self) -> &str {
        &self.0
    }
}

// wraped UserMail & UserPass
// this struct send by client as a json payload
#[derive(Debug, Clone, Deserialize)]
pub struct Credential {
    user_mail: UserMail,
    user_pass: UserPass,
}

impl Credential {
    pub fn new(user_mail: UserMail, user_pass: UserPass) -> Self {
        Self {
            user_mail,
            user_pass,
        }
    }

    pub fn get_user_mail(&self) -> &UserMail {
        &self.user_mail
    }
}

// UserData is stored by database
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct UserData {
    #[sqlx(flatten)]
    user_id: UserId,
    #[sqlx(flatten)]
    user_mail: UserMail,
    #[sqlx(flatten)]
    user_pass: UserPass,
}

impl UserData {
    pub fn new(payload: Credential) -> Self {
        Self {
            user_id: UserId(Uuid::new_v4().to_string()),
            user_mail: payload.user_mail,
            user_pass: payload.user_pass,
        }
    }

    pub fn get_user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn get_user_mail(&self) -> &UserMail {
        &self.user_mail
    }

    pub fn get_user_pass(&self) -> &UserPass {
        &self.user_pass
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum DbConnectionError {
    #[error("connection failed")]
    ConectionRefused,
}

// connectionpool
// this struct is used and sheared by endpoint as a axum::extract::State<_>
#[derive(Debug, Clone)]
pub struct UserDataDb {
    pool: MySqlPool,
}

impl UserDataDb {
    pub async fn new(database_url: &str) -> Result<Self, DbConnectionError> {
        let pool = MySqlPool::connect(database_url)
            .await
            .map_err(|_e| DbConnectionError::ConectionRefused)?;
        Ok(Self { pool })
    }
}

impl FromRef<AppState> for UserDataDb {
    fn from_ref(input: &AppState) -> Self {
        input.users_pool.clone()
    }
}

// this trait is used by databasepool to use in handler
#[async_trait]
pub trait AuthManageDb {
    type UserData;
    type Error;

    // insert new user
    // this method is called by handler with payload
    // parse payload -> instance UserData -> insert to db
    async fn insert_new_user(&self, credential: UserData) -> Result<Self::UserData, Self::Error>;
    // this method is called by session handler
    // parse payload(Credential) -> get user pass that keeped db -> verify password -> create session
    async fn verify_password(&self, credential: Credential) -> Result<Self::UserData, Self::Error>;
    // this method is called by handler
    // parse cookie -> get user id -> delete user
    async fn delete_user(&self, credential: Credential) -> Result<(), Self::Error>;
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("User not found")]
    UserNotFound,
    #[error("Different pass")]
    DifferentPassword,
    #[error("Database error")]
    DbError,
}

#[async_trait]
impl AuthManageDb for UserDataDb {
    type UserData = UserData;
    type Error = AuthError;

    // only create new user
    // this mehod not check already user have
    async fn insert_new_user(&self, credential: UserData) -> Result<Self::UserData, Self::Error> {
        let user_data: UserData = sqlx::query_as(
            r#"
            "#,
        )
        .bind(credential.get_user_id().get_id_txt())
        .bind(credential.get_user_mail().get_mail_txt())
        .bind(credential.user_pass.get_pass_txt())
        .fetch_one(&self.pool)
        .await
        .map_err(|_e| AuthError::DbError)?;

        Ok(user_data)
    }

    // verify password between client sended and database have
    // if user not found, return error(check)
    async fn verify_password(&self, credential: Credential) -> Result<Self::UserData, Self::Error> {
        let query_user: Option<UserData> = sqlx::query_as(
            r#"
            "#,
        )
        .bind(credential.user_mail.get_mail_txt())
        .fetch_optional(&self.pool)
        .await
        .map_err(|_e| AuthError::DbError)?;

        match query_user {
            Some(user) => {
                let payload_pass = credential.user_pass.get_pass_txt();
                let database_pass = user.get_user_pass().get_pass_txt();

                if payload_pass == database_pass {
                    Ok(user)
                } else {
                    Err(AuthError::DifferentPassword)
                }
            }
            None => Err(AuthError::UserNotFound),
        }
    }

    // delete user
    // if user not found, return error
    async fn delete_user(&self, credential: Credential) -> Result<(), Self::Error> {
        sqlx::query(
            r#"
            "#,
        )
        .bind(credential.get_user_mail().get_mail_txt())
        .execute(&self.pool)
        .await
        .map_err(|_e| AuthError::DbError)?;
        Ok(())
    }
}
