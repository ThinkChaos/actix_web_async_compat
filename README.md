This crate allows using nightly async/await features with the latest [actix-web](https://crates.io/crates/actix-web) version (1.0.0-rc).

### Example

```rust
#![feature(await_macro, async_await)]

use actix_web::{get, web, App, Error, HttpResponse, HttpServer, Result};
use actix_web_async_compat::async_compat;
use futures03::{compat::Future01CompatExt as _, FutureExt as _, TryFutureExt as _};
use hyper::Client;
use serde::Deserialize;
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

    let response = client
        .get(uri)
        .compat()
        .await
        .map_err(|e| HttpResponse::InternalServerError().body(format!("ERROR: {:?}", e)))?;

    println!("Response: {}", response.status());

    let mut body = response.into_body();

    body.for_each(|chunk| {
        io::stdout()
            .write_all(&chunk)
            .map_err(|e| panic!("example expects stdout is open, error={}", e))
    })
    .compat()
    .map_err(|e| HttpResponse::InternalServerError().body(format!("ERROR: {:?}", e)))
    .await?;

    Ok(HttpResponse::Ok().body("OK"))
}

#[derive(Debug, Deserialize)]
struct UserForm {
    name: String,
}

#[async_compat]
async fn index2(form: actix_web::web::Form<UserForm>) -> Result<HttpResponse> {
    dbg!(form);
    use tokio::timer::Delay;

    // Wait 2s
    Delay::new(Instant::now() + Duration::from_secs(2))
        .compat()
        .await?;

    Ok(HttpResponse::Ok().body("OK"))
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(index)
            .service(web::resource("/welcome2").route(web::post().to_async(index2)))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap();
}
```

Please take a look into `examples` dir for more information.
