// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use param::Parameter;

mod param;
mod switchbot;

use switchbot::scene::{get_scene_list, post_scene, Scene};
use switchbot::device::{get_infrared_remote_list, InfraredRemote};
use param::{get_parameter, set_parameter};


// パラメータ関連
#[tauri::command]
async fn save_parameter(parameter: Parameter) -> bool {
    let err = set_parameter(parameter);

    match err {
        Ok(_) => true,
        Err(e) => {
            println!("error: {}", e);
            false
        }
    }
}

// シーン関連
#[tauri::command]
async fn execute_scene(scene_id: String) -> bool {

    // パラメータ読み込み
    let _param = get_parameter().unwrap();
    post_scene(&_param.token, &_param.secret, &scene_id).await
}

#[tauri::command]
async fn get_scenes() ->  Vec<Scene> {
    // パラメータ読み込み
    let _param = get_parameter().unwrap();

    // パラメータを使ってSwitchBotのAPIを叩く処理を書く
    let scenes = get_scene_list(&_param.token, &_param.secret).await.unwrap();

    scenes
}

// 無線デバイスの一覧を取得する関数
#[tauri::command]
async fn get_infrared_remotes() -> Vec<InfraredRemote> {
    // パラメータ読み込み
    let _param = get_parameter().unwrap();

    // パラメータを使ってSwitchBotのAPIを叩く処理を書く
    let devices = get_infrared_remote_list(&_param.token, &_param.secret).await.unwrap();

    devices
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_parameter, execute_scene, get_scenes, get_infrared_remotes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
