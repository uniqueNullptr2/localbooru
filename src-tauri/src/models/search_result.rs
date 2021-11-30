use std::{fmt::Display, path::Path, time::SystemTime};

use anyhow::Error;
use crypto::{digest::Digest, sha3::Sha3};
use image::{load_from_memory};
use img_hash::{HashAlg, HasherConfig};
use sqlx::{Pool, Sqlite};
use serde::Serialize;


#[derive(sqlx::FromRow, Serialize)]
pub struct SearchResult {
    pub id: u32,
    pub path: String,
    pub sha3: String,
    pub p_hash: String,
    pub search_id: i32,
    pub is_image: bool,
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.id, self.path)
    }
}


impl SearchResult {

    pub fn new<T: AsRef<Path>> (path: T) -> Self {
        Self{
            id: 0,
            path: path.as_ref().to_string_lossy().into_owned(),
            p_hash: "".to_owned(),
            sha3: "".to_owned(),
            search_id: 0,
            is_image: false,
        }
    }

    pub fn set_is_image(&mut self, is_image: bool) {
        self.is_image = is_image;
    }
    pub async fn calculate_hash(&mut self) -> Result<(), Error> {
        let buf = tokio::fs::read(&self.path).await?;
        // let s1 = SystemTime::now();
        let img = load_from_memory(&buf)?;
        // let s2 = SystemTime::now();
        let hasher = HasherConfig::new().hash_alg(HashAlg::DoubleGradient).hash_size(10, 8).preproc_dct().to_hasher();
        let h1 = hasher.hash_image(&img);
        self.p_hash = h1.to_base64();
        // let s3 = SystemTime::now();
        let mut sha_hasher = Sha3::sha3_256();
        sha_hasher.input(&buf);
        self.sha3 = sha_hasher.result_str();
        // let s4 = SystemTime::now();
        // println!("{}, {}, {}, {}", s2.duration_since(s1)?.as_secs_f32(), s3.duration_since(s2)?.as_secs_f32(), s4.duration_since(s3)?.as_secs_f32(),s4.duration_since(s1)?.as_secs_f32());
        Ok(())
    }

    pub async fn insert(&mut self, pool: &Pool<Sqlite>) -> Result<(), Error> {
        let res: (u32,) = sqlx::query_as("INSERT INTO SearchResult (path, p_hash, sha3, search_id, is_image) VALUES(?, ?, ?, ?, ?) Returning id;")
        .bind(&self.path)
        .bind(&self.p_hash)
        .bind(&self.sha3)
        .bind(self.search_id)
        .bind(self.is_image)
        .fetch_one(pool).await?;
        self.id = res.0;
        Ok(())
    }
    pub async fn get_by_search(pool: &Pool<Sqlite>, search_id: u32) -> Result<Vec<SearchResult>, Error> {
        let res = sqlx::query_as::<_, SearchResult>("SELECT id, path, p_hash, sha3, search_id, is_image FROM SearchResult WHERE search_id=?;")
            .bind(search_id)
            .fetch_all(pool).await?;
        Ok(res)
    }
}