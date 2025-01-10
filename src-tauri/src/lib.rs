// src-tauri/src/lib.rs

pub mod projet; // Declare the 'projet' module

use crate::projet::proj::generate_random_prime; // Import the function from your module

#[tauri::command]
fn generate_big_prime(max: u64) -> Result<u64, String> {
    generate_random_prime(max).ok_or_else(|| "Failed to generate a big prime".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_big_prime]) // Register the command
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
