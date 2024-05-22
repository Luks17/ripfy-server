use redis::{aio::MultiplexedConnection, AsyncCommands};

use crate::{util::error::RedisError, AppState};

pub struct RedisConnection {
    con: MultiplexedConnection,
}

impl RedisConnection {
    pub async fn from_app_state(state: &AppState) -> Result<Self, RedisError> {
        let client = &state.redis_client;

        tracing::debug!("Connecting to redis...");

        Ok(Self {
            con: client
                .get_multiplexed_tokio_connection()
                .await
                .map_err(|_| RedisError::RedisConnFailed)?,
        })
    }

    pub async fn get(&mut self, key: String) -> Result<String, RedisError> {
        self.con
            .get(key)
            .await
            .map_err(|e| RedisError::RedisQueryFailed(e.to_string()))
    }

    pub async fn getdel(&mut self, key: String) -> Result<String, RedisError> {
        self.con
            .get_del(key)
            .await
            .map_err(|e| RedisError::RedisQueryFailed(e.to_string()))
    }

    pub async fn set(&mut self, key: String, value: String) -> Result<(), RedisError> {
        self.con
            .set(key, value)
            .await
            .map_err(|e| RedisError::RedisQueryFailed(e.to_string()))?;

        Ok(())
    }

    pub async fn setex(&mut self, key: String, value: String, exp: u64) -> Result<(), RedisError> {
        self.con
            .set_ex(key, value, exp)
            .await
            .map_err(|e| RedisError::RedisQueryFailed(e.to_string()))?;

        Ok(())
    }
}
