use mongodb::{bson::doc, options::ClientOptions, Client};

#[tauri::command]
async fn connect(connection_string: &str) -> Result<String, String> {
    let client_options = match ClientOptions::parse(connection_string).await {
        Ok(options) => options,
        Err(e) => return Err(format!("Failed to parse connection string: {}", e)),
    };
    let client = match Client::with_options(client_options) {
        Ok(client) => client,
        Err(e) => return Err(format!("Failed to create client: {}", e)),
    };

    // Ping the server to see if you can connect to the deployment.
    match client.database("admin").run_command(doc! {"ping": 1}).await {
        Ok(_) => Ok(format!(
            "Successfully connected to MongoDB with connection string: {}",
            connection_string
        )),
        Err(e) => Err(format!("Failed to connect to MongoDB: {}", e)),
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
