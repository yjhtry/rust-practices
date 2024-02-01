// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn query(sql: &str, output: &str) -> String {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut data = rt.block_on(async { queryer::query(sql).await.unwrap() });

    match output {
        "csv" => data.to_csv().unwrap(),
        v => format!("Output type {} not supported", v),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
