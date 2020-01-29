use actix_web::{web, App, Responder, HttpServer};

async fn index(info: web::Path<String>) -> impl Responder {
    format!("Hello {}!", info)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
/*    HttpServer::new(|| App::new().service(
        web::resource("/{name}").to(index))
    )
        .bind("localhost:8080")?
        .run()
        .await*/
    HttpServer::new(|| App::new().service(
        web::resource("/{name}").to(index))
    ).bind("localhost:8080")?.run().await
}
