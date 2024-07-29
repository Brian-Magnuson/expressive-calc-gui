// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use expressive_calc::Calculator;
use std::sync::Mutex;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(serde::Deserialize)]
struct StringPayload {
    value: String,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let calculator = Mutex::new(Calculator::new());

            app.listen_global("evaluate", move |event| {
                // The event message is a JSON string
                let message = event.payload().unwrap();
                // Deserialize the JSON string into a struct
                let payload: StringPayload = serde_json::from_str(&message).unwrap();

                // Lock the calculator and evaluate the expression
                let result = match calculator.lock().unwrap().evaluate(&payload.value) {
                    Ok((name, value)) => format!("{} = {}", name, value),
                    Err(err) => err.to_string(),
                };
                // Calculator lock is released here

                // Send the result back to the front-end
                window.emit("result", result).unwrap();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
