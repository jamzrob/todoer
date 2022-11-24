use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use rust::{config::Config, opts::Opts, todoer::Todoer};

#[get("/")]
async fn print() -> impl Responder {
    let config: Config = Opts::parse().try_into().expect("Error parsing data");
    let proj = Todoer::from_config(config.config);
    let value = proj.print_values();
    HttpResponse::Ok().body(value)
}

#[post("/add")]
async fn add(body: String) -> impl Responder {
    let config: Config = Opts::parse().try_into().expect("Error parsing data");
    let mut proj = Todoer::from_config(config.config);
    proj.set_value(body);
    match proj.save() {
        Ok(()) => HttpResponse::Ok(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

#[post("/done")]
async fn complete(body: String) -> impl Responder {
    let config: Config = Opts::parse().try_into().expect("Error parsing data");
    let mut proj = Todoer::from_config(config.config);
    proj.mark_done(body.parse().unwrap());
    match proj.save() {
        Ok(()) => HttpResponse::Ok(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

#[post("/remove")]
async fn remove(body: String) -> impl Responder {
    let config: Config = Opts::parse().try_into().expect("Error parsing data");
    let mut proj = Todoer::from_config(config.config);
    proj.remove_value(body.parse().unwrap());
    match proj.save() {
        Ok(()) => HttpResponse::Ok(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(print)
            .service(add)
            .service(complete)
            .service(remove)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
