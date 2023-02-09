use std::{io};
use actix_files::{Files, NamedFile};
use serde::{Deserialize};
use askama::Template;
use std::path::{Path, PathBuf};
use actix_web::{
    get,
    http::{
        header::{self, ContentType},
        Method, StatusCode,
    },
    web, App, Either, HttpRequest, HttpResponse, HttpServer, Responder, Result,ResponseError
};


#[derive(Debug)] // Macro qui implémente l'erreur
pub struct MyError(String); // <-- needs debug and display
 impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A validation error occured on the input.")
    }
} 

impl ResponseError for MyError {} // <-- key // je crée l'instance erreur avec la macro du dessus

#[derive(Template)] //déclaration de la 1ére template html (le formulaire)
#[template(path = "index.html")]
struct Index {}

#[derive(Template)] // déclaration de la 2éme template html (affichage de la variable)
#[template(path = "show.html")]
struct Show {
    thing_to_show: String,
    thing_to_show2: String,
    thing_to_show3: String
}

#[derive(Deserialize)] // pour adapter la donnée
struct FormData {
    thing_to_show:String,
    thing_to_show2:String,
    thing_to_show3: String
}

/// simple index handler
/*#[get("/index")]
async fn SignUp(req: HttpRequest) -> Result<HttpResponse> {
    println!("{req:?}");
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(include_str!("../templates/index.html")))
}*/


async fn default_handler(req_method: Method) -> Result<impl Responder> {
    match req_method {
        Method::GET => {
            let file = NamedFile::open("templates/404.html")?
                .customize()
                .with_status(StatusCode::NOT_FOUND);
            Ok(Either::Left(file))
        }
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}

/* FONCTION EN PLUS DE THOMAS */
async fn showthis(form_data: web::Form<FormData>) -> Result<NamedFile> { //fonction pour afficher le 2éme rendu html
    let html = Show{ thing_to_show: form_data.thing_to_show.to_string(),thing_to_show2: form_data.thing_to_show2.to_string(),thing_to_show3: form_data.thing_to_show3.to_string()}.render().unwrap();
    println!("{}",html);
    let path: PathBuf = "templates/menushowthis.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

/*
#[get("templates/marre")]
async fn marre(req: HttpRequest) -> Result<HttpResponse> {
    println!("{req:?}");
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(include_str!("../templates/marre.html")))
}

#[get("templates/marre2")]
async fn marre2(req: HttpRequest) -> Result<HttpResponse> {
    println!("{req:?}");
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(include_str!("../templates/marre2.html")))
}#[get("templates/menu")]
async fn menu(req: HttpRequest) -> Result<HttpResponse> {
    println!("{req:?}");
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(include_str!("../templates/menu.html")))
}

#[get("templates/page3")]
async fn page3(req: HttpRequest) -> Result<HttpResponse> {
    println!("{req:?}");
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(include_str!("../templates/page3.html")))
}

#[get("templates/contact")]
async fn contact(req: HttpRequest) -> Result<HttpResponse> {
    println!("{req:?}");
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(include_str!("../templates/contact.html")))
}

#[get("templates/bifrost")]
async fn bifrost(req: HttpRequest) -> Result<HttpResponse> {
    println!("{req:?}");
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(include_str!("../templates/bifrost.html")))
}*/

#[get("templates/menu1")]
async fn menu1(req: HttpRequest) -> Result<HttpResponse> {
    println!("{req:?}");
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(include_str!("../templates/menu1.html")))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .service(menu1)
        /* .service(SignUp) 
        .service(contact)
        .service(bifrost)
        .service(marre)
        .service(marre2)
        .service(menu)
        //.service(page3)  */     
         // static files
        .service(Files::new("/templates", "templates").show_files_listing())
            // redirect
        .service(
                web::resource("/").route(web::get().to(|req: HttpRequest| async move {
                    //println!("{req:?}");
                    HttpResponse::Found()
                        .insert_header((header::LOCATION, "templates/login.html"))
                        .finish()
                })),
            )
            // default
        .default_service(web::to(default_handler))
        .route("/showthis", web::post().to(showthis))
    
    })
    .bind(("127.0.0.1", 1001))?
    .workers(2)
    .run()
    .await
}