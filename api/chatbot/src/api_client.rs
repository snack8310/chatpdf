use reqwest::Client;
use serde_json::json;
// use serde_json::json;
use crate::error::{Result,Error};
use serde::{Deserialize, Serialize};

pub struct OpenAiClient {
    api_key: String,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
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

    pub async fn get_embeddings(&self, text: &str) -> Result<Vec<f32>> {
        // 使用OpenAI API进行文本向量化请求
        // 返回文本的向量或错误
        // 使用 error.rs 中的 Result 类型
        // 可能的错误：HttpError
        let url = "https://api.openai.com/v1/engines/davinci-codex/completions";
        let payload = json!({
            "documents": [text],
            "model": "text-davinci-003"
        });

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload)
            .send()
            .await?;

        let json_response: serde_json::Value = response.json().await?;
        let embeddings = json_response["documents"][0]["embedding"]
            .as_array()
            .ok_or(Error::QdrantError("Failed to get embeddings".to_string()))?;

        let embeddings: Vec<f32> = embeddings
            .iter()
            .map(|v| v.as_f64().unwrap() as f32)
            .collect();

        Ok(embeddings)
    }
}
