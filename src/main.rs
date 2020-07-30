use std::env;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rand::seq::SliceRandom;

static L_INPUT: &str = include_str!("dicts/l.txt");
static G_INPUT: &str = include_str!("dicts/g.txt");
static T_INPUT: &str = include_str!("dicts/t.txt");
static M_INPUT: &str = include_str!("dicts/m.txt");

struct Dict<'a> {
    l: Vec<&'a str>,
    g: Vec<&'a str>,
    t: Vec<&'a str>,
    m: Vec<&'a str>,
}

async fn index<'a>(data: web::Data<Dict<'a>>) -> impl Responder {
    let mut rng = rand::thread_rng();
    HttpResponse::Ok().body(
        [
            (&data.l[..], "let's"),
            (&data.g[..], "get"),
            (&data.t[..], "this"),
            (&data.m[..], "merged"),
        ]
        .iter()
        .map(|d| *d.0.choose(&mut rng).unwrap_or_else(|| &d.1))
        .collect::<Vec<&str>>()
        .join(" ") + "\n",
    )
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
            .data(Dict {
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
