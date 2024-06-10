//! src/main.rs
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

// cargo watch로 수정하면 바로바로 적용되게 해보자 (아니 이것은 마치 nodemon??)
//   cargo watch -x check -x test -x run
// cargo expand로 tokio::main을 확인해보자!
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
