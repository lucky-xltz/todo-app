mod database;

use database::{Database, CreateTodo, UpdateTodo};
use tauri::State;

#[tauri::command]
fn get_todos(db: State<'_, Database>) -> Result<Vec<database::Todo>, String> {
    db.get_all_todos().map_err(|e| e.to_string())
}

#[tauri::command]
fn create_todo(db: State<'_, Database>, todo: CreateTodo) -> Result<database::Todo, String> {
    db.create_todo(todo).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_todo(db: State<'_, Database>, id: String, updates: UpdateTodo) -> Result<database::Todo, String> {
    db.update_todo(&id, updates).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_todo(db: State<'_, Database>, id: String) -> Result<(), String> {
    db.delete_todo(&id).map_err(|e| e.to_string())
}

#[tauri::command]
fn toggle_todo(db: State<'_, Database>, id: String) -> Result<database::Todo, String> {
    db.toggle_todo(&id).map_err(|e| e.to_string())
}

#[tauri::command]
fn clear_completed(db: State<'_, Database>) -> Result<(), String> {
    db.clear_completed().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db = Database::new().expect("Failed to initialize database");
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            get_todos,
            create_todo,
            update_todo,
            delete_todo,
            toggle_todo,
            clear_completed
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}