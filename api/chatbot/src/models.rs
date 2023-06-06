use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UploadRequest {
    pub file: String, // 根据你的需求，这里可以是文件路径、文件名或其他文件标识符
}

#[derive(Deserialize)]
pub struct QueryRequest {
    pub query: String, // 根据你的需求，这里可以是查询的内容、关键字等
}

#[derive(Deserialize)]
pub struct ApiResponse {
    // 定义通用的API响应结构
    // 例如：状态码、消息等
}
