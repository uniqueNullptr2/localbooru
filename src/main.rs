#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::Result;
use async_channel::Receiver;
use futures::future::join_all;
use models::SearchResult;
use sqlx::{ Pool, Sqlite, sqlite::SqlitePoolOptions};
use anyhow::Error;
use tokio::fs::OpenOptions;
use walkdir::{DirEntry, WalkDir};
use tauri::{Manager};
mod db;
mod models;

#[tauri::command]
async fn get_search_results(state: tauri::State<'_, Pool<Sqlite>>) -> Result<Vec<SearchResult>, String>{
    println!("command executed yo");
    let pool = state.inner();
    let r = SearchResult::get_by_search(&pool, 0).await.map_err(|_| "Failed to get Results".to_owned())?;
    println!("{}", r.len());
    Ok(r)
}

#[tauri::command]
async fn search(pool: tauri::State<'_, Pool<Sqlite>>, path: String) -> Result<(), String>{
    let (s, r) = async_channel::unbounded();
    let mut v = vec!();
    for _ in 0..8 {
        let r2 = r.clone();
        let p2 = pool.inner().clone();
        v.push(tokio::spawn(async move {
            loop {
                if r2.is_closed() && r2.is_empty() {
                    break;
                }
                match handle_image(&r2, &p2).await {
                    Err(err) => println!("ERROR YO:  {}", err),
                    _ => ()
                }
            }
        }));
    }
    for entry in WalkDir::new(path){
        let e = entry.map_err(|_| "dir error".to_owned())?;
        if e.file_type().is_file() {
            s.send(e).await.map_err(|_| "send error".to_owned())?;
        }
    }
    s.close();
    join_all(v).await;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let mut path = tauri::api::path::home_dir().expect("dirs");
            path = path.join(".localbooru");
            tauri::async_runtime::block_on(tokio::fs::create_dir_all(&path)).expect("create dirs");
            path = path.join("localbooru.db");
            tauri::async_runtime::block_on(OpenOptions::new().create(true).write(true).open(&path)).expect("database path");
            let pool = tauri::async_runtime::block_on(SqlitePoolOptions::new().connect(path.to_str().expect("tgus is bs"))).expect("Sqlite");
            tauri::async_runtime::block_on(db::make_tables(&pool)).expect("failed to make tables");
            app.manage(pool);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_search_results, search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn is_image(e: &DirEntry) -> bool{
    let f_name = e.file_name();
    let l_name = f_name.to_ascii_lowercase();
    let name = l_name.to_string_lossy();
    println!("{}", name);
    name.ends_with(".png") || name.ends_with(".jpg") ||name.ends_with(".jpeg")
}

async fn handle_image (r: &Receiver<DirEntry>, pool: &Pool<Sqlite>) -> Result<(), Error> {
    let e = r.recv().await?;
    let mut sr = SearchResult::new(e.path());

    if is_image(&e) {
        sr.calculate_hash().await?;
        sr.set_is_image(true);
    }

    sr.insert(pool).await.expect("insert err");
    Ok(())
}
