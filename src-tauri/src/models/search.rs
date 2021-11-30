use std::{path::Path};

use anyhow::Error;
use sqlx::{Pool, Sqlite};
use serde::Serialize;


#[derive(Serialize)]
pub struct Search {
    id: u32,
    base_path: String,
    user: String
}

impl Search {

    pub fn new<T: AsRef<Path>> (path: T, user: String) -> Self {
        Self{
            id: 0,
            base_path: path.as_ref().to_string_lossy().into_owned(),
            user
        }
    }

    pub async fn insert(&mut self, pool: &Pool<Sqlite>) -> Result<(), Error> {
        let res: (u32,) = sqlx::query_as("INSERT INTO Search (base_path, user) VALUES(?, ?) Returning id;")
        .bind(&self.base_path)
        .bind(&self.user)
        .fetch_one(pool).await?;
        self.id = res.0;
        Ok(())
    }
}