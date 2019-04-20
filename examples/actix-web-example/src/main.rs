#![feature(await_macro, futures_api, async_await)]

use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_async_compat::async_compat;
use futures::Future;
use tokio_async_await::await;

use hyper::Client;
use std::{
    io,
    time::{Duration, Instant},
};
use tokio::prelude::*;

#[get("/welcome")]
#[async_compat]
async fn index() -> Result<HttpResponse> {
    let client = Client::new();
    let uri = "http://httpbin.org/ip".parse().unwrap();

    let response = await!({ client.get(uri).timeout(Duration::from_secs(10)) }).unwrap();

    println!("Response: {}", response.status());

    let mut body = response.into_body();

    await!(body.for_each(|chunk| {
        io::stdout()
            .write_all(&chunk)
            .map_err(|e| panic!("example expects stdout is open, error={}", e))
    }))
    .unwrap();

    Ok(HttpResponse::Ok().body("OK"))
}

#[async_compat]
async fn index2(_req: HttpRequest) -> Result<HttpResponse> {
    use tokio::timer::Delay;

    // Wait 2s
    await!(Delay::new(Instant::now() + Duration::from_secs(2)))?;

    Ok(HttpResponse::Ok().body("OK"))
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(web::resource("/welcome2").route(web::get().to_async(index2)))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap();
}
