//Dépendances pour la page web 
use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::{get, Result};
use std::io;
use actix_files::NamedFile;
use serde::{Deserialize};
use std::path::PathBuf;


//Dépendances pour le mail
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

//Dépendance pour le nombre aléatoire
use rand::Rng;

fn nombre_aleatoire() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=999999)
}

#[get("/{filename:.*}")]
async fn files(path: web::Path<(String,)>) -> Result<NamedFile> {
    Ok(NamedFile::open(format!("templates/{}", path.0))?)
}

#[derive(Deserialize)]
struct Information {
    name: String,
    surname : String,
    adresse : String,
}

//Déclaration variable globale
static mut GLOBAL_VAR: Option<u32> = None;

async fn info_mail(form_data: web::Form<Information>) -> /*impl Responder*/std::result::Result<HttpResponse, Box<dyn std::error::Error>> {
    let nom= form_data.name.to_string();
    println!("Nom: {}", nom);
    let prenom= form_data.surname.to_string();
    println!("Prénom: {}", prenom);
    let adresse= form_data.adresse.to_string();
    println!("Adresse mail: {}", adresse);

    let number = nombre_aleatoire();
    println!("Le nombre aléatoire est : {}", number);

    //Placer number dans une variable globale
    unsafe {
        GLOBAL_VAR = Some(number);
    }
    println!("Le nombre aléatoire est : {}", number);

    //Code pour envoyer le mail
    //let email = EmailBuilder::new()
    //.to(adresse)
    //.from("loic.allegretti@mailfence.com")
    //.subject("Example subject")
    //.text(number.to_string())
    //.build()
    //.unwrap();

    //let mut mailer = SmtpClient::new_simple("imap.mailfence.com")
    //    .unwrap()
    //    .credentials(Credentials::new("loic.allegretti".into(), "loic1234!!".into()))
    //    .transport();

    //let result = mailer.send(email.into());
    //println!("{:?}", result);

    Ok(HttpResponse::Ok().content_type("text/html").body("super"))
}

#[derive(Deserialize)]
struct Verif {
    code: String,
}

async fn on_submit_form(form_data: web::Form<Verif>) -> std::result::Result<HttpResponse, Box<dyn std::error::Error>> {
    let value = form_data.code.to_string();
    println!("Code: {}", value); 

    // Utilisation de la variable stockée globalement   
    let global_val = unsafe { GLOBAL_VAR };
    match global_val {
        Some(val) => {
            let val_str = val.to_string();
            if val_str == value {
                println!("La valeur globale est égale à la valeur du formulaire.");
            } else {
                println!("La valeur globale est différente de la valeur du formulaire.");
                
                //Faire la gestion d'erreur 
            }
        },
        None => println!("Aucune valeur n'a été stockée dans la variable globale."),
    }
    Ok(HttpResponse::Ok().content_type("text/html").body("super"))
}



#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(files)
            .service(web::resource("/").route(web::post().to(info_mail)))
            .service(web::resource("/verif.html").route(web::post().to(on_submit_form)))     
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
