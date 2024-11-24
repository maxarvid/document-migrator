// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn connect(connection_string: &str) -> Result<String, String> {
    // Here you can add your logic to connect to MongoDB using the connection string
    // For now, we'll just return a success message
    Ok(format!(
        "Successfully connected to MongoDB with connection string: {}",
        connection_string
    ))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![connect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
