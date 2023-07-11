// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use param::Parameter;

mod param;
mod switchbot;

use switchbot::scene::{get_scene_list, post_scene, Scene};
use switchbot::device::{
    get_infrared_remote_list,
    InfraredRemote,
    infrated_airconfitioner_command,
    infrated_remote_status_switch,
    AirCondFanSpeedSettingValue,
    AirCondModeSettingValue
};
use param::{get_parameter, set_parameter};


// パラメータ関連
#[tauri::command(rename_all = "snake_case")]
async fn save_parameter(parameter: Parameter) -> Result<(), String> {
    let err = set_parameter(parameter);
    match err {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("error: {}", e);
            Err(e.to_string())
        }
    }
}

// シーン関連
#[tauri::command(rename_all = "snake_case")]
async fn execute_scene(scene_id: String) -> Result<(), String> {
    // パラメータ読み込み
    let _param = get_parameter().unwrap();
    let res = post_scene(&_param.token, &_param.secret, &scene_id).await;
    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("error: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
async fn get_scenes() ->  Result<Vec<Scene>, String> {
    // パラメータ読み込み
    let _param = get_parameter().unwrap();
    let scenes = get_scene_list(&_param.token, &_param.secret).await;
    match scenes {
        Ok(scenes) => Ok(scenes),
        Err(e) => {
            println!("error: {}", e);
            Err(e.to_string())
        }
    }
}

// 無線デバイスの一覧を取得する関数
#[tauri::command]
async fn get_infrared_remotes() -> Result<Vec<InfraredRemote>, String> {
    // パラメータ読み込み
    let _param = get_parameter().unwrap();
    // パラメータを使ってSwitchBotのAPIを叩く処理を書く
    let devices = get_infrared_remote_list(&_param.token, &_param.secret).await;
    match devices {
        Ok(devices) => Ok(devices),
        Err(e) => {
            println!("error: {}", e);
            Err(e.to_string())
        }
    }
}

// 無線デバイスのオンオフ
#[tauri::command(rename_all = "snake_case")]
async fn execute_infrated_remote_enable(device_id: String, enable: bool) -> Result<(), String> {
    // パラメータ読み込み
    let _param = get_parameter().unwrap();
    // パラメータを使ってSwitchBotのAPIを叩く処理を書く
    let res = infrated_remote_status_switch(&_param.token, &_param.secret, &device_id, enable).await;
    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("error: {}", e);
            Err(e.to_string())
        }
    }
}

// エアコンの操作
#[tauri::command(rename_all = "snake_case")]
async fn execute_infrated_airconfitioner_command(device_id: &str, temperature: u16, mode: AirCondModeSettingValue, fan_speed: AirCondFanSpeedSettingValue, power_state: bool) -> Result<(), String> {
    // パラメータ読み込み
    let _param = get_parameter().unwrap();
    // パラメータを使ってSwitchBotのAPIを叩く処理を書く
    let res = infrated_airconfitioner_command(&_param.token, &_param.secret, device_id, temperature, mode, fan_speed, power_state).await;
    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("error: {}", e);
            Err(e.to_string())
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_parameter, execute_scene, get_scenes, get_infrared_remotes, execute_infrated_remote_enable, execute_infrated_airconfitioner_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
