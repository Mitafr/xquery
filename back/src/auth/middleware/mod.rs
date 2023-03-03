use axum::{
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::{cookie::Cookie, SignedCookieJar};
use hyper::Request;

use crate::session::UserIdFromSession;

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
