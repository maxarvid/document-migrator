use mongodb::options::ClientOptions;

#[tauri::command]
async fn connect(connection_string: &str) -> Result<String, String> {
    match ClientOptions::parse(connection_string).await {
        Ok(client_options) => {
            println!("Successfully parsed connection string.");
            Ok(format!(
                "Successfully connected to MongoDB with options: {:?}",
                client_options
            ))
        }
        Err(e) => {
            Err(format!("Failed to parse connection string: {}", e))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![connect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
