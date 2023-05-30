use reqwest::{Client, Error};

use crate::error::Result;

pub struct OpenAiClient {
    api_key: String,
    client: Client,
}

impl OpenAiClient {
    pub fn new(api_key: String) -> Self {
        let client = Client::new();
        OpenAiClient { api_key, client }
    }

    pub async fn chat_gpt(&self, message: &str) -> Result<String> {
        // 使用OpenAI API进行ChatGPT的请求
        // 返回ChatGPT生成的答案或错误
        // 使用 error.rs 中的 Result 类型
        // 可能的错误：HttpError
        // 示例代码：
        // let response = self.client.post("https://api.openai.com/v1/...")
        //     .header("Authorization", format!("Bearer {}", self.api_key))
        //     .json(...)
        //     .send()
        //     .await?;
        // let answer: String = response.json().await?;
        // Ok(answer)
        Ok(String::from("Sample answer"))
    }
}
