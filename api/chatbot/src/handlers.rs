use actix_web::{web, HttpResponse};
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{CreateCollection, SearchPoints, VectorParams, VectorsConfig};

use crate::api_client::OpenAiClient;
use crate::error::{Error, Result};
use crate::models::{QueryRequest, UploadRequest};
use crate::text_splitter::RecursiveCharacterTextSplitter;
use crate::utils::extract_text_from_pdf;

pub async fn upload_handler(request: web::Json<UploadRequest>) -> HttpResponse {
    // 读取文件流
    let file_path = &request.file;

    // 切片文件内容。使用TextSplitter里面的方法，每1500字符切片。返回docs的列表
    let docs = match split_file_content(file_path) {
        Ok(docs) => docs,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    // 拿docs列表，访问OpenAI的embedding，生成向量和索引，插入向量库（qdrant）
    match insert_vectors_into_qdrant(docs).await {
        Ok(_) => HttpResponse::Ok().body("File uploaded successfully"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

fn split_file_content(file_path: &str) -> Result<Vec<String>> {
    // 从文件中提取文本内容
    let text = extract_text_from_pdf(file_path)?;

    // 使用TextSplitter切片文本内容，每1500字符切片
    let splitter = RecursiveCharacterTextSplitter::new(None, true, 1500);
    let docs = splitter.split_text(&text);

    Ok(docs)
}

async fn insert_vectors_into_qdrant(docs: Vec<String>) -> Result<()> {
    // 使用OpenAI API进行向量生成和向量库插入操作
    let openai_client = OpenAiClient::new("YOUR_API_KEY".to_string());

    let config: QdrantClientConfig = QdrantClientConfig::from_url("http://localhost:6334");
    let qdrant_client = match QdrantClient::new(Some(config)).await {
        Ok(c) => c,
        Err(err) => return Err(Error::QdrantError(err.to_string())),
    };

    for doc in docs {
        // 生成向量并插入向量库
        let vector = openai_client.get_embeddings(&doc).await?;

        // 将向量插入qdrant向量库
        // qdrant_client
        //     .upsert_points_blocking("collection_name", vector, None)
        //     .await?;
    }

    Ok(())
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
