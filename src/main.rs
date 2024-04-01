use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
  "<p>Hello World!</p>"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| App::new().service(index))
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}