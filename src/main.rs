use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};

async fn status() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hey")]
async fn hey() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))
            .service(hey)
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}
