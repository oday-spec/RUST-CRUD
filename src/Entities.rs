use sqlx::{self, prelude::FromRow};
use serde::{self, Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, FromRow)]
pub struct Post {
    pub id : i32,
    pub title: String,
}
