// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[derive(serde::Serialize)]
enum OutputType {
    CSV(String),
    JSON(serde_json::Value),
}

#[tauri::command]
fn query(sql: &str, output: &str) -> OutputType {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut data = rt.block_on(async { queryer::query(sql).await.unwrap() });

    match output {
        "csv" => OutputType::CSV(data.to_csv().unwrap()),
        "json" => OutputType::JSON(data.to_json().unwrap()),
        v => OutputType::CSV(format!("Output type {} not supported", v)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
