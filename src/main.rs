use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::Local;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Brayckner's Computer!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let response_string = format!("[{}] - {}", current_time, req_body);
    HttpResponse::Ok().body(response_string)
}

async fn manuel_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there from manual response")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manuel_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}