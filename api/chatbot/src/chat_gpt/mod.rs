use async_trait::async_trait;

use crate::error::Result;
use chat_gpt_35_turbo::ChatGpt35Turbo;
use serde::{Deserialize, Serialize};

pub mod chat_gpt_35_turbo;

#[async_trait]
pub trait ChatApi {
    async fn send_message(
        &self,
        // client: &Client,
        token: String,
        prompt: String,
        context: &Vec<Conversation>,
        message_from_user: &str,
    ) -> Result<String>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub req_message: String,
    pub resp_message: String,
}

pub fn _get_chat_api(api_model: String) -> Box<dyn ChatApi> {
    let api: Box<dyn ChatApi> = match api_model.as_str() {
        "gpt-3.5-turbo" => Box::new(ChatGpt35Turbo),
        _ => Box::new(ChatGpt35Turbo),
    };
    api
}
