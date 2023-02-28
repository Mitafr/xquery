use std::borrow::Borrow;
use std::time::Duration;

use async_session::{Session, SessionStore};
use axum::http::StatusCode;
use axum::{debug_handler, Json};
use axum::{
    extract::State,
    http::HeaderValue,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::cookie::{Cookie, SignedCookieJar};
use axum_sessions::SameSite;
use hyper::header::COOKIE;
use hyper::Request;
use ldap3::{ldap_escape, Ldap, LdapConnAsync, Scope, SearchEntry};
use serde::{Deserialize, Serialize};

use crate::session::UserId;
use crate::session::UserIdFromSession;
use crate::AppState;

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginInfo {
    user: String,
    password: String,
}

pub struct LoginError {
    message: String,
    code: StatusCode,
}

impl LoginError {
    fn new(message: &str, code: StatusCode) -> Self {
        let message = String::from(message);
        tracing::error!("LoginError={}", message);
        Self { message, code }
    }
}

impl<E> From<E> for LoginError
where
    E: Into<anyhow::Error> + std::fmt::Display,
{
    fn from(e: E) -> Self {
        tracing::error!("{}", e.to_string());
        LoginError {
            message: String::from("Un problème est survenu"),
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for LoginError {
    fn into_response(self) -> Response {
        (self.code, self.message).into_response()
    }
}

pub async fn require_authentication<T>(
    jar: SignedCookieJar,
    user: UserIdFromSession,
    request: Request<T>,
    next: Next<T>,
) -> (SignedCookieJar, Response) {
    match user {
        UserIdFromSession::FoundUserId(u) => {
            tracing::debug!("{:#?}", u);
            (jar, next.run(request).await)
        }
        UserIdFromSession::NotFound() => {
            tracing::debug!("No user found");
            (
                jar.remove(Cookie::named("AXUM_SESSION_COOKIE_NAME")),
                Redirect::to("/login").into_response(),
            )
        }
    }
}

pub async fn logout_handler(
    user_id: UserIdFromSession,
    jar: SignedCookieJar,
    State(_state): State<AppState>,
) -> Result<(SignedCookieJar, impl IntoResponse), StatusCode> {
    match user_id {
        UserIdFromSession::FoundUserId(u) => {
            tracing::debug!("{:?}", u);
            // state.store.destroy_session();
        }
        UserIdFromSession::NotFound() => {
            tracing::debug!("NOT FOUND");
            //state.store.destroy_session(session);
        }
    }
    Ok((
        jar.remove(Cookie::named("AXUM_SESSION_COOKIE_NAME")),
        Redirect::to("/login").into_response(),
    ))
}

#[debug_handler]
pub async fn login_handler(
    jar: SignedCookieJar,
    State(state): State<AppState>,
    payload: Json<LoginInfo>,
) -> Result<(SignedCookieJar, impl IntoResponse), LoginError> {
    tracing::debug!("{:#?}", payload);
    let (conn, mut ldap) = LdapConnAsync::new("ldap://127.0.0.1:1389").await?;
    ldap3::drive!(conn);
    let rs = search(
        &mut ldap,
        "ou=users,dc=example,dc=org",
        "(&(objectClass=posixAccount)(uid={account}))",
        &payload.user,
    )
    .await;
    if rs.len() == 0 {
        return Err(LoginError::new("Compte introuvable", StatusCode::FORBIDDEN));
    }
    let res = ldap.simple_bind(&rs[0].dn, &payload.password).await;
    if res.is_err() {
        tracing::error!("LDAP error {}", res.err().unwrap());
        return Ok((jar, Redirect::to("/login").into_response()));
    }
    let code = match res.unwrap().rc {
        0 => StatusCode::OK,
        _ => StatusCode::UNAUTHORIZED,
    };
    ldap.unbind().await.unwrap();
    tracing::debug!("LDAP code={}", code);
    if code == StatusCode::OK {
        let mut response = Redirect::to("/").into_response();
        let user_id = UserId::new(payload.user.to_owned());
        let mut session = Session::new();
        session.expire_in(Duration::from_secs(86400));
        session.insert("user_id", user_id).unwrap();
        session.insert("user_role", "TEST").unwrap();
        let cookie_value = state.store.store_session(session).await?.unwrap();
        tracing::debug!(
            "Put session cookie ({}) in response",
            &cookie_value.as_str()
        );
        let mut new_cookie = Cookie::new("AXUM_SESSION_COOKIE_NAME", cookie_value);
        new_cookie.set_same_site(SameSite::Strict);
        new_cookie.set_http_only(true);
        new_cookie.set_max_age(Some(time::Duration::days(1)));
        response
            .headers_mut()
            .append(COOKIE, HeaderValue::from_str(&new_cookie.value()).unwrap());
        return Ok((jar.add(new_cookie), response));
    }
    Ok((jar, Redirect::to("/login").into_response()))
}

async fn search(
    connection: &mut Ldap,
    base: &str,
    filter: &str,
    account: &str,
) -> Vec<SearchEntry> {
    let a = ldap_escape(account);
    let search_filter = filter.replace("{account}", a.borrow());

    let s = connection
        .search(base, Scope::Subtree, &search_filter, [""])
        .await
        .unwrap();
    match s.success() {
        Ok(r) => r.0.into_iter().map(SearchEntry::construct).collect(),
        Err(e) => {
            tracing::error!("{}", e.to_string());
            Vec::new()
        }
    }
}
