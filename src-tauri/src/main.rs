// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[derive(Debug, Serialize)]
struct Person<'a> {
    name: &'a str,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() {
    // Connect to the database
    DB.connect::<Ws>("127.0.0.1:8000").await.unwrap();
    // Select a namespace + database
    DB.use_ns("surreal").use_db("surreal").await.unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, init_db, query_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
async fn init_db(name: &str) -> surrealdb::Result<()> {
    // Create or update a specific record
    let _: Option<Record> = DB
        .update(("person", name))
        .content(Person { name })
        .await
        .unwrap();
    Ok(())
}

#[tauri::command]
async fn query_db() -> String {
    let result = DB
        .query("SELECT * FROM person")
        .bind(("table", "person"))
        .await
        .unwrap();
    // Return the result as string
    format!("Query result: {:?}", result)
}
