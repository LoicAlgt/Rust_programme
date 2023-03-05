use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mysql::{Pool, prelude::*};

fn html(result: Vec<(String, String)>) -> String {
    let mut html = String::new();
    html.push_str("<html><body><table>");

    for (name, surname) in result {
        html.push_str("<tr><td>");
        html.push_str(&name);
        html.push_str("</td><td>");
        html.push_str(&surname);
        html.push_str("</td></tr>");
    }

    html.push_str("</table></body></html>");
    html
}

async fn index3(db: web::Data<Pool>) -> impl Responder {
    let mut conn = db.get_conn().unwrap();

    let query = "SELECT name, surname FROM loic";
    let result = conn.query_map(query, |(name, surname)| (name, surname)).unwrap();

    let html_content = html(result);
    HttpResponse::Ok().body(html_content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = "mysql://username:password@localhost:3306/database_name";
    let db_pool = Pool::new(db_url).unwrap();
    let db_data = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .route("/index3", web::get().to(index3))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
