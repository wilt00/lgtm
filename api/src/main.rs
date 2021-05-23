use actix_web::{get, App, HttpRequest, HttpServer};
use lgtm::lgtm;
use serde::Deserialize;

fn default_port() -> u16 {
    8088
}

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default = "default_port")]
    port: u16,
}

#[get("/")]
async fn index(_req: HttpRequest) -> String {
    lgtm() + "\n"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match envy::from_env::<Config>() {
        Ok(config) => {
            println!("Listening on port: {}", config.port);
            HttpServer::new(|| App::new().service(index))
                .bind(("0.0.0.0", config.port))?
                .run()
                .await
        }
        Err(error) => panic!("{:#?}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}
