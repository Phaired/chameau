// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn generate_big_prime(max: u64) -> Option<u64> {
    generate_random_prime(max)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
