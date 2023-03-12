use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::io;
use serde::{Deserialize};
use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Test {
    nom: String,
    auto: String,
}

async fn index() -> io::Result<impl Responder> {
    let html = include_str!("index.html");
    Ok(HttpResponse::Ok().body(html))
}

#[derive(Deserialize)]
struct FormData {
    name: String,
}

async fn on_submit_form(form_data: web::Form<FormData>) -> /*impl Responder*/std::result::Result<HttpResponse, Box<dyn std::error::Error>> {
    let value= form_data.name.to_string();
    println!("Nom: {}", value);
    let url = "mysql://aimasu:BIFROST@localhost:3306/autorisation";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    let query = format!("SELECT * FROM credential WHERE Nom = '{}'", value);
    /*let _test = vec![
        Password {nom: value  , auto: ?},
    ];*/
    let selected_passwords = conn
    	.query_map(
            &query,
            |(nom, auto)| {
                Test { nom, auto}
            },
        )?;

        let mut log_extrait = "".to_string();

        for burnout in selected_passwords {
        log_extrait.push_str(&burnout.auto);

        }


        println!("{}", log_extrait);

        Ok(HttpResponse::Ok().content_type("text/html").body("super"))
}


#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index))
                                      .route(web::post().to(on_submit_form)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
