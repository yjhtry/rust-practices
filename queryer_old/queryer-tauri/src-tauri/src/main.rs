// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::Value;

#[derive(serde::Serialize)]
struct QueryResult {
    data: serde_json::Value,
    error: Option<String>,
}

impl QueryResult {
    fn new(data: serde_json::Value, error: Option<String>) -> Self {
        Self { data, error }
    }
}

#[tauri::command]
fn query(sql: &str, output: &str) -> QueryResult {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut data = rt.block_on(async { queryer::query(sql).await.unwrap() });

    match output {
        "csv" => QueryResult::new(Value::String(data.to_csv().unwrap()), None),
        "json" => QueryResult::new(serde_json::to_value(data.to_json().unwrap()).unwrap(), None),
        v => QueryResult::new(
            Value::Null,
            Some(format!("Output type {} not supported", v)),
        ),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
