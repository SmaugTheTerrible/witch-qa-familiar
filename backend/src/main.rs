use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpServer, Responder};
use dotenv::dotenv;
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::str::FromStr;

#[get("/hello")]
async fn hello() -> actix_web::Result<impl Responder> {
    let resp = Resp {
        result: format!("Hello User"),
    };
    Ok(web::Json(resp))
}

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[derive(Serialize)]
struct Resp {
    result: String,
}

#[post("/hello")]
async fn factory(info: web::Json<Info>) -> actix_web::Result<impl Responder> {
    let name = info.username.clone();
    let resp = Resp {
        result: format!("Hello {name}"),
    };
    Ok(web::Json(resp))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = match env::var_os("PORT") {
        None => 5000,
        Some(p) => u16::from_str(p.into_string().unwrap().as_str()).unwrap(),
    };
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    info!("Starting server on {}", &port);

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .service(web::scope("/api").service(factory).service(hello))
        // .service(hello)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
