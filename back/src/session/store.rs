use async_session::{async_trait, serde_json, Result, Session, SessionStore};
use axum::extract::FromRef;
use redis::{AsyncCommands, Client, IntoConnectionInfo, RedisResult};

use crate::AppState;

#[derive(Clone, Debug)]
pub struct RedisSessionStore {
    client: Client,
    prefix: Option<String>,
}

impl FromRef<AppState> for RedisSessionStore {
    fn from_ref(state: &AppState) -> Self {
        state.store.clone()
    }
}

impl RedisSessionStore {
    /// creates a redis store from an existing [`redis::Client`]
    /// ```rust
    /// # use async_redis_session::RedisSessionStore;
    /// let client = redis::Client::open("redis://127.0.0.1").unwrap();
    /// let store = RedisSessionStore::from_client(client);
    /// ```
    pub fn from_client(client: Client) -> Self {
        Self {
            client,
            prefix: None,
        }
    }

    /// creates a redis store from a [`redis::IntoConnectionInfo`]
    /// such as a [`String`], [`&str`](str), or [`Url`](../url/struct.Url.html)
    /// ```rust
    /// # use async_redis_session::RedisSessionStore;
    /// let store = RedisSessionStore::new("redis://127.0.0.1").unwrap();
    /// ```
    pub fn new(connection_info: impl IntoConnectionInfo) -> RedisResult<Self> {
        Ok(Self::from_client(Client::open(connection_info)?))
    }

    async fn connection(&self) -> RedisResult<redis::aio::Connection> {
        self.client.get_tokio_connection().await
    }

    fn prefix_key(&self, key: impl AsRef<str>) -> String {
        if let Some(ref prefix) = self.prefix {
            tracing::debug!("Prefix {}", prefix);
            format!("{}{}", prefix, key.as_ref())
        } else {
            tracing::debug!("Prefix {}", String::from(key.as_ref()));
            key.as_ref().into()
        }
    }

    async fn ids(&self) -> Result<Vec<String>> {
        Ok(self.connection().await?.keys(self.prefix_key("*")).await?)
    }
}

#[async_trait]
impl SessionStore for RedisSessionStore {
    async fn load_session(&self, cookie_value: String) -> Result<Option<Session>> {
        tracing::debug!("Loading a session from a cookie {}", cookie_value);
        let id = Session::id_from_cookie_value(&cookie_value)?;
        tracing::debug!("Cookie id decrypted {}", id);
        let mut connection = self.connection().await?;
        let record: Option<String> = connection.get(self.prefix_key(id)).await?;
        match record {
            Some(value) => Ok(serde_json::from_str(&value)?),
            None => Ok(None),
        }
    }
    async fn store_session(&self, session: Session) -> Result<Option<String>> {
        let id = self.prefix_key(session.id());
        tracing::debug!("Storing session cookie with id {}", id);
        let string = serde_json::to_string(&session)?;
        let mut connection = self.connection().await?;
        match session.expires_in() {
            None => connection.set(id, string).await?,
            Some(expiry) => {
                connection
                    .set_ex(id, string, expiry.as_secs() as usize)
                    .await?
            }
        };

        Ok(session.into_cookie_value())
    }

    async fn destroy_session(&self, session: Session) -> Result {
        let mut connection = self.connection().await?;
        let key = self.prefix_key(session.id().to_string());
        connection.del(key).await?;
        Ok(())
    }

    async fn clear_store(&self) -> Result {
        let mut connection = self.connection().await?;

        if self.prefix.is_none() {
            let _: () = redis::cmd("FLUSHDB").query_async(&mut connection).await?;
        } else {
            let ids = self.ids().await?;
            if !ids.is_empty() {
                connection.del(ids).await?;
            }
        }
        Ok(())
    }
}
