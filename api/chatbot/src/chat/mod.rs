use async_trait::async_trait;

use crate::error::Result;
use serde::{Deserialize, Serialize};

pub mod chat_gpt_35_turbo;
pub mod vector_db;

#[async_trait]
pub trait ChatApi {
    async fn get_answer(
        &self,
        // client: &Client,
        token: String,
        prompt: String,
        context: &Vec<Conversation>,
        message_from_user: &str,
    ) -> Result<String>;

    // fn get_chat_history() -> Result<String>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub req_message: String,
    pub resp_message: String,
}
