// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use expressive_calc::Calculator;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
fn evaluate_expression(expression: &str, calculator: State<Mutex<Calculator>>) -> String {
    let mut calc = calculator.lock().unwrap();
    match calc.evaluate(expression) {
        Ok((name, value)) => format!("{} = {}", name, value),
        Err(err) => err.to_string(),
    }
}

#[tauri::command]
fn reset_calculator(calculator: State<Mutex<Calculator>>) {
    let mut calc = calculator.lock().unwrap();
    calc.reset();
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(Calculator::new()))
        .invoke_handler(tauri::generate_handler![
            evaluate_expression,
            reset_calculator
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
