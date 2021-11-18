use std::env;
use actix_web::{web, App, HttpServer, http};
use actix_session::CookieSession;
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::middleware;
use actix_cors::Cors;
use sqlx::sqlite::SqlitePool;
// use apparch::core;

pub mod template;
pub mod models;
pub mod route;
pub mod state;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // logging
    std::env::set_var("RUST_LOG", "actix_web=warn");
    log4rs::init_file("log/config.yaml", Default::default()).unwrap();
    
    let address = "0.0.0.0";
    let port = "8080";
    let server_root = format!("{}:{}", address, port);

    let db_pool = SqlitePool::connect(":memory:").await.unwrap();
    let mut conn = db_pool.acquire().await.unwrap();

    sqlx::query(r#"
CREATE TABLE IF NOT EXISTS todos
(
    id          INTEGER PRIMARY KEY NOT NULL,
    description TEXT                NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT 0
);
    "#)
    .execute(&db_pool)
    .await
    .unwrap();
    log::info!(target: "app::backend::db","[INFO] SQLite Database inititialized");
    
    println!("[Http Server]: http://127.0.0.1:8080");
    log::info!("[INFO] starting server http://{}", server_root);

    HttpServer::new(move || {
        App::new()
            .data(
                state::Application {
                    title: String::from("BitFields"),
                    version: String::from("0.0.1"),
                }
            )
            .wrap(
                CookieSession::signed(&[0; 32]).secure(true),
            )
            .wrap(
                middleware::Logger::new("%a %{User-Agent}i")
            ).wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)
            ).wrap(
                middleware::Compress::default()
            )
            .wrap(
                ErrorHandlers::new().handler(
                    http::StatusCode::INTERNAL_SERVER_ERROR, route::handlers::error::error_500
                )
            ).wrap(
                ErrorHandlers::new().handler(
                    http::StatusCode::NOT_FOUND, route::handlers::error::error_404
                )
            )
            .service(
                actix_files::Files::new("/assets", "./assets")
                .use_last_modified(true)
            )
            .route("/", web::get().to(route::handlers::http::index))
            .route("/about", web::get().to(route::handlers::http::about))
    })
    .bind(server_root)?
    .run()
    .await
}
