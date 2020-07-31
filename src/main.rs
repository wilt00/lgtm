mod lgtm;

use std::env;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

static L_INPUT: &str = include_str!("dicts/l.txt");
static G_INPUT: &str = include_str!("dicts/g.txt");
static T_INPUT: &str = include_str!("dicts/t.txt");
static M_INPUT: &str = include_str!("dicts/m.txt");

async fn index<'a>(data: web::Data<lgtm::Dict<'a>>) -> impl Responder {
    HttpResponse::Ok().body(lgtm::lgtm(&data) + "\n")
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
            .data(lgtm::Dict {
                l: L_INPUT.split('\n').collect(),
                g: G_INPUT.split('\n').collect(),
                t: T_INPUT.split('\n').collect(),
                m: M_INPUT.split('\n').collect(),
            })
            .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
