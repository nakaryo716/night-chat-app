use std::{
    collections::HashMap,
    sync::{self, Arc},
};

use axum::async_trait;
use axum_session_manager::SessionManage;
use thiserror::Error;
use uuid::Uuid;

use crate::auth::UserData;

#[derive(Debug, Clone)]
pub struct SessionUserInfo {
    user_id: String,
}

impl SessionUserInfo {
    fn get_id(&self) -> &str {
        &self.user_id
    }
}

#[derive(Debug, Clone)]
pub struct SessionPool {
    pool: Arc<sync::Mutex<HashMap<String, SessionUserInfo>>>,
}

impl SessionPool {
    pub fn new() -> Self {
        Self {
            pool: Arc::default(),
        }
    }
}
#[derive(Debug, Error)]
pub enum SessionError {
    #[error("failed to parse")]
    ParseError,
    #[error("failed to query")]
    DbError,
    #[error("not found")]
    NotFound,
    #[error("unexpected error")]
    UnexpectedError,
}

#[async_trait]
impl<'a> SessionManage<UserData> for SessionPool {
    type SessionID = String;
    type UserInfo = SessionUserInfo;
    type Error = SessionError;

    async fn add_session(&self, session_data: UserData) -> Result<Self::SessionID, Self::Error> {
        let session_id = Uuid::new_v4().to_string();
        {
            let mut lock = self
                .pool
                .lock()
                .map_err(|_e| SessionError::UnexpectedError)?;

            lock.insert(
                session_id.clone(),
                SessionUserInfo {
                    user_id: session_data.get_user_id().to_owned(),
                },
            );
        }
        Ok(session_id)
    }

    async fn verify_session(
        &self,
        session_id: &str,
    ) -> Result<Option<Self::UserInfo>, Self::Error> {
        let lock = self
            .pool
            .lock()
            .map_err(|_| SessionError::UnexpectedError)?;
        match lock.get(session_id).map(|info| Some(info.to_owned())) {
            Some(info) => Ok(info),
            None => Err(SessionError::DbError),
        }
    }

    async fn delete_session(&self, session_id: &str) -> Result<(), Self::Error> {
        let mut lock = self
            .pool
            .lock()
            .map_err(|_| SessionError::UnexpectedError)?;
        match lock.remove(session_id) {
            Some(_) => Ok(()),
            None => Err(SessionError::NotFound),
        }
    }
}

#[cfg(test)]
mod test {
    use super::SessionPool;
    use crate::auth::{Credential, UserData};
    use axum_session_manager::SessionManage;

    const USER_MAIL: &str = "test_user_mail";
    const USER_PASS: &str = "test_user_pass";

    #[tokio::test]
    async fn add_and_get_session() {
        let db = SessionPool::new();

        let payload_user_data = UserData::new(Credential::new(
            USER_MAIL.to_string(),
            USER_PASS.to_string(),
        ));

        let session_id = db.add_session(payload_user_data.clone()).await.unwrap();

        let res = db.verify_session(&session_id).await.unwrap().unwrap();
        let res_user_id = res.get_id();

        assert_eq!(res_user_id, payload_user_data.get_user_id())
    }
}
