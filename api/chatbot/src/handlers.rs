use actix_web::{web, HttpResponse};

use log::info;
use qdrant_client::prelude::*;

use crate::api_client::OpenAiClient;
use crate::error::{Error, Result};
use crate::models::{QueryRequest, UploadRequest};
use crate::text_splitter::RecursiveCharacterTextSplitter;
use crate::utils::extract_text_from_pdf;
use serde_json::json;

pub async fn list_handler() -> Result<HttpResponse> {
    info!("list_handler");
    get_qdrant_client().await?;
    Ok(HttpResponse::Ok().body("list"))
}

async fn get_qdrant_client() -> Result<QdrantClient> {
    let config = QdrantClientConfig::from_url("http://172.30.131.214:6334");
    let client = match QdrantClient::new(Some(config)).await {
        Ok(docs) => docs,
        Err(err) => return Err(Error::QdrantError(err.to_string())),
    };
    let collections_list = client.list_collections().await.unwrap();
    dbg!(collections_list);
    Ok(client)
}
pub async fn upload_handler(request: web::Json<UploadRequest>) -> HttpResponse {
    info!("upload_handler");
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
    info!("split_file_content");
    // 从文件中提取文本内容
    let text = extract_text_from_pdf(file_path)?;
    info!("RecursiveCharacterTextSplitter");
    // 使用TextSplitter切片文本内容，每1500字符切片
    let splitter = RecursiveCharacterTextSplitter::new(None, true, 1500);
    let docs = splitter.split_text(&text);

    Ok(docs)
}

async fn insert_vectors_into_qdrant(docs: Vec<String>) -> Result<()> {
    info!("insert_vectors_into_qdrant");
    // 使用OpenAI API进行向量生成和向量库插入操作
    let openai_client = OpenAiClient::new("d9673357f6a540dc92e66626d39ae31f".to_string());

    let config: QdrantClientConfig =
        QdrantClientConfig::from_url("http://172.30.131.214:6334");
    let qdrant_client = match QdrantClient::new(Some(config)).await {
        Ok(c) => c,
        Err(err) => return Err(Error::QdrantError(err.to_string())),
    };

    for doc in docs {
        info!("doc is {}", doc);
        // 生成向量并插入向量库
        let vector = openai_client.get_embeddings(&doc).await?;
        // let vector: Vec<f32> = vec![0.5, 0.3, 0.2, 0.7, 0.9];
        info!("vector 0 is {}", vector[0]);
        let payload: Payload = json!(
            {
                "doc": doc,
            }
        )
        .try_into()
        .unwrap();
        let points = vec![PointStruct::new(0, vector, payload)];
        // 将向量插入qdrant向量库
        qdrant_upsert(&qdrant_client, points).await?;
    }

    Ok(())
}

async fn qdrant_upsert(
    qdrant_client: &QdrantClient,
    points: Vec<PointStruct>,
) -> Result<(), Error> {
    qdrant_client
        .upsert_points_blocking("chat-pdf", points, None)
        .await
        .map_err(|err| Error::QdrantError(err.to_string()))?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use actix_web::web;
    use actix_web::App;
    use reqwest::StatusCode;

    #[actix_rt::test]
    async fn test_upload_handler() {
        // Create a test upload request with a mock file path
        let upload_request = web::Json(UploadRequest {
            file: "/Users/huisheng/Desktop/bbbb.txt".to_string(),
        });

        // Send a test request to the upload_handler
        let mut app =
            test::init_service(App::new().route("/upload", web::post().to(upload_handler))).await;

        let request = test::TestRequest::post()
            .uri("/upload")
            .set_json(&upload_request)
            .to_request();

        let response = test::call_service(&mut app, request).await;

        // Assert that the response status code is 200 (OK)
        assert_eq!(response.status(), StatusCode::OK);

        // Assert that the response body is the expected message
        let body = test::read_body(response).await;
        assert_eq!(body, "File uploaded successfully");
    }
}
