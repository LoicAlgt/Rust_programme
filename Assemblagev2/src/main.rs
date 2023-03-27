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
    web, App, Either, HttpRequest, HttpServer, Responder, Result,ResponseError
};

////////////////use de thomas/////////////////
use mysql::*;
use mysql::prelude::*;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use actix_web::HttpResponse;
use std::fmt::Debug;
use std::fmt;
use rand::distributions::DistString;
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce // Or `Aes128Gcm`
};
use argon2::{self, Config};
use generic_array::{GenericArray, sequence::GenericSequence};

////////////////////////////////////////////////////

use std::future::Future;


////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq)]
struct Password {
    sel_1: String,
    sel_2: String,
    sel_gcm: String,
    clefs: String,
    login: String,
    passw: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Salo {
	labaleine: String,
}



//fonction de hash
impl Hash for Password {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.passw.hash(state);
    }
}



fn salt() -> String {
let mut rng = thread_rng();
let _x: u32 = rng.gen();


let s: String = (&mut rng).sample_iter(Alphanumeric)
    .take(15)
    .map(char::from)
    .collect();
return s;
}



fn passwordhash(a:String ,b: String) -> String {
let password = b.as_bytes();
let salt = a.as_bytes();
let config = Config::default();
let encoded_hash = argon2::hash_encoded(password, salt, &config).unwrap();
let hash = encoded_hash.as_str();
println!("ceci est le hash argon2: {}", hash);
return hash.to_string();
}

fn chiffrement (entree: String, sal: String) -> (String , String) {
let key = Aes256Gcm::generate_key(&mut OsRng);

let mut keystring= "".to_owned();
let vir = " ";
for a in key{
    let cle = a.to_string();
    keystring = keystring + &cle + vir ;
}

let cipher = Aes256Gcm::new(&key);
let mut valeurs   = Vec::new();
let mut crypt ="".to_owned();

//Mettre un sel dans la variable
let salt = sal.as_bytes();
let nonce = Nonce::from_slice(salt); // 96-bits; unique per message

//Mettre l'entréé à chiffrer dans la variable 
let preplain = entree;
let plaintext = preplain.as_bytes();
let ciphertext =cipher.encrypt(nonce, plaintext.as_ref());

match ciphertext{
    Ok(n) => valeurs=n,
    Err(..) => {}
}

for numbers in valeurs{
    let texte = numbers.to_string();
    crypt=crypt+&texte;
}
//La sortie est crypt, il s'agit d'un string correspondant à une suite de chiffre 
println!("{}", crypt);
return (crypt,keystring);
}


async fn bdd_create(form_data: web::Form<FormData>) -> std::result::Result<HttpResponse, Box<dyn std::error::Error>>{
    println!("salut les copains");
    let url = "mysql://GAGA:mypass@localhost:3306/passwd";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    let hashlolo = form_data.thing_to_show.to_string(); 
    println!("ceci est le hash lolo create: {}", hashlolo);   
    let log = form_data.thing_to_show2.to_string();    
    let sellolo= form_data.thing_to_show3.to_string();
    let y= salt();
    let sel= y.clone();
    let concat = hashlolo + &y.to_string();
    let x = passwordhash(y, concat);
    let presalt = Alphanumeric.sample_string(&mut rand::thread_rng(), 12);
    let salt_gcm = presalt.clone();
    let (aes , key) = chiffrement(x , presalt);
    println!("{} chiffrement aes:", aes);
    println!("{} la clefffffffff", key);
  	
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS password (
            sel_1 text not null,
            sel_2 text not null,
            sel_gcm text not null,
            clefs text not null,
            login text not null,
            password text not null
        )")?;
    let _passwords = vec![
        Password { sel_1: sellolo  , sel_2:sel , sel_gcm:salt_gcm , clefs:key,  login: log , passw: aes },
    ];


    conn.exec_batch(
        r"INSERT INTO password (sel_1, sel_2, sel_gcm, clefs, login, password)
          VALUES (:sel_1, :sel_2, :sel_gcm, :clefs, :login, :password)",
        _passwords.iter().map(|p| params! {
            "sel_1" => &p.sel_1,
            "sel_2" => &p.sel_2,
            "sel_gcm" => &p.sel_gcm,
            "clefs" => &p.clefs,
            "login" => &p.login,
            "password" => &p.passw,
        })
    )?;
Ok(HttpResponse::Ok().content_type("text/html").body("super"))
 
 }
 
//////////////////////////////////////////

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
    let response = bdd_create(form_data).await?; // appel de la fonction bdd_create
    println!("{:?}", response.body());
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

fn dechiffrement (ki: String , entree: String, sal: String) -> String {
let result: Vec<&str> = ki.split(" ").collect();
let mut array = GenericArray::generate(|i: usize| i as u8);
    let mut i = 0;
    for a in result{
    	if i<32{
    		let my_string = a.to_string();
    		let my_int= my_string.parse::<u8>().unwrap();
    		array[i] = my_int;
    		i = i+1;
    	}
    }
    
let cipher = Aes256Gcm::new(&array);
let mut valeurs   = Vec::new();
let mut crypt ="".to_owned();

//Mettre un sel dans la variable
let salt = sal.as_bytes();
let nonce = Nonce::from_slice(salt); // 96-bits; unique per message

//Mettre l'entréé à chiffrer dans la variable 
let preplain = entree;
let plaintext = preplain.as_bytes();
let ciphertext =cipher.encrypt(nonce, plaintext.as_ref());

match ciphertext{
    Ok(n) => valeurs=n,
    Err(..) => {}
}

for numbers in valeurs{
    let texte = numbers.to_string();
    crypt=crypt+&texte;
}
//La sortie est crypt, il s'agit d'un string correspondant à une suite de chiffre 
println!("{}", crypt);
return crypt;
}


#[derive(Debug, Deserialize)]
struct MyForm {
    hash: String,
    //password: String
}

async fn index3() -> impl Responder {
    let value = unsafe { THING_TO_SHOW.clone() }; // call the unsafe function to obtain the value of SEL_HTML
    let valuebis = value.unwrap_or_default();
    println!("Nom: {}", valuebis);
    println!("Hello world");
    let url = "mysql://GAGA:mypass@localhost:3306/passwd";
    let pool = match Pool::new(url) {
        Ok(pool) => pool,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error connecting to the database: {}", e));
        }
    };
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error getting a connection from the pool: {}", e));
        }
    };
    let query = format!("SELECT * FROM password WHERE login = '{}'", valuebis);

    let selected_passwords = match conn.query_map(
        &query,
        |(sel_1, sel_2, sel_gcm, clefs, login, passw)| {
            Password {
                sel_1,
                sel_2,
                sel_gcm,
                clefs,
                login,
                passw,
            }
        },
    ) {
        Ok(results) => results,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error executing the query: {}", e));
        }
    };

    let mut log_extrait = "".to_string();

    for burnout in selected_passwords {
        log_extrait.push_str(&burnout.sel_1);
    }

    unsafe {
        SEL_HTML = Some(log_extrait.clone());
        println!("I'm the SEL :{:?}", SEL_HTML);
    }

    let html_content = html();
    HttpResponse::Ok().body(html_content)
}


static mut THING_TO_SHOW: Option<String> = None;


async fn showthis2(form_data: web::Form<FormData2>) -> Result<HttpResponse, MyError> {
    let html = SHOW_HTML
        .replace("{{thing_to_show}}", &form_data.thing);
        println!("Envoie le nom d'utilisateur");
        println!("Le nom d'utilisateur est :{}",form_data.thing);
        //let path: PathBuf = "http://127.0.0.1:8080/index%22.parse().unwrap();
        unsafe {
            THING_TO_SHOW = Some(form_data.thing.clone()); // stockage de form_data.thing dans la variable globale THING_TO_SHOW
        }
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}


/*
//Fonction salt 
async fn selhtml() -> String {
    let mut log_extrait_sel_web = "bonjour".to_owned();
use std::future::Future;
    // Appelle de la fonction bdd_authentification pour obtenir log_extrait_sel_web
    match bdd_authentification("thomas".to_owned()).await {
        Ok((_, log_extrait)) => {
            log_extrait_sel_web = log_extrait;
        },
        Err(e) => {
            println!("Erreur lors de l'appel de bdd_authentification : {}", e);
        }
    };

    // Vérification de la variable THING_TO_SHOW
    unsafe {
        if let Some(thing) = &THING_TO_SHOW {
            println!("{}", thing);
            return log_extrait_sel_web;
        }
    }
    log_extrait_sel_web
}
*/
static mut SEL_HTML: Option<String> = None;

async fn on_submit_form(form: web::Form<MyForm>) -> /*impl Responder*/std::result::Result<HttpResponse, Box<dyn std::error::Error>> {
  let value = unsafe { THING_TO_SHOW.clone() }; // call the unsafe function to obtain the value of SEL_HTML
  let valuebis = value.unwrap_or_default();
   // println!("Nom: {}", valuebis);
    //println!("Hello world");
    let url = "mysql://GAGA:mypass@localhost:3306/passwd";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    let query = format!("SELECT * FROM password WHERE login = '{}'", valuebis);

    /*let selected_passwords = conn
        .query_map(
            &query,
            |(sel_1, sel_2, sel_gcm, clefs, login, passw)| {
                Password { sel_1, sel_2, sel_gcm, clefs, login, passw }
            },
        )?;

    let mut log_extrait = "".to_string();

    for burnout in selected_passwords {
        log_extrait.push_str(&burnout.sel_1);
    }
    
unsafe {
    SEL_HTML = Some(log_extrait.clone());
    println!("I'm the SEL :{:?}", SEL_HTML);
}
*/
	
	println!("Test");

    let hash = &form.hash;
        let selected_passwords2 = conn
        .query_map(
            &query,
            |(sel_1, sel_2, sel_gcm, clefs, login, passw)| {
                Password { sel_1, sel_2, sel_gcm, clefs, login, passw }
            },
        )?;
            let selected_passwords3 = conn
        .query_map(
            &query,
            |(sel_1, sel_2, sel_gcm, clefs, login, passw)| {
                Password { sel_1, sel_2, sel_gcm, clefs, login, passw }
            },
        )?;
            let selected_passwords4 = conn
        .query_map(
            &query,
            |(sel_1, sel_2, sel_gcm, clefs, login, passw)| {
                Password { sel_1, sel_2, sel_gcm, clefs, login, passw }
            },
        )?;
            let selected_passwords5 = conn
        .query_map(
            &query,
            |(sel_1, sel_2, sel_gcm, clefs, login, passw)| {
                Password { sel_1, sel_2, sel_gcm, clefs, login, passw }
            },
        )?;
        
        
let mut log_extrait_sel_backend = "".to_string();
let mut log_extrait_password = "".to_string();
let mut log_extrait_sel_gcm = "".to_string();
let mut log_extrait_clefs_aes = "".to_string();



for salting in selected_passwords2 {
log_extrait_sel_backend.push_str(&salting.sel_2);
}

for motdepasse in selected_passwords3 {
log_extrait_password.push_str(&motdepasse.passw);
}

for gcmsuite in selected_passwords4 {
log_extrait_sel_gcm.push_str(&gcmsuite.sel_gcm);
}

for aesfinit in selected_passwords5 {
log_extrait_clefs_aes.push_str(&aesfinit.clefs);
}

    println!("I'm the hashLOLO :{:?}", hash);
    println!("ceci est le sel 2: {}", log_extrait_sel_backend);
println!("ceci est le password: {}", log_extrait_password);
println!("ceci est le sel-gcm: {}", log_extrait_sel_gcm);
println!("ceci est la clef-gcm: {}", log_extrait_clefs_aes);
let y= log_extrait_sel_backend;
let concat = hash.to_owned() + &y.to_string();
let x = passwordhash(y ,concat);
let aes = dechiffrement(log_extrait_clefs_aes, x , log_extrait_sel_gcm); 
    
    
    
    println!("ceci est le password entrée: {}", aes);   
        
    
    assert_eq!(log_extrait_password, aes);
    //HttpResponse::Found().header("LOCATION", "/templates/menu1.html").finish()
    Ok(HttpResponse::Found().header("LOCATION", "/templates/menu1.html").finish())
}



//page3
fn html() -> String {
    let sellallegreti = unsafe { SEL_HTML.clone() }; // call the unsafe function to obtain the value of SEL_HTML
    let sellallegreti_str = sellallegreti.unwrap_or_default();
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
                    var mdphash = document.getElementsByName("hash_html")[0].value + "{sellallegreti_str}".toString();
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
    "#, sellallegreti_str=sellallegreti_str);
    println!("I'm the SELLE :{:?}", sellallegreti_str);

    html
}

async fn submit_form(form: web::Form<MyForm>) -> impl Responder {
    let hash = &form.hash;
    //let password = &form.password;
    println!("Le hash html est: {}", hash);
    let salt_value = "salut".to_string(); 
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
        .route("/submit", web::post().to(on_submit_form))
        .route("/index3", web::get().to(index3))
        .route("/submit", web::post().to(submit_form))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
