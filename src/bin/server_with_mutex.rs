use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use rust::config::get_config;
use rust::todoer::Todoer;

#[get("/")]
async fn print(data: web::Data<Mutex<Todoer>>) -> impl Responder {
    let value = data.lock().unwrap().print_values();
    HttpResponse::Ok().body(value)
}

#[post("/add")]
async fn add(data: web::Data<Mutex<Todoer>>, body: String) -> impl Responder {
    let mut data = data.lock().unwrap();
    data.set_value(body);
    match data.save() {
        Ok(()) => HttpResponse::Ok(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

#[post("/done")]
async fn complete(data: web::Data<Mutex<Todoer>>, body: String) -> impl Responder {
    let mut data = data.lock().unwrap();
    data.mark_done(body.parse().unwrap());
    match data.save() {
        Ok(()) => HttpResponse::Ok(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

#[post("/remove")]
async fn remove(data: web::Data<Mutex<Todoer>>, body: String) -> impl Responder {
    let mut data = data.lock().unwrap();
    data.remove_value(body.parse().unwrap());
    match data.save() {
        Ok(()) => HttpResponse::Ok(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_config(Some(std::env::current_dir().unwrap())).unwrap();

    let data = web::Data::new(Mutex::new(Todoer::from_config(config)));
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(print)
            .service(add)
            .service(complete)
            .service(remove)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// (|| App::new().app_data(web::Data::new(&proj)).service(print))
