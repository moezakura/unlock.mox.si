use serde::Deserialize;
use std::error::Error;

pub struct Service {}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub switch_bot_token: String,
    pub switch_bot_secret: String,
    pub interphone_bot_id: String,
    pub lock_bot_id: String,
}

impl Service {
    pub fn load() -> Result<Config, Box<dyn Error>> {
        let config = envy::from_env::<Config>();
        Ok(config?)
    }
}
