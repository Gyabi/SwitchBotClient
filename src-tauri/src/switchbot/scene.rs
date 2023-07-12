use super::utils::{create_header, SWITCH_BOT_API_URL};
use serde::{Deserialize, Serialize};
use std::error::Error;

// シーンを示す構造体
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetScenesResponse {
    status_code : u16,
    message : String,
    body : Vec<Scene>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Scene {
    scene_id : String,
    scene_name : String
}

// シーンのリストを出力する関数
pub(crate) async fn get_scene_list(token: &str, secret:&str) -> Result<Vec<Scene>, Box<dyn Error>> {
    // reqwestクレートを用いてAPIを叩く
    let url = format!("{}/scenes", SWITCH_BOT_API_URL);
    let client = reqwest::Client::new();

    // ヘッダーを作成
    let headers = create_header(token, secret);

    // APIを叩く
    let res = client.get(url).headers(headers).send().await;
    // resのステータスコードを使ってエラー処理
    match res {
        Ok(res) => {
            if res.status().is_success() {
                // レスポンスからSceneのリストを取り出す
                let res_data:GetScenesResponse = res.json().await.expect("Failed to parse get scene list response");

                Ok(res_data.body)
            } else {
                Err("Failed to get scene list".into())
            }
        },
        Err(e) => Err(e.into())
    }
}

// シーンを実行する関数
pub(crate) async fn post_scene(token: &str, secret:&str, scene_id:&str) -> Result<(), Box<dyn Error>> {
    // reqwestクレートを用いてAPIを叩く
    let url = format!("{}/scenes/{}/execute", SWITCH_BOT_API_URL, scene_id);
    let client = reqwest::Client::new();

    // ヘッダーを作成
    let headers = create_header(token, secret);

    // APIを叩く
    let res = client.post(url).headers(headers).send().await;

    // resのステータスコードを使ってエラー処理
    match res {
        Ok(res) => {
            if res.status().is_success() {
                Ok(())
            } else {
                Err("Failed to post scene".into())
            }
        },
        Err(e) => Err(e.into())
    }
}
