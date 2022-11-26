use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use rust::config::get_config;
use rust::todoer::Todoer;

#[get("/")]
async fn print() -> impl Responder {
    let config = get_config(Some(std::env::current_dir().unwrap())).unwrap();
    let proj = Todoer::from_config(config);
    let value = proj.print_values();
    HttpResponse::Ok().body(value)
}

#[post("/add")]
async fn add(body: String) -> impl Responder {
    let config = get_config(Some(std::env::current_dir().unwrap())).unwrap();
    let mut proj = Todoer::from_config(config);
    proj.set_value(body);
    match proj.save() {
        Ok(()) => HttpResponse::Ok(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

#[post("/done")]
async fn complete(body: String) -> impl Responder {
    let config = get_config(Some(std::env::current_dir().unwrap())).unwrap();
    let mut proj = Todoer::from_config(config);
    proj.mark_done(body.parse().unwrap());
    match proj.save() {
        Ok(()) => HttpResponse::Ok(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

#[post("/remove")]
async fn remove(body: String) -> impl Responder {
    let config = get_config(Some(std::env::current_dir().unwrap())).unwrap();
    let mut proj = Todoer::from_config(config);
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
