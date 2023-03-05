pub mod store;

use async_session::SessionStore;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, HeaderValue},
};
use axum_extra::extract::{cookie::Key, SignedCookieJar};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::session::store::RedisSessionStore;

#[derive(Debug)]
pub struct FreshUserId {
    pub user_id: UserId,
    pub cookie: HeaderValue,
}

#[derive(Debug)]
pub enum UserIdFromSession {
    FoundUserId(UserId),
    NotFound(),
}

#[async_trait]
impl<S> FromRequestParts<S> for UserIdFromSession
where
    RedisSessionStore: FromRef<S>,
    Key: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let signed_cookie = SignedCookieJar::from_headers(&parts.headers, Key::from_ref(state));
        let session_cookie = signed_cookie.get("SID");

        if session_cookie.is_none() {
            return Ok(Self::NotFound());
        }

        tracing::debug!(
            "UserIdFromSession: got session cookie from user agent, {}={}",
            "SID",
            session_cookie.as_ref().unwrap()
        );
        match RedisSessionStore::from_ref(state)
            .load_session(session_cookie.as_ref().unwrap().value().to_string())
            .await
        {
            Ok(result) => {
                // continue to decode the session cookie
                let user_id = if let Some(session) = result {
                    if let Some(user_id) = session.get::<UserId>("user_id") {
                        tracing::debug!(
                            "UserIdFromSession: session decoded success, user_id={:?}",
                            user_id
                        );
                        user_id
                    } else {
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "No `user_id` found in session",
                        ));
                    }
                } else {
                    tracing::debug!(
                        "UserIdFromSession: err session not exists in store, {}={}",
                        "SID",
                        session_cookie.as_ref().unwrap()
                    );
                    return Ok(Self::NotFound());
                };
                return Ok(Self::FoundUserId(user_id));
            }
            Err(e) => {
                tracing::error!("No Session attached to this Cookie \n {:#?}", e);
                return Ok(Self::NotFound());
                // return Err((
                //     StatusCode::UNAUTHORIZED,
                //     "No Session attached to this Cookie",
                // ));
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserId {
    pub user_id: Uuid,
    pub username: String,
}

impl UserId {
    pub fn new(username: String) -> Self {
        Self {
            user_id: Uuid::new_v4(),
            username,
        }
    }
}
