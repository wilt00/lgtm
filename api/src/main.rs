use lgtm;

use std::env;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index<'a>() -> impl Responder {
    HttpResponse::Ok().body(lgtm::lgtm() + "\n")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let default_port = String::from("8088");
    let port = env::var("PORT")
        .unwrap_or_else(|_| default_port)
        .parse()
        .expect("PORT must be a number");
    println!("Listening on port: {}", port);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
