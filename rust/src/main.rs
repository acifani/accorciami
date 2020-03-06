extern crate env_logger;
use actix_web::{guard, http, middleware, web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};

mod base62;
mod db;
use base62::encode_in_base62;
use db::DbClient;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let redis_host = std::env::var("REDIS_HOST").unwrap_or("redis://127.0.0.1:6379/".to_string());
    let db = db::make(redis_host).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .wrap(middleware::Logger::default())
            .configure(app_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn app_config(config: &mut web::ServiceConfig) {
    config
        .service(web::resource("/").route(web::get().to(index)))
        .service(
            web::resource("/accorcia")
                .guard(guard::Header("content-type", "application/json"))
                .route(web::post().to(accorcia_json)),
        )
        .service(
            web::resource("/accorcia")
                .guard(guard::Header(
                    "content-type",
                    "application/x-www-form-urlencoded",
                ))
                .route(web::post().to(accorcia_form)),
        )
        .route("/{id}", web::get().to(redirect_to_long_url));
}

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

#[derive(Serialize, Deserialize)]
struct AccorciaParams {
    url: String,
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    status_code: i32,
    error: String,
}

#[derive(Serialize, Deserialize)]
struct AccorciaResponse {
    status_code: i32,
    short_url: String,
}

async fn accorcia_json(
    params: web::Json<AccorciaParams>,
    db: web::Data<DbClient>,
) -> Result<HttpResponse, HttpResponse> {
    accorcia_handler(&params.url, &db).await
}

async fn accorcia_form(
    params: web::Form<AccorciaParams>,
    db: web::Data<DbClient>,
) -> Result<HttpResponse, HttpResponse> {
    accorcia_handler(&params.url, &db).await
}

async fn accorcia_handler(url: &str, db: &db::DbClient) -> Result<HttpResponse, HttpResponse> {
    let next_id_result = db::get_next_id(&db).await;
    let next_id = match next_id_result {
        Ok(next_id) => next_id,
        Err(_) => {
            return Err(HttpResponse::ServiceUnavailable().json(ErrorResponse {
                status_code: 503,
                error: "We're having issues".to_string(),
            }))
        }
    };

    let url: String = if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        "http://".to_owned() + url
    };

    let short_url = encode_in_base62(next_id);
    let create_url_result = db::create_new_url(&db, &short_url, &url).await;

    match create_url_result {
        Ok(_) => Ok(HttpResponse::Ok().json(AccorciaResponse {
            status_code: 200,
            short_url: short_url.to_string(),
        })),
        Err(_) => Err(HttpResponse::ServiceUnavailable().json(ErrorResponse {
            status_code: 503,
            error: "We're having issues".to_string(),
        })),
    }
}

async fn redirect_to_long_url(
    id: web::Path<String>,
    db: web::Data<DbClient>,
) -> Result<HttpResponse, HttpResponse> {
    let long_url_res = db::get_long_url(&db, &id).await;
    let long_url = match long_url_res {
        Ok(long_url) => long_url,
        Err(_) => {
            return Err(HttpResponse::NotFound().json(ErrorResponse {
                status_code: 404,
                error: "URL not found".to_string(),
            }))
        }
    };

    let _ = db::increment_visit_counter(&db, &long_url).await;

    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, long_url)
        .finish()
        .into_body())
}
