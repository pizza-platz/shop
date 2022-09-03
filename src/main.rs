mod api;
mod status;

use anyhow::Result;
use status::ShopStatus;
use tokio::select;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let shop_status = ShopStatus::new();

    select! {
        result = api::start(shop_status.clone()) => {
            Ok(result?)
        }
        result = shop_status.start() => {
            Ok(result?)
        }
    }
}
