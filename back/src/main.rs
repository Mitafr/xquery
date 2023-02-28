#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;
use auth::{login_handler, logout_handler};
use axum::http::HeaderValue;
use axum::middleware;
use axum::response::{Html, Redirect};
use axum::routing::get_service;
use axum::{
    extract::FromRef,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::Key;
use hyper::header;
use serde_json::{to_value, Value};
use session::store::RedisSessionStore;
use session::UserIdFromSession;
use tera::{Context, Result, Tera};
use tower::ServiceBuilder;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;

mod auth;
mod session;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("dist/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html", ".sql", ".ico"]);
        tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}

pub fn do_nothing_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("do_nothing_filter", "value", String, value);
    Ok(to_value(&s).unwrap())
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::debug!("listening on {}", addr);
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

#[derive(Clone)]
pub struct AppState {
    store: RedisSessionStore,
    key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

fn setup_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any())
}

fn setup_logging() {
    let file_appender = tracing_appender::rolling::daily(Path::new("./logs"), Path::new("w"));
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "w=debug,tower_http=debug".into()),
        )
        .with(
            fmt::Layer::new()
                .with_writer(file_appender)
                .with_ansi(false),
        )
        .with(fmt::layer())
        .init();
}

fn app() -> Router {
    setup_logging();
    let state = AppState {
        store: RedisSessionStore::new("redis://redis:7000").unwrap(),
        key: Key::from(
            "4t7w!z%C*F-JaNdRgUkXp2r5u8x/A?D(G+KbPeShVmYq3t6v9y$B&E)H@McQfTjWnZr4u7x!z%C*F-JaNdRgUkXp2s5v8y/B?D(G+KbPeShVmYq3t6w9z$C&F)H@McQfTjWnZr4u7x!A%D*G-KaNdRgUkXp2s5v8y/B?E(H+MbQeShVmYq3t6w9z$C&F)J@NcRfUjWnZr4u7x!A%D*G-KaPdSgVkYp2s5v8y/B?E(H+MbQeThWmZq4t6w9z$C&F)".as_bytes(),
        ),
    };

    Router::new()
        .route("/", get(index_handler))
        .route("/stats", get(index_handler))
        .route("/map", get(index_handler))
        .route("/query", get(index_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth::require_authentication,
        ))
        .route("/logout", get(logout_handler))
        .route("/login", post(login_handler))
        .route("/login", get(index_handler))
        .nest_service(
            "/assets",
            get_service(
                ServiceBuilder::new()
                    .layer(SetResponseHeaderLayer::if_not_present(
                        header::CACHE_CONTROL,
                        HeaderValue::from_static("max-age=31536000"),
                    ))
                    .service(ServeDir::new("dist/assets").precompressed_gzip()),
            )
            .handle_error(handle_error),
        )
        .nest_service(
            "/favicon.ico",
            get_service(ServeFile::new("dist/favicon.ico")).handle_error(handle_error),
        )
        .nest_service(
            "/unauthorized",
            get_service(ServeFile::new("dist/errors/403.html")).handle_error(handle_error),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(setup_cors()),
        )
        .fallback(handler_404)
        .with_state(state)
}

async fn handler_404() -> impl IntoResponse {
    Redirect::permanent("/").into_response()
}

async fn index_handler(user_id: UserIdFromSession) -> impl IntoResponse {
    let mut context = Context::new();
    match user_id {
        UserIdFromSession::FoundUserId(user) => {
            context.insert("authenticated", "true");
            context.insert("username", &user.username);
        }
        UserIdFromSession::NotFound() => {
            context.insert("authenticated", "false");
        }
    }
    Html(TEMPLATES.render("index.html", &context).unwrap())
}
