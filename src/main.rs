use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use chrono::Utc;
use division::{CleanFrequency, Division};

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

#[get("/status")]
async fn home_status() -> Result<impl Responder, actix_web::Error> {
    let divisions = repositories::DivisionRepo::get_divisions();
    Ok(web::Json(divisions))
}

#[get("/division/{division_id}/clean")]
async fn home_clean(req: HttpRequest) -> impl Responder {
    let division_id: String = req
        .match_info()
        .get("division_id")
        .unwrap()
        .parse()
        .unwrap();

    HttpResponse::Ok().body(format!("Clean division {}", division_id))
}
