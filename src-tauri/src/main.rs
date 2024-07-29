// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use expressive_calc::Calculator;
use std::sync::Mutex;
use tauri::Manager;

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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
