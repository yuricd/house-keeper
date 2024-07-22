use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;

mod division;
mod repositories;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(home_status)
            .service(home_clean)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/divisions/status")]
async fn home_status() -> Result<impl Responder, actix_web::Error> {
    let divisions = repositories::DivisionRepo::get_divisions();
    Ok(web::Json(divisions))
}

#[derive(Serialize)]
struct CleanResponse {
    result: bool,
}

#[get("/divisions/{division_name}/clean")]
async fn home_clean(division_name: web::Path<String>) -> Result<impl Responder, actix_web::Error> {
    let res = repositories::DivisionRepo::update(division_name.to_string());
    Ok(web::Json(CleanResponse { result: res }))
}
