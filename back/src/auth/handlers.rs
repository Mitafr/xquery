use std::time::Duration;

use async_session::{Session, SessionStore};
use axum::{
    debug_handler,
    extract::State,
    http::HeaderValue,
    response::{IntoResponse, Redirect},
    Json,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    SignedCookieJar,
};
use hyper::{header::COOKIE, StatusCode};
use ldap3::LdapConnAsync;
use serde::{Deserialize, Serialize};

use crate::{
    auth::{backend::ldap::search, errors::LoginError},
    session::{UserId, UserIdFromSession},
    AppState,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginInfo {
    user: String,
    password: String,
}

pub async fn logout_handler(
    user_id: UserIdFromSession,
    jar: SignedCookieJar,
    State(_state): State<AppState>,
) -> Result<(SignedCookieJar, impl IntoResponse), StatusCode> {
    match user_id {
        UserIdFromSession::FoundUserId(_u) => {
            // state.store.destroy_session();
        }
        UserIdFromSession::NotFound() => {
            //state.store.destroy_session(session);
        }
    }
    Ok((
        jar.remove(Cookie::named("SID")),
        Redirect::to("/login").into_response(),
    ))
}

#[debug_handler]
pub async fn login_handler(
    jar: SignedCookieJar,
    State(state): State<AppState>,
    payload: Json<LoginInfo>,
) -> Result<(SignedCookieJar, impl IntoResponse), LoginError> {
    let url = dotenvy::var("LDAP").unwrap();
    let (conn, mut ldap) = LdapConnAsync::new(&url).await?;
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
        let mut new_cookie = Cookie::new("SID", cookie_value);
        new_cookie.set_same_site(Some(SameSite::Strict));
        new_cookie.set_http_only(true);
        new_cookie.set_max_age(Some(time::Duration::days(1)));
        response
            .headers_mut()
            .append(COOKIE, HeaderValue::from_str(&new_cookie.value()).unwrap());
        return Ok((jar.add(new_cookie), response));
    }
    Ok((jar, Redirect::to("/login").into_response()))
}
