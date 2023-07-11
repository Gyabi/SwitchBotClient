use super::utils::{create_header, SWITCH_BOT_API_URL};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetDeviceResponse {
    
    status_code: u16,
    body: GetDeviceResponseBody,
    message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct GetDeviceResponseBody {
    device_list: Vec<Device>,
    infrared_remote_list: Vec<InfraredRemote>,
    
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Device {
    device_id: String,
    device_name: String,
    device_type: String,
    enable_cloud_service: Option<bool>,
    hub_device_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InfraredRemote {
    device_id : String,
    device_name : String,
    remote_type : String,
    hub_device_id : String
}

// 無線デバイスのリストを出力する関数
pub(crate) async fn get_infrared_remote_list(token: &str, secret:&str) -> Result<Vec<InfraredRemote>, reqwest::Error> {
    // reqwestクレートを用いてAPIを叩く
    let url = format!("{}/devices", SWITCH_BOT_API_URL);
    let client = reqwest::Client::new();

    // ヘッダーを作成
    let headers = create_header(token, secret);

    // APIを叩く
    let res = client.get(url).headers(headers).send().await.expect("Failed to get infrared remote list");

    // レスポンスからbodyからinfraredRemoteListをInfraredRemoteのリストとして取り出す
    let get_device_response: GetDeviceResponse = res.json().await.expect("Failed to parse get infrared remote list response");

    // 返却
    Ok(get_device_response.body.infrared_remote_list)
}

// 無線デバイスのオンオフ 戻り値はResultでOkなら中身はなし、Errorならreqwest::Error
// ボディにはcommandType,Command,commandparameterを入れる
pub(crate) async fn infrated_remote_status_switch(token: &str, secret:&str, device_id: &str, enable: bool) -> Result<(), reqwest::Error>{
    // reqwestクレートを用いてAPIを叩く
    let url = format!("{}/devices/{}/commands", SWITCH_BOT_API_URL, device_id);
    let client = reqwest::Client::new();

    // ヘッダーを作成
    let headers = create_header(token, secret);

    // ボディを作成
    let body = format!("{{\"commandType\":\"command\",\"command\":\"{}\",\"parameter\":\"default\"}}", if enable { "turnOn" } else { "turnOff" });

    // APIを叩く
    let res = client.post(url).headers(headers).body(body).send().await.expect("Failed to switch infrared remote status");

    // ステータスコードに応じてboolを返す
    let status_code = res.status().as_u16();
    if status_code == 200 {
        Ok(())
    } else {
        Err(res.error_for_status().unwrap_err())
    }
}

// エアコンの設定関連
// #[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub(crate) enum AirCondModeSettingValue {
    #[serde(rename = "1")]
    AUTO,
    #[serde(rename = "2")]
    COOL,
    #[serde(rename = "3")]
    DRY,
    #[serde(rename = "4")]
    FAN,
    #[serde(rename = "5")]
    HEAT
}

// #[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub(crate) enum AirCondFanSpeedSettingValue {
    #[serde(rename = "1")]
    AUTO,
    #[serde(rename = "2")]
    LOW,
    #[serde(rename = "3")]
    MEDIUM,
    #[serde(rename = "4")]
    HIGH
}
// エアコンの設定値を反映させる
// ボディにはcommandType,Command,commandparameterを入れる
pub(crate) async fn infrated_airconfitioner_command(token: &str, secret:&str, device_id: &str, temperature: u16, mode: AirCondModeSettingValue, fan_speed: AirCondFanSpeedSettingValue, power_state: bool) -> Result<(), reqwest::Error> {
    // reqwestクレートを用いてAPIを叩く
    let url = format!("{}/devices/{}/commands", SWITCH_BOT_API_URL, device_id);
    let client = reqwest::Client::new();

    // ヘッダーを作成
    let headers = create_header(token, secret);

    // ボディを作成
    let _parameter = format!("{},{},{},{}",
        temperature, 
        match mode {
            AirCondModeSettingValue::AUTO => 1,
            AirCondModeSettingValue::COOL => 2,
            AirCondModeSettingValue::DRY => 3,
            AirCondModeSettingValue::FAN => 4,
            AirCondModeSettingValue::HEAT => 5,
        },
        match fan_speed {
            AirCondFanSpeedSettingValue::AUTO => 1,
            AirCondFanSpeedSettingValue::LOW => 2,
            AirCondFanSpeedSettingValue::MEDIUM => 3,
            AirCondFanSpeedSettingValue::HIGH => 4,
        },
        if power_state { "on" } else { "off" }
    );
    let body = format!("{{\"commandType\":\"command\",\"command\":\"setAll\",\"parameter\":\"{}\"}}", _parameter);

    // APIを叩く
    let res = client.post(url).headers(headers).body(body).send().await.expect("Failed to switch infrared remote status");

    // ステータスコードに応じてboolを返す
    let status_code = res.status().as_u16();
    if status_code == 200 {
        Ok(())
    } else {
        Err(res.error_for_status().unwrap_err())
    }
}
