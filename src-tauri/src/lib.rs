// src-tauri/src/lib.rs (extrait)
// -- Documentation et commentaires mis à jour pour une meilleure lisibilité

pub mod projet; // Déclare le module 'projet'

use crate::projet::proj::{
    generate_random_prime, generate_rsa_keys, sign_message, fast_expo,
};

#[tauri::command]
fn generate_big_prime(max: u64) -> Result<u64, String> {
    generate_random_prime(max).ok_or_else(|| "Échec lors de la génération d'un grand nombre premier".to_string())
}

#[tauri::command]
fn generate_rsa_keys_command(max: u64) -> Result<((u64, u64), (u64, u64)), String> {
    generate_rsa_keys(max).ok_or_else(|| "Échec lors de la génération des clés RSA".to_string())
}

#[tauri::command]
fn sign_message_command(message: u64, private_n: u64, private_d: u64) -> u64 {
    sign_message(message, (private_n, private_d))
}

/// Commande pour décoder le message signé.
/// On calcule \( S^e \mod n \) pour récupérer M (si la signature est correcte).
#[tauri::command]
fn decode_message_command(signature: u64, public_n: u64, public_e: u64) -> u64 {
    fast_expo(signature, public_e, public_n)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_big_prime,
            generate_rsa_keys_command,
            sign_message_command,
            decode_message_command
        ])
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("Erreur lors de l'exécution de l'application Tauri");
}
