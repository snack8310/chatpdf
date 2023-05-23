use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chat_gpt::{Conversation, ChatApi, chat_gpt_35_turbo::ChatGpt35Turbo};
use error::Result;
use serde::Deserialize;
mod error;
mod chat_gpt;

#[derive(Deserialize)]
struct ChatInput {
    message: String,
}

async fn call_chat_gpt(
    input: String,
    system_prompt: String,
    chat_histories: Vec<Conversation>,
) -> Result<String> {
    // 在这里调用你的ChatGPT模型
    let chat_api = ChatGpt35Turbo {};

    let chat_gpt_token = "";
    let message_from_chat = chat_api
        .send_message(chat_gpt_token.to_owned(), system_prompt, &chat_histories, &input)
        .await?;

    Ok(String::from(message_from_chat.trim()))
}

async fn chat(input: web::Json<ChatInput>) -> impl Responder {
    // let response = call_chat_gpt(input.message.clone()).await;
    // HttpResponse::Ok().json(response.ok())
    HttpResponse::Ok().json("success")
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(chat)))
        .bind("127.0.0.1:8088")?
        .run()
        .await?;
    Ok(())
}
