// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod param;

use param::{Parameter, ParameterManager};

// tauriで引数に数値1つを取るexecute関数
#[tauri::command]
fn execute(num: i32) -> bool {
    println!("execute: {}", num);

    true
}

#[tauri::command]
fn save_parameter(param: Parameter) -> bool {
    println!("save_parameter: {:?}", param);
    true
    // let mut pm = ParameterManager::new().unwrap();
    // pm.set_parameter(param)
}

fn main() {
    // configの生成
    // let mut pm = ParameterManager::new().unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![execute])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
