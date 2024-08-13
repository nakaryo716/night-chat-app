use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{async_trait, extract::{FromRef, State}, http::StatusCode, response::IntoResponse, routing::{delete, post}, Json, Router};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::{app_state::AppState, utility::acquire_lock};

// pub fn auth_router() -> Router {
//     Router::new()
//         .route("/user/create", post(create_user_handle))
//         .route("/user/verify", post(verify_user_handle))
//         .route("/user/delete", delete(delete_user_handle))
// }

pub async fn create_user_handle(
    State(app_state): State<UserDataDb>,
    Json(payload): Json<Credential>,
) -> Result<impl IntoResponse, StatusCode> {
    app_state
        .add_user(payload)
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub async fn verify_user_handle(
    State(app_state): State<UserDataDb>,
    Json(payload): Json<Credential>,
) -> Result<impl IntoResponse, StatusCode> {
    app_state
        .verify_user(payload)
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    Ok(StatusCode::OK)
}

pub async fn delete_user_handle(
    State(app_state): State<UserDataDb>,
    Json(payload): Json<Credential>,    
) -> Result<impl IntoResponse, StatusCode> {
    app_state
        .delete_user(payload)
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    Ok(StatusCode::OK)
}

#[derive(Debug, Clone, Serialize)]
pub struct UserData {
    user_id: String,
    user_mail: String,
    user_pass: String,
}

impl UserData {
    pub fn new(payload: Credential) -> Self {
        Self {
            user_id: Uuid::new_v4().to_string(),
            user_mail: payload.user_mail,
            user_pass: payload.user_pass,
        }
    }

    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credential {
    user_mail: String,
    user_pass: String,
}

impl Credential {
    pub fn new(user_mail: String, user_pass: String) -> Self {
        Self {
            user_mail,
            user_pass,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserDataDb {
    pool: Arc<Mutex<HashMap<String, UserData>>>,
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
impl AuthManage<Credential, Credential> for UserDataDb {
    type UserData = UserData;
    type Error = AuthError;

    async fn add_user(&self, new_user: Credential) -> Result<Self::UserData, Self::Error> {
        let user = UserData::new(new_user);
        match acquire_lock(&self.pool)
            .and_then(|mut lock| lock.insert(user.get_user_id().to_string(), user.clone()))
        {
            Some(_) => Ok(user),
            None => Ok(user),
        }
    }

    async fn verify_user(
        &self,
        credential: Credential,
    ) -> Result<Option<Self::UserData>, Self::Error> {
        match acquire_lock(&self.pool) {
            Some(lock) => {
                let user_data = lock
                    .iter()
                    .filter(|(_id, user)| credential.user_mail == user.user_mail)
                    .map(|(_id, user)| user.to_owned())
                    .next();

                Ok(user_data)
            }
            None => Err(AuthError::DbError),
        }
    }

    async fn delete_user(
        &self,
        credential: Credential,
    ) -> Result<Option<Self::UserData>, Self::Error> {
        let lock = acquire_lock(&self.pool);

        match lock {
            Some(mut lock) => {
                let id = lock
                    .iter()
                    .filter(|(_id, user)| credential.user_mail == user.user_mail)
                    .map(|(id, _user)| id.to_owned())
                    .next();

                let res = match id {
                    Some(id) => lock.remove(&id),
                    None => None,
                };
                Ok(res)
            }
            None => Err(AuthError::DbError),
        }
    }
}

#[async_trait]
pub trait AuthManage<N, C> {
    type UserData;
    type Error;

    async fn add_user(&self, new_user: N) -> Result<Self::UserData, Self::Error>;
    async fn verify_user(&self, credential: C) -> Result<Option<Self::UserData>, Self::Error>;
    async fn delete_user(&self, credential: C) -> Result<Option<Self::UserData>, Self::Error>;
}

#[cfg(test)]
mod test {
    use super::{AuthManage, Credential, UserDataDb};

    #[tokio::test]
    async fn add_and_verify_user() {
        let a = UserDataDb::new();
        let new_user = Credential {
            user_mail: "rustmail1234@gmail.com".to_string(),
            user_pass: "rustpass1234".to_string(),
        };

        a.add_user(new_user.clone()).await.unwrap();
        let user_data = a.verify_user(new_user.clone()).await.unwrap().unwrap();

        assert_eq!(user_data.user_mail, new_user.user_mail);
        assert_eq!(user_data.user_pass, new_user.user_pass);
    }

    #[tokio::test]
    async fn delete_user() {
        let db = UserDataDb::new();
        let credential = Credential {
            user_mail: "rustmail1234@gmail.com".to_string(),
            user_pass: "rustpass1234".to_string(),
        };

        db.add_user(credential.clone()).await.unwrap();
        let deleted_data = db.delete_user(credential.clone()).await.unwrap().unwrap();

        assert_eq!(deleted_data.user_mail, credential.user_mail);
        assert_eq!(deleted_data.user_pass, credential.user_pass);
    }
}
