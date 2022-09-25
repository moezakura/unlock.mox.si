use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::error::Error;

pub struct Service {
    pub switch_bot_token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandResult {
    pub status_code: i32,
    pub body: Map<String, Value>,
    pub message: String,
}

impl Service {
    pub fn new(switch_bot_token: String) -> Self {
        Self { switch_bot_token }
    }

    pub fn clone(&self) -> Self {
        Self {
            switch_bot_token: self.switch_bot_token.clone(),
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
        let client = reqwest::Client::new();
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
}
