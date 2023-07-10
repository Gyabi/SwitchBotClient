use reqwest::header::HeaderMap;
use uuid::Uuid;
use reqwest::header::AUTHORIZATION;
use chrono::Utc;
use ring::hmac;
use base64::{Engine as _, engine::general_purpose};
use reqwest;

use super::super::param::{Parameter, set_parameter};
// SwitchBotのAPIを叩く関数の集合


// deviceAPI用構造体
pub(crate) struct GetDeviceResponse {
    pub(crate) body: String,
    pub(crate) status: u16,
}


// URLを変数として保持
pub(crate) const SWITCH_BOT_API_URL: &str = "https://api.switch-bot.com/v1.1";

// ヘッダーを作成する共用関数
pub(crate) fn create_header(token: &str, secret:&str) -> HeaderMap {
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

// パラメータ保存
#[tauri::command]
pub(crate) fn save_parameter(param: Parameter) -> bool {
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