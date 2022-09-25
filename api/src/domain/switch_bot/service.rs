use chrono::Local;
use hmac::{Hmac, Mac};
use reqwest::header::HeaderMap;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::Sha256;
use std::collections::HashMap;
use std::error::Error;

use crate::domain::switch_bot::random;
type HmacSha256 = Hmac<Sha256>;

pub struct Service {
    pub switch_bot_token: String,
    pub switch_bot_secret: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandResult {
    pub status_code: i32,
    pub body: Map<String, Value>,
    pub message: String,
}

impl Service {
    pub fn new(switch_bot_token: String, switch_bot_secret: String) -> Self {
        Self {
            switch_bot_token,
            switch_bot_secret,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            switch_bot_token: self.switch_bot_token.clone(),
            switch_bot_secret: self.switch_bot_secret.clone(),
        }
    }

    /**
     * execute command to switch bot
     */
    async fn exec_command(
        &self,
        device_id: String,
        command: String,
    ) -> Result<CommandResult, Box<dyn Error>> {
        let client = Client::new();
        let mut params = HashMap::new();
        params.insert("command", command);
        params.insert("parameter", "default".to_string());
        params.insert("commandType", "command".to_string());

        let res = client
            .post(format!(
                "https://api.switch-bot.com/v1.0/devices/{}/commands",
                device_id
            ))
            .header("Authorization", format!("Bearer {}", self.switch_bot_token))
            .json(&params)
            .send()
            .await?;

        let command_result: CommandResult = res.json().await?;
        Ok(command_result)
    }

    /**
     * open interphone
     */
    pub async fn push_button(&self, bot_id: String) -> Result<CommandResult, Box<dyn Error>> {
        let res = self.exec_command(bot_id, "press".to_string()).await?;
        Ok(res)
    }

    // open door lock
    pub async fn open_lock(&self, bot_id: String) -> Result<CommandResult, Box<dyn Error>> {
        let command = "unlock".to_string();

        let secret = self.switch_bot_secret.clone();
        let time: i64 = Local::now().timestamp_millis();
        let nonce = random::random_string(24);
        let payload = format!("{}{}{}", self.switch_bot_token, time, nonce.clone());

        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("");
        mac.update(payload.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        let sign = base64::encode(&code_bytes);

        let client = Client::new();
        let mut params = HashMap::new();
        params.insert("command", command);
        params.insert("parameter", "default".to_string());
        params.insert("commandType", "command".to_string());

        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            self.switch_bot_token.clone().parse().unwrap(),
        );
        headers.insert("sign", sign.parse().unwrap());
        headers.insert("nonce", nonce.parse().unwrap());
        headers.insert("t", time.to_string().parse().unwrap());

        let res = client
            .post(format!(
                "https://api.switch-bot.com/v1.1/devices/{}/commands",
                bot_id
            ))
            .headers(headers)
            .json(&params)
            .send()
            .await?;

        let command_result: CommandResult = res.json().await?;
        Ok(command_result)
    }
}
