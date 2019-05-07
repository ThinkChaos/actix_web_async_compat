This crate allows using nightly async/await features with the latest [actix-web](https://crates.io/crates/actix-web) version (1.0.0-beta.3).

### Example

```rust
#![feature(await_macro, async_await)]

use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_async_compat::async_compat;
use futures::Future;
use tokio_async_await::await;
use std::time::{Duration, Instant};

#[get("/welcome")]
#[async_compat]
async fn index(req: HttpRequest) -> Result<HttpResponse> {
    println!("req: {:?}", req);
    Ok(HttpResponse::Ok().body("OK"))
}

#[async_compat]
async fn index2() -> Result<HttpResponse> {
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
```

Please take a look into `examples` dir for more information.
