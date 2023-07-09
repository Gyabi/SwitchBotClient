// SwitchBotのAPIを実行するモジュール
// 実行用にSwitchBotAPIClient構造体を定義する

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct SwitchBotAPIClient {
    token: String,
    device_id: String,
    client: Client,
}

impl SwitchBotAPIClient {
    pub fn new(token: String, device_id: String) -> Self {
        Self {
            token,
            device_id,
            client: Client::new(),
        }
    }

    pub fn execute(&self, command: &str) -> Result<ExecuteResponse, reqwest::Error> {
        let mut map = HashMap::new();
        map.insert("command", command);

        let url = format!(
            "https://api.switch-bot.com/v1.0/devices/{}/commands",
            self.device_id
        );

        let res = self
            .client
            .post(&url)
            .header("Authorization", &self.token)
            .json(&map)
            .send()?
            .text()?;

        let res: ExecuteResponse = serde_json::from_str(&res)?;

        Ok(res)
    }
}