use std::io::Result;

use actix_web::{App, Error, HttpServer};
use paperclip::actix::{api_v2_operation, get, web, Apiv2Schema, OpenApiExt};
use serde::{Deserialize, Serialize};

#[api_v2_operation]
#[get("/health")]
async fn health() -> core::result::Result<web::Json<HealthResponse>, Error> {
    Ok(web::Json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
    }))
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
struct HealthResponse {
    pub status: String,
    pub version: String,
}

pub async fn serve(address: String) -> Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap_api()
            .with_json_spec_at("/-/api/spec/v2")
            .with_rapidoc_at("/-/rapidoc")
            .with_swagger_ui_at("/-/swagger")
            .service(health)
            .build()
    })
    .bind(address)?;

    server.run().await
}
