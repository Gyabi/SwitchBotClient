// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod param;
mod switchbot;

use param::{Parameter, get_parameter, set_parameter};
use switchbot::get_device_list;

// tauriで引数に数値1つを取るexecute関数
#[tauri::command]
async fn execute(num: i32) -> bool {
    println!("execute: {}", num);

    // パラメータ読み込み
    let _param = get_parameter().unwrap();

    // パラメータを使ってSwitchBotのAPIを叩く処理を書く
    let _devices = get_device_list(&_param.token, &_param.secret).await.unwrap();

    true
}

// パラメータ保存
#[tauri::command]
fn save_parameter(param: Parameter) -> bool {
    println!("save_parameter: {:?}", param);
    let err = set_parameter(param);

    match err {
        Ok(_) => true,
        Err(e) => {
            println!("error: {}", e);
            false
        }
    }
}

fn main() {
    println!("Hello, world!");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![execute, save_parameter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
