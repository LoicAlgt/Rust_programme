use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::ServiceConfig;

fn html() -> String {
    let html = r#"
        <html>
            <body>
                <form action="/submit" method="post">
                    <input type="text" name="username">
                    <input type="text" name="password">
                    <input type="submit" value="Submit">
                </form>
            </body>
        </html>
    "#;

    html.to_owned()
}

async fn index3() -> impl Responder {
    let html_content = html();
    HttpResponse::Ok().body(html_content)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/index3", web::get().to(index3))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
