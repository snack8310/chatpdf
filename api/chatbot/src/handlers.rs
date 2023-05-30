use actix_web::{web, HttpResponse};

use crate::models::{UploadRequest, QueryRequest};
use crate::utils::extract_text_from_pdf;
use crate::api_client::OpenAiClient;
use crate::error::Result;

pub async fn upload_handler(request: web::Json<UploadRequest>) -> HttpResponse {
    // 处理上传请求，切片文件，向量化，上传到向量库
    // 返回成功响应或错误响应
    match upload_file(&request.0.file) {
        Ok(_) => HttpResponse::Ok().body("File uploaded successfully"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn query_handler(request: web::Query<QueryRequest>) -> HttpResponse {
    // 处理查询请求，从向量库中检索相似向量，使用ChatGPT生成答案
    // 返回答案响应或错误响应
    match search_vector(&request.0.query) {
        Ok(answer) => HttpResponse::Ok().body(answer),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

fn upload_file(file: &str) -> Result<()> {
    // 处理上传文件逻辑
    // 返回成功或错误
    // 使用 error.rs 中的 Result 类型
    // 可能的错误：HttpError、IoError、JsonError、QdrantError
    // 示例代码：
    // let qdrant_client = qdrant::Client::new();
    // qdrant_client.upload_file(file)?;
    Ok(())
}

fn search_vector(query: &str) -> Result<String> {
    // 处理向量检索和ChatGPT逻辑
    // 返回答案或错误
    // 使用 error.rs 中的 Result 类型
    // 可能的错误：HttpError、IoError、JsonError、QdrantError
    // 示例代码：
    // let qdrant_client = qdrant::Client::new();
    // let result = qdrant_client.search(query)?;
    // let answer = openai_chat_gpt(&result)?;
    // Ok(answer)
    Ok(String::from("Sample answer"))
}
