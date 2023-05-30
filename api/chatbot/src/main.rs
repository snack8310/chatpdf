use actix_web::{web, App, HttpServer};

mod handlers;
mod models;
mod utils;
mod api_client;
mod error;
mod text_splitter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置路由
    HttpServer::new(|| {
        App::new()
            .route("/upload", web::post().to(handlers::upload_handler))
            .route("/query", web::get().to(handlers::query_handler))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
