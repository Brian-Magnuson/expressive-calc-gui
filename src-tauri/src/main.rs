// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use expressive_calc::Calculator;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(serde::Deserialize)]
struct ExpressionPayload {
    expression: String,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let calculator = Mutex::new(Calculator::new());

            app.listen_global("evaluate", move |event| {
                let expr = event.payload().unwrap();
                let payload: ExpressionPayload = serde_json::from_str(expr).unwrap();
                let result = match calculator.lock().unwrap().evaluate(&payload.expression) {
                    Ok((name, value)) => format!("{} = {}", name, value),
                    Err(err) => err.to_string(),
                };

                println!("Result: {}", result);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
