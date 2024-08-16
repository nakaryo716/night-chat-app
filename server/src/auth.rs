use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{async_trait, extract::FromRef};
use serde::{Deserialize, Serialize};
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
    fn new(user_mail: UserMail, user_pass: UserPass) -> Self {
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

#[derive(Debug, Clone)]
pub struct UserDataDb {
    pool: Arc<Mutex<HashMap<UserId, UserData>>>,
}

impl UserDataDb {
    pub fn new() -> Self {
        Self {
            pool: Arc::default(),
        }
    }
}

impl FromRef<AppState> for UserDataDb {
    fn from_ref(input: &AppState) -> Self {
        input.users_pool.clone()
    }
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("query error")]
    DbError,
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
    async fn verify_password(&self, credential: Credential) -> Result<Option<Self::UserData>, Self::Error>;
    // this method is called by handler 
    // parse cookie -> get user id -> delete user
    async fn delete_user(&self, credential: UserId) -> Result<(), Self::Error>;
}
