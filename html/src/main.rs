use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::io;
use serde::{Deserialize};
use mysql::*;
use mysql::prelude::*;
use actix_files::{Files, NamedFile};

async fn index() -> io::Result<impl Responder> {
    let html = include_str!("../templates/Marre.html");
    Ok(HttpResponse::Ok().body(html))
}

#[derive(Deserialize)]
struct FormData {
    name: String,
    surname : String,
}

async fn on_submit_form(form_data: web::Form<FormData>) -> /*impl Responder*/std::result::Result<HttpResponse, Box<dyn std::error::Error>> {
    let nom= form_data.name.to_string();
    println!("Nom: {}", nom);
    let prenom= form_data.surname.to_string();
    println!("Prenom: {}", prenom);
    
    Ok(HttpResponse::Ok().content_type("text/html").body("super"))
}


#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(Files::new("/templates", "templates").show_files_listing())
        .service(web::resource("/").route(web::get().to(index))
                                      .route(web::post().to(on_submit_form)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
