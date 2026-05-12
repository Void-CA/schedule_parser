use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::domain::models::Class;

/// Carga {año: [clases]} — formato original sin wrapper
pub fn load_schedule(path: &str) -> Result<HashMap<String, Vec<Class>>, String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

/// Persiste de vuelta al archivo original
pub fn save_schedule(path: &str, data: &HashMap<String, Vec<Class>>) -> Result<(), String> {
    save_json(path, data)
}

/// Guarda cualquier objeto serializable a JSON
pub fn save_json<T: serde::Serialize>(path: &str, data: &T) -> Result<(), String> {
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    let mut file = File::create(path).map_err(|e| e.to_string())?;
    file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}