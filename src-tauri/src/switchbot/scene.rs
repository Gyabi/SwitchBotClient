use super::utils::{create_header, SWITCH_BOT_API_URL};
use serde::{Deserialize, Serialize};

// シーンを示す構造体
#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct GetScenesResponse {
    status_code : u16,
    message : String,
    body : GetScenesResponseBody
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct  GetScenesResponseBody {
    scene_list : Vec<Scene>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct Scene {
    scene_id : String,
    scene_name : String
}

// シーンのリストを出力する関数
pub(crate) async fn get_scene_list(token: &str, secret:&str) -> Result<Vec<Scene>, reqwest::Error> {
    // reqwestクレートを用いてAPIを叩く
    let url = format!("{}/scenes", SWITCH_BOT_API_URL);
    let client = reqwest::Client::new();

    // ヘッダーを作成
    let headers = create_header(token, secret);

    // APIを叩く
    let res = client.get(url).headers(headers).send().await;

    // レスポンスからSceneのリストを取り出す
    let body = res?.text().await?;
    let get_scenes_response: GetScenesResponse = serde_json::from_str(&body).unwrap();
    let scene_list = get_scenes_response.body.scene_list;

    Ok(scene_list)
}

// シーンを実行する関数
pub(crate) async fn post_scene(token: &str, secret:&str, scene_id:&str) -> bool {
    // reqwestクレートを用いてAPIを叩く
    let url = format!("{}/scenes/{}/execute", SWITCH_BOT_API_URL, scene_id);
    let client = reqwest::Client::new();

    // ヘッダーを作成
    let headers = create_header(token, secret);

    // APIを叩く
    let res = client.post(url).headers(headers).send().await;

    // ステータスコードに応じてboolを返す
    let status_code = res.unwrap().status().as_u16();
    if status_code == 200 {
        true
    } else {
        false
    }
}
