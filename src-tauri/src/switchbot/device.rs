use super::utils::{create_header, SWITCH_BOT_API_URL};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct GetDeviceResponse {
    
    status_code: u16,
    body: String,
    message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct GetDeviceResponseBody {
    device_list: Vec<Device>,
    infrared_remote_list: Vec<InfraredRemote>,
    
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct Device {
    device_id: String,
    device_name: String,
    device_type: String,
    enable_cloud_service: Option<bool>,
    hub_device_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct InfraredRemote {
    device_id : String,
    device_name : String,
    remote_type : String,
    hub_device_id : String
}

// デバイスのリストを出力する関数
pub(crate) async fn get_infrared_remote_list(token: &str, secret:&str) -> Result<Vec<InfraredRemote>, reqwest::Error> {
    // reqwestクレートを用いてAPIを叩く
    let url = format!("{}/devices", SWITCH_BOT_API_URL);
    let client = reqwest::Client::new();

    // ヘッダーを作成
    let headers = create_header(token, secret);

    // APIを叩く
    let res = client.get(url).headers(headers).send().await;

    // レスポンスからbodyからinfraredRemoteListをInfraredRemoteのリストとして取り出す
    let body = res?.text().await?;
    let get_device_response: GetDeviceResponse = serde_json::from_str(&body).unwrap();
    let get_device_response_body: GetDeviceResponseBody = serde_json::from_str(&get_device_response.body).unwrap();
    let infrared_remote_list = get_device_response_body.infrared_remote_list;

    Ok(infrared_remote_list)
}

