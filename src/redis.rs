use log::error;
use redis::{AsyncCommands};
use redis::{Client};
use crate::errors::{APIResult, AppError};

#[derive(Clone)]
pub struct RedisHolder {
    pub(crate) client: Client
}
impl RedisHolder {
    pub async fn get_session(&self) -> APIResult<RedisSession> {
        let conn = self.client.get_tokio_connection().await?;
        Ok(RedisSession::new(conn))
    }
}


pub struct RedisSession {
    conn: redis::aio::Connection
}

impl RedisSession {
    pub fn new(conn: redis::aio::Connection) -> Self {
        Self {
            conn
        }
    }
    
    pub async fn get(&mut self, key: &str, default_val: &str) -> String {
        if let Ok(val) = self.conn.get::<&str, String>(key).await {
            val
        } else {
            default_val.to_string()
        }
    }

    pub async fn set(&mut self, key: &str, val: &str, time_out: u32) {
        if let Err(err) = self.conn.set::<&str, &str, ()>(key, val).await {
            error!("set redis val with err:{err:?}");
        } else {
            if time_out > 0 {
                if let Err(err) = self.conn.expire::<&str, ()>(key, time_out as usize).await {
                    error!("set redis expire with err:{err:?}");
                }
            }
        }
    }


    pub async fn exists(&mut self, key: &str) -> bool {
        if let Ok(exists) = self.conn.exists::<&str, bool>(key).await {
            exists
        } else {
            false
        }
    }

    pub async fn set_add(&mut self, key: &str, member: &str) {
        if let Err(e) = self.conn.sadd::<&str, &str, ()>(key, member).await {
            error!("set redis sadd with err:{e:?}");
        }
    }


    pub async fn set_smembers(&mut self, key: &str) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        if let Ok(mut members) = self.conn.sscan::<&str, String>(key).await {
            while let Some(element) = members.next_item().await {
                result.push(element);
            }
        }
        result
    }

    pub async fn set_rm(&mut self, key: &str, member: &str) {
        if let Err(e) = self.conn.srem::<&str, &str, ()>(key, member).await {
            error!("set redis srem with err:{e:?}");
        }
    }


    pub async fn set_exists_member(&mut self, key: &str, member: &str) -> bool {
        if let Ok(rs) = self.conn.sismember::<&str, &str, bool>(key, member).await {
            rs
        } else {
            false
        }
    }

    pub async fn remove_key(&mut self, key: &str) {
        if let Err(e) = self.conn.del::<&str, ()>(key).await {
            error!("remove redis key with err:{e:?}");
        }
    }
}
