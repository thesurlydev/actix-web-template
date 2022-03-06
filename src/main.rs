use actix_web::{App, guard, HttpRequest, HttpResponse, HttpServer, web};
use actix_web::middleware::Logger;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Info {
    first_name: String,
    last_name: String,
}

#[derive(Debug, Serialize)]
struct Output {
    message: String,
}

async fn index() -> HttpResponse {
    info!("Hello");
    HttpResponse::Ok().body("Hello")
}

async fn user(info: web::Json<Info>, req: HttpRequest) -> HttpResponse {
    info!("{:?}", &req);
    let out = Output {
        message: format!("Hello there {}", info.first_name)
    };
    HttpResponse::Ok().json(out)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .route("/", web::get().to(index))
            .route("/user", web::post().to(user)
                .guard(guard::Header("content-type", "application/json")))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
