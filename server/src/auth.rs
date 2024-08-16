use axum::{async_trait, extract::FromRef};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use thiserror::Error;
use uuid::Uuid;

use crate::app_state::AppState;

mod handler;
// UserId wraped uuid
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct UserId(String);

impl UserId {
    pub fn get_id_txt(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserMail(String);

impl UserMail {
    pub fn new(user_mail: &str) -> Self {
        Self(user_mail.to_owned())
    }

    pub fn get_mail_txt(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserPass(String);

impl UserPass {
    pub fn new(user_pass: &str) -> Self {
        Self(user_pass.to_owned())
    }

    pub fn get_pass_txt(&self) -> &str {
        &self.0
    }
}

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
}

#[derive(Debug, Clone, Serialize)]
pub struct UserData {
    user_id: UserId,
    user_mail: UserMail,
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

#[async_trait]
pub trait AuthManageDb {
    type UserData;
    type Error;

    // insert new user
    // this method is called by handler with payload
    // parse payload -> instance UserData -> insert to db
    async fn insert_new_user(&self, credential: UserData) -> Result<Self::UserData, Self::Error>;
    // this method is not called by handler
    // called other trait method befor excute to db
    // e.g. befor insert_new_user to avoid daplicate user
    async fn have_user(&self, credential: UserMail) -> Result<bool, Self::Error>;
    // this method is called by session handler
    // parse payload(Credential) -> get user pass that keeped db -> verify password -> create session
    async fn verify_password(
        &self,
        credential: Credential,
    ) -> Result<Option<Self::UserData>, Self::Error>;
    // this method is called by handler
    // parse cookie -> get user id -> delete user
    async fn delete_user(&self, credential: UserId) -> Result<(), Self::Error>;
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("query error")]
    DbError,
}
