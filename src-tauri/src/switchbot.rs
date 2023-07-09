// SwitchBotのAPIを叩く関数の集合

use std::vec;

use reqwest::header::HeaderMap;
use uuid::Uuid;
use reqwest::{
    self,
    header::{AUTHORIZATION, CONTENT_TYPE},
};
use chrono::Utc;
use ring::hmac;
use base64::{Engine as _, engine::general_purpose};
use serde::Deserialize;

// URLを変数として保持
const SWITCH_BOT_API_URL: &str = "https://api.switch-bot.com/v1.1";

// デバイスを示す構造体
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub(crate) struct Device {
    device_id: String,
    device_name: String,
    device_type: String,
    enable_cloud_service: bool,
    hub_device_id: String
}

// デバイスのリストを出力する関数
pub(crate) async fn get_device_list(token: &str, secret:&str) -> Result<Vec<Device>, reqwest::Error> {
    // reqwestクレートを用いてAPIを叩く
    let url = format!("{}/devices", SWITCH_BOT_API_URL);
    let client = reqwest::Client::new();

    // ヘッダーを作成
    let headers = create_header(token, secret);

    // APIを叩く
    let res = client.get(url).headers(headers).send().await;

    // レスポンスからbodyを取り出して表示
    let body = res?.text().await?;
    println!("{}", body);

    Ok(vec![])
}

// ヘッダーを作成する共用関数
fn create_header(token: &str, secret:&str) -> HeaderMap {
    let t = Utc::now().timestamp_millis();
    let nonce = Uuid::new_v4().to_string();
    let sign = {
        let key = hmac::Key::new(ring::hmac::HMAC_SHA256, secret.as_bytes());
        let data = format!("{}{}{}", token, t, nonce);
        let sign = hmac::sign(&key, data.as_bytes());
        general_purpose::STANDARD.encode(sign.as_ref())
    };

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, token.parse().unwrap());
    headers.insert("sign", sign.parse().unwrap());
    headers.insert("t", t.to_string().parse().unwrap());
    headers.insert("nonce", nonce.parse().unwrap());

    headers
}