use crate::error::Result;
use actix_web::{web, App, HttpServer};
use log::LevelFilter;
use simple_logger::SimpleLogger;

mod api_client;
mod error;
mod handlers;
mod models;
mod text_splitter;
mod utils;

#[actix_web::main]
async fn main() -> Result<()> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
    // 设置路由
    HttpServer::new(|| {
        App::new()
            .route("/upload", web::post().to(handlers::upload_handler))
            .route("/list", web::get().to(handlers::list_handler))
            .route("/query", web::get().to(handlers::query_handler))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}
