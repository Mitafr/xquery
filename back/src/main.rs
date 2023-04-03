extern crate tera;
#[macro_use]
extern crate lazy_static;
use auth::handlers::{login_handler, logout_handler};
use auth::middleware::require_authentication;
use axum::extract::Host;
use axum::http::HeaderValue;
use axum::middleware;
use axum::response::{Html, Redirect};
use axum::routing::get_service;
use axum::{
    extract::FromRef,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::Key;
use axum_server::tls_rustls::RustlsConfig;
use controller::issued_date_get;
use db::init_db;
use hyper::{header, Uri};
use sea_orm::DatabaseConnection;
use session::store::RedisSessionStore;
use session::UserIdFromSession;
use tera::{Context, Tera};
use tower::ServiceBuilder;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;
use tracing::{debug, info};
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use axum::handler::HandlerWithoutStateExt;
use dotenvy::dotenv;

use std::net::SocketAddr;
use std::path::{Path, PathBuf};

mod auth;
mod controller;
mod db;
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
        tera
    };
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let http_addr = SocketAddr::from((
        [0, 0, 0, 0],
        dotenvy::var("HTTP_PORT")
            .unwrap_or("8080".to_string())
            .parse()
            .unwrap(),
    ));
    let https_addr = SocketAddr::from((
        [0, 0, 0, 0],
        dotenvy::var("HTTPS_PORT")
            .unwrap_or("443".to_string())
            .parse()
            .unwrap(),
    ));
    let config = RustlsConfig::from_pem_file(
        PathBuf::from(std::env::current_dir().unwrap())
            .join("certs")
            .join("cert.pem"),
        PathBuf::from(std::env::current_dir().unwrap())
            .join("certs")
            .join("key.pem"),
    )
    .await
    .unwrap();
    let app = app().await.into_make_service();
    setup_logging();
    info!("Listening on HTTP_PORT {}", http_addr);
    info!("Listening on HTTPS_PORT {}", https_addr);
    debug!("listening on {}", https_addr);
    tokio::spawn(redirect_http(http_addr.clone(), https_addr.clone()));
    axum_server::bind_rustls(https_addr, config)
        .serve(app)
        .await
        .unwrap();
}

async fn redirect_http(http_addr: SocketAddr, https_addr: SocketAddr) {
    let redirect = move |Host(host): Host, uri: Uri| async move {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);
        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }
        let https_host = host.replace(
            &http_addr.port().to_string(),
            &https_addr.port().to_string(),
        );
        parts.authority = Some(https_host.parse().unwrap());

        let uri = Uri::from_parts(parts).unwrap();

        Redirect::permanent(&uri.to_string())
    };

    axum::Server::bind(&http_addr)
        .serve(redirect.into_make_service())
        .await
        .unwrap();
}

#[derive(Clone)]
pub struct AppState {
    store: RedisSessionStore,
    key: Key,
    db: DatabaseConnection,
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
                .unwrap_or_else(|_| "w=info,tower_http=info".into()),
        )
        .with(
            fmt::Layer::new()
                .with_writer(file_appender)
                .with_ansi(false),
        )
        .with(fmt::layer())
        .init();
}

async fn app() -> Router {
    let url = dotenvy::var("REDIS").unwrap();
    let state = AppState {
        store: RedisSessionStore::new(url).unwrap(),
        key: Key::from(
            "4t7w!z%C*F-JaNdRgUkXp2r5u8x/A?D(G+KbPeShVmYq3t6v9y$B&E)H@McQfTjWnZr4u7x!z%C*F-JaNdRgUkXp2s5v8y/B?D(G+KbPeShVmYq3t6w9z$C&F)H@McQfTjWnZr4u7x!A%D*G-KaNdRgUkXp2s5v8y/B?E(H+MbQeShVmYq3t6w9z$C&F)J@NcRfUjWnZr4u7x!A%D*G-KaPdSgVkYp2s5v8y/B?E(H+MbQeThWmZq4t6w9z$C&F)".as_bytes(),
        ),
        db: init_db().await.unwrap()
    };
    Router::new()
        .route("/api/dateofissue", get(issued_date_get))
        .route("/", get(index_handler))
        .route("/stats", get(index_handler))
        .route("/map", get(index_handler))
        .route("/query", get(index_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_authentication,
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
            ),
        )
        .nest_service(
            "/favicon.ico",
            get_service(ServeFile::new("dist/favicon.ico")),
        )
        .nest_service(
            "/robots.txt",
            get_service(ServeFile::new("dist/robots.txt")),
        )
        .nest_service(
            "/unauthorized",
            get_service(ServeFile::new("dist/errors/403.html")),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(setup_cors())
                .layer(SetResponseHeaderLayer::if_not_present(
                    header::X_CONTENT_TYPE_OPTIONS,
                    HeaderValue::from_static("nosniff"),
                ))
                .layer(SetResponseHeaderLayer::if_not_present(
                    header::REFERRER_POLICY,
                    HeaderValue::from_static("strict-origin-when-cross-origin"),
                ))
                .layer(SetResponseHeaderLayer::if_not_present(
                    header::X_FRAME_OPTIONS,
                    HeaderValue::from_static("SAMEORIGIN"),
                ))
                .layer(SetResponseHeaderLayer::if_not_present(
                    header::CONTENT_SECURITY_POLICY,
                    HeaderValue::from_static("default-src 'self' https; script-src 'self' https  'unsafe-inline'; style-src 'self' https 'unsafe-inline'; font-src 'self'; img-src 'self' https://*.tile.openstreetmap.org; frame-src 'self'"),
                ))
                .layer(SetResponseHeaderLayer::if_not_present(
                    header::STRICT_TRANSPORT_SECURITY,
                    HeaderValue::from_static("max-age=31536000; includeSubDomains"),
                ))
                .layer(SetResponseHeaderLayer::if_not_present(
                    header::X_XSS_PROTECTION,
                    HeaderValue::from_static("1; mode=block"),
                )),
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
