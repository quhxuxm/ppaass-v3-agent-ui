mod vo;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn start_agent(configuration: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn stop_agent(){

}

#[tauri::command]
fn import_users(){}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_agent, stop_agent, import_users])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
