use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use thiserror::Error;
use uuid::Uuid;

use crate::utility::acquire_lock;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("query error")]
    DbError,
}

impl AuthManage<Credential, Credential> for UserDataDb {
    type UserData = UserData;
    type Error = AuthError;

    fn add_user(&self, new_user: Credential) -> Result<Self::UserData, Self::Error> {
        let user = UserData::new(new_user);
        match acquire_lock(&self.pool)
            .and_then(|mut lock| lock.insert(user.get_user_id().to_string(), user.clone()))
        {
            Some(_) => Ok(user),
            None => Ok(user),
        }
    }

    fn verify_user(&self, credential: Credential) -> Result<Option<Self::UserData>, Self::Error> {
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
    fn delete_user(&self, credential: Credential) -> Result<Option<Self::UserData>, Self::Error> {
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

pub trait AuthManage<N, C> {
    type UserData;
    type Error;

    fn add_user(&self, new_user: N) -> Result<Self::UserData, Self::Error>;
    fn verify_user(&self, credential: C) -> Result<Option<Self::UserData>, Self::Error>;
    fn delete_user(&self, credential: C) -> Result<Option<Self::UserData>, Self::Error>;
}

#[cfg(test)]
mod test {
    use super::{AuthManage, Credential, UserDataDb};

    #[test]
    fn add_and_verify() {
        let a = UserDataDb::new();
        let new_user = Credential {
            user_mail: "rustmail1234@gmail.com".to_string(),
            user_pass: "rustpass1234".to_string(),
        };

        a.add_user(new_user.clone()).unwrap();
        let user_data = a.verify_user(new_user.clone()).unwrap().unwrap();

        assert_eq!(user_data.user_mail, new_user.user_mail);
        assert_eq!(user_data.user_pass, new_user.user_pass);
    }

    #[test]
    fn delete() {
        let db = UserDataDb::new();
        let credential = Credential {
            user_mail: "rustmail1234@gmail.com".to_string(),
            user_pass: "rustpass1234".to_string(),
        };

        db.add_user(credential.clone()).unwrap();
        let deleted_data = db.delete_user(credential.clone()).unwrap().unwrap();

        assert_eq!(deleted_data.user_mail, credential.user_mail);
        assert_eq!(deleted_data.user_pass, credential.user_pass);
    }
}
