use std::sync::Arc;

use async_std::{
    sync::Mutex,
    task::{self},
};
use serde::{Deserialize, Serialize};
use tide::{
    http::headers::HeaderValue,
    security::{CorsMiddleware, Origin},
    Body, Request,
};

#[derive(Deserialize, Serialize)]
struct LedResponse {
    status: bool,
}
#[derive(Deserialize, Serialize)]
struct LedRequest {
    on: bool,
}
mod service;

use crate::service::orchestrator_service::RdcService;

#[async_std::main]
async fn main() -> tide::Result<()> {
    femme::start();
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());

    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(true);

    app.with(cors);

    let rdc_service = RdcService::new();

    let arc_rdc_service = Arc::new(Mutex::new(rdc_service));
    let arc = arc_rdc_service.clone();

    app.at("/api/action").post(move |mut req: Request<()>| {
        let arc = arc.clone();
        async move {
            let led_request: LedRequest = req.body_json().await?;
            let led_response = arc.lock().await.led_on_or_off(led_request);
            Body::from_json(&led_response)
        }
    });

    let arc = arc_rdc_service.clone();
    app.at("/api/status").get(move |_| {
        let arc = arc.clone();
        async move {
            let led_response = arc.lock().await.is_led_on();
            Body::from_json(&led_response)
        }
    });

    let mut arc = arc_rdc_service.clone();

    // task::spawn(async move {
    //     println!("starting sensor...");
    //     loop {
    //         println!("starting step");
    //         let mutable = &mut arc;
    //         if mutable.lock().await.is_pir_active() {
    //             println!("movement!");
    //         }
    //         mutable.lock().await.wait_for_motion();
    //         println!("movement!");
    //         mutable.lock().await.wait_for_no_motion();
    //         println!("ent step");
    //     }
    // });

    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
