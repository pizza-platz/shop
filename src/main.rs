mod api;

use actix_web::{App, HttpServer};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();

    Ok(HttpServer::new(|| App::new().configure(api::configure))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?)
}
