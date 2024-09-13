use axum_extra::extract::{cookie::Cookie, CookieJar};
use serde::Deserialize;

pub const COOKIEKEY: &str = "user_name";

#[derive(Debug, Clone, Deserialize)]
pub struct UserNameForCoockie {
    user_name: String,
}

impl UserNameForCoockie {
    pub fn get_user_name(&self) -> &str {
        &self.user_name
    }
}

pub fn add_cookie_to_jar(user_name: String, jar: CookieJar) -> CookieJar {
    let cookie = Cookie::new(COOKIEKEY, user_name);
    jar.add(cookie)
}

pub fn get_user_name_from_cookie(jar: CookieJar) -> Option<String> {
    jar.get(COOKIEKEY).map(|cookie| cookie.value().to_string())
}

pub fn delete_cookie_from_jar(jar: CookieJar) -> CookieJar {
    jar.remove(Cookie::from(COOKIEKEY))
}
