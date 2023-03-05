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

const INDEX_HTML: &str = r#"
<html>
    <head>
        <title>Username B!frost</title>
        <script src="/templates/lottie-player.js" type="text/javascript"></script>
        <link rel="stylesheet" type="text/css" href="/templates/page1.css" />
    </head>
    <style>
    lottie-player {
        width: 370px;
        height: 370px;
      }
    </style>
    <body class="body">     
    <div class="login-page">
      <div class="form">
      <div class="titre">
      <h1 align="center">SIGNUP BIFROST</h1>
        </div>
        <form id="my-form" action="/showthis2" method="post">
      <lottie-player
      src="https://assets4.lottiefiles.com/datafiles/XRVoUu3IX4sGWtiC3MPpFnJvZNq7lVWDCa8LSqgS/profile.json"
      background="transparent"
      speed="1"
      style="justify-content: center"
      loop
      autoplay
    ></lottie-player>
          <input name="thing" id="username" placeholder="&#xf007;  username" />
          <input type="submit" value="Login" style="background-color: #B22222; color: white;">
      </form>
      </div>
    </div>
  </body>
</html>
"#;

const SHOW_HTML: &str = r#"
<html>
    <head>
        <title>Username B!frost</title>
        <script>
            // Rediriger automatiquement vers la page suivante
            window.location.replace('http://127.0.0.1:8080/index3');
        </script>
    </head>
    <body>
        <h1>Showing thing:</h1>
    </body>
</html>
"#;


#[derive(Deserialize)]
struct FormData2 {
    thing: String,
}

async fn index2() -> Result<HttpResponse, MyError> {
    Ok(HttpResponse::Ok().content_type("text/html").body(INDEX_HTML))
}




#[derive(Debug, Deserialize)]
struct MyForm {
    hash: String,
    //password: String
}

async fn index3() -> impl Responder {
    let html_content = html();
    HttpResponse::Ok().body(html_content)
}

static mut THING_TO_SHOW: Option<String> = None;

struct user {
    nom: String,
}

async fn showthis2(form_data: web::Form<FormData2>) -> Result<HttpResponse, MyError> {
    let html = SHOW_HTML
        .replace("{{thing_to_show}}", &form_data.thing);
        println!("Envoie le nom d'utilisateur");
        println!("Le nom d'utilisateur est :{}",form_data.thing);
        //let path: PathBuf = "http://127.0.0.1:8080/index".parse().unwrap();
        unsafe {
            THING_TO_SHOW = Some(form_data.thing.clone()); // stockage de form_data.thing dans la variable globale THING_TO_SHOW
        }
        let nom = user { nom : String ::from(form_data.thing.to_string())};
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

//Fonction salt 
fn salt() -> String {
    let salt = "bonjour".to_owned();
    unsafe {
        if let Some(thing) = &THING_TO_SHOW {
            println!("{}", thing);
            //fonction toto
            return salt;
        }
    }
    salt
}



//page3
fn html() -> String {
    let salt = salt(); // appel de la fonction salt pour obtenir la valeur de la variable salt
    let html = format!(r#"
        <html>
        <head>
        <meta charset="UTF-8">
        <title>Conexion B!frost</title>
        <script src="/templates/lottie-player.js" type="text/javascript"></script>
        <link rel="stylesheet" type="text/css" href="/templates/page1.css" />
        </head>
        <script src="/templates/sha256.js" type="text/javascript"></script>
        <script>
            function t(){{
                if (document.getElementsByName("hash_html")[0].value == "" ) {{
                    alert("Veuillez entrer un mot de passe.");
                    return false;
                }}
                else if (document.getElementsByName("hash_html")[0].value.length < 8) {{
                    alert("Veuillez rentrer un mot de passe de plus de 8 caractères");
                    return false;
                }}
                else {{
                    var mdp = document.getElementsByName("hash_html")[0].value ;
                    var mdphash = document.getElementsByName("hash_html")[0].value + "{salt}".toString();
                    let hash = sha256(mdphash)
		            alert(hash);
                    let hash_input = document.getElementsByName("hash")[0];
                    hash_input.setAttribute("value", hash);
                    return true;
                }}
            }}


        </script>
        <style>
        lottie-player {{
            width: 370px;
            height: 370px;
          }}
        </style>
            <body class="body">
                <div class="login-page">
                    <div class="form">
                        <div class="titre">
                            <h1 align="center">SIGNUP BIFROST</h1>
                        </div>


                <form action="/submit" method="post" onsubmit="return t();">
                <lottie-player
                src="https://assets4.lottiefiles.com/datafiles/XRVoUu3IX4sGWtiC3MPpFnJvZNq7lVWDCa8LSqgS/profile.json"
                background="transparent"
                speed="1"
                style="justify-content: center"
                loop
                autoplay
              ></lottie-player>
                    <input type="text" name="hash_html" placeholder="&#xf023;  mdp">
                    <input type="hidden" name="hash" >
                    <input type="submit" value="Login" style="background-color: #B22222; color: white;">
                </form>
                </div>
    </div>
            </body>
        </html>
    "#, salt=salt);

    html
}

fn var(salt: &str) {
    unsafe {
        if let Some(thing_to_show) = &THING_TO_SHOW {
            println!("Le sel html est: {}", salt);
            println!("Le nom d'utilisateur est: {}", thing_to_show);
        }
    }
}


async fn submit_form(form: web::Form<MyForm>) -> impl Responder {
    let hash = &form.hash;
    //let password = &form.password;
    println!("Le hash html est: {}", hash);
    let salt_value = salt(); 
    //recuperation var avec la fonction var
    var(&salt_value);
    //println!("{}", password);
    HttpResponse::Found().header("LOCATION", "/templates/menu1.html").finish()
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
        //.data(salt()).route("/index", web::get().to(index))
        .route("/index2", web::get().to(index2))
        .route("/showthis2", web::post().to(showthis2))
        .route("/index3", web::get().to(index3))
        .route("/submit", web::post().to(submit_form))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}