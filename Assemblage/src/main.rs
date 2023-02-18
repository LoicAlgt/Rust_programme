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
use actix_web::web::ServiceConfig;



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


#[get("templates/menu1")]
async fn menu1(req: HttpRequest) -> Result<HttpResponse> {
    println!("{req:?}");
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(include_str!("../templates/menu1.html")))
}

//Connexion
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


async fn index(data: web::Data<String>) -> HttpResponse {
    let html = format!(r#"
    <html>
        <head>
            <script>
                var rust_variable = '{}';
            </script>
        </head>
        <body>
            <h1>Rust variable in JavaScript</h1>
            <p>Value of the rust_variable is: <span id='rust_variable_value'></span></p>
            <script>
                document.getElementById('rust_variable_value').innerHTML = rust_variable;                   
            </script>
            <button onclick="window.location.href='http://127.0.0.1:8080/index3'">ok</button>
        </body>
    </html>
"#, data.get_ref());
    HttpResponse::Ok().content_type("text/html").body(html)
}

async fn index3() -> impl Responder {
    let html_content = html();
    HttpResponse::Ok().body(html_content)
}

async fn submit_form(form: web::Form<MyForm>) -> impl Responder {
    let username = &form.username;
    let password = &form.password;
    println!("{}", username);
    println!("{}", password);
    HttpResponse::Found().header("LOCATION", "/templates/menu1.html").finish()
}

fn static_files_config(config: &mut ServiceConfig) {
    config.service(
        Files::new("/templates", "./templates")
            .index_file("menu1.html")
    );
}


#[derive(Debug, Deserialize)]
struct MyForm {
    username: String,
    password: String,
}

async fn index2() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(r#"
        <html>
            <head>
                <title>Page d'accueil</title>
            </head>
            <body>
                <h1>Bienvenue !</h1>
                <input type="text" id="username" placeholder="&#xf007;  username"/>
                <button onclick="window.location.href='http://127.0.0.1:8080/index'">ok</button>
            </body>
        </html>
    "#)
}


//Fonction salt 
fn salt() -> String {
    let salt = "bonjour".to_owned();
    salt
}


#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .service(menu1) 
         // static files
        .service(Files::new("/templates", "templates").show_files_listing())
            // redirect
        .service(
                web::resource("/templates/login.html").route(web::get().to(|req: HttpRequest| async move {
                    //println!("{req:?}");
                    HttpResponse::Found()
                        .insert_header((header::LOCATION, "templates/login.html"))
                        .finish()
                })),
            )
            // default
        .default_service(web::to(default_handler))
        .route("/showthis", web::post().to(showthis))
        .data(salt()).route("/index", web::get().to(index))
        .configure(static_files_config)
        .route("/index3", web::get().to(index3))
        .route("/submit", web::post().to(submit_form))
        .route("/", web::get().to(index2))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}