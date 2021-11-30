use anyhow::Error;
use sqlx::{Pool, Sqlite};

pub async fn make_tables(pool: &Pool<Sqlite>) -> Result<(), Error> {
    sqlx::query("CREATE TABLE IF NOT EXISTS Image(id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, sha3 TEXT NOT NULL, p_hash TEXT NOT NULL);").execute(pool).await?;
    sqlx::query("CREATE TABLE IF NOT EXISTS Tag(id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL);").execute(pool).await?;
    sqlx::query("CREATE TABLE IF NOT EXISTS Tag_Image(tag_id INTEGER NOT NULL, image_id INTEGER NOT NULL, PRIMARY KEY(tag_id, image_id), FOREIGN KEY(tag_id) REFERENCES Tag(id), FOREIGN KEY(image_id) REFERENCES Image(id));").execute(pool).await?;
    sqlx::query("CREATE TABLE IF NOT EXISTS Search(id INTEGER PRIMARY KEY AUTOINCREMENT, base_path TEXT NOT NULL, user TEXT NOT NULL);").execute(pool).await?;
    sqlx::query("CREATE TABLE IF NOT EXISTS SearchResult(id INTEGER PRIMARY KEY AUTOINCREMENT, path TEXT NOT NULL, sha3 TEXT NOT NULL, p_hash TEXT NOT NULL,is_image BOOLEAN, search_id INTEGER, FOREIGN KEY (search_id) REFERENCES Search(id));").execute(pool).await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS search_result_search_id ON SearchResult(search_id);").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS image_sha3 ON Image(sha3);").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS image_p_hash ON Image(p_hash);").execute(pool).await?;
    Ok(())
}

