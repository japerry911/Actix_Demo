use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello World Again")
}

async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hello for the third time")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .route("/again2", web::get().to(index3))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}