use std::env;
use std::str::FromStr;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use log::info;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = match env::var_os("PORT") {
        None => 5000,
        Some(p) => u16::from_str(p.into_string().unwrap().as_str()).unwrap(),
    };

    info!("Starting server on {}", &port);

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .service(hello)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
