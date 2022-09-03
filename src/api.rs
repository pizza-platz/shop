use crate::status::ShopStatus;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use platz_sdk::{Metric, PlatzStatus, StatusColor};
use serde::Serialize;

pub async fn start(shop_status: ShopStatus) -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shop_status.clone()))
            .configure(configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/status", web::get().to(status));
}

#[derive(Clone, Serialize)]
enum OpenStatus {
    Opened,
    Closed,
}

async fn status(shop_status: web::Data<ShopStatus>) -> impl Responder {
    let platz_status = PlatzStatus {
        status: if shop_status.is_open().await {
            platz_sdk::Status {
                name: OpenStatus::Opened,
                color: StatusColor::Success,
            }
        } else {
            platz_sdk::Status {
                name: OpenStatus::Closed,
                color: StatusColor::Danger,
            }
        },
        primary_metric: Some(Metric {
            value: shop_status.revenue().await,
            unit: "USD".to_owned(),
            color: Some(StatusColor::Success),
            short_description: "Daily Revenue".to_owned(),
        }),
        metrics: None,
        notices: Default::default(),
    };

    HttpResponse::Ok().json(platz_status)
}
