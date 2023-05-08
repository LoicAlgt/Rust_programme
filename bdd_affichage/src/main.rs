use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::io;
use mysql::*;
use mysql::prelude::*;
use actix_files::{Files};
use serde::{Deserialize};

#[derive(Deserialize)]
struct Credential {
    name: String,
    value : String,
}

//////////////////////////////////////////PAGE BDD///////////////////////////////////////////////////////////

fn see_bdd(result: Vec<(String,String)>) -> String {
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <style>
        table {{
            border-collapse: collapse;
            width: 100%;
        }}
        th, td {{
            text-align: left;
            padding: 8px;
        }}
        th {{
            background-color: #4CAF50;
            color: white;
        }}
        tr:nth-child(even) {{
            background-color: #f2f2f2;
        }}
    </style>
    
            <head>
                <meta charset="UTF-8">
                <title>BDD USER Bifrost</title>
                <link rel="stylesheet" type="text/css" href="templates/marre.css">
                <style>
                    table {{
                        border-collapse: collapse;
                    }}
                    table, th, td {{
                        border: 1px solid black;
                    }}
                    caption {{
                        text-align: center;
                    }}
                </style>
            </head>
            <body>
                <caption>Liste des nom et value</caption>
                <input type="button" onclick="window.location.href='http://127.0.0.1:8080/ADD'" value="Ajouter User">
                <input type="button" onclick="window.location.href='http://127.0.0.1:8080/delete'" value="Supprimer User">
                <input type="button" onclick="window.location.href='http://127.0.0.1:8080/modifier'" value="Modifier User">
                <table>
                    <tr>
                        <th>Nom</th>
                        <th>Value</th>
                    </tr>
                    {}
                </table>
                <script>
                    // Bloquer l'accès à la flèche retour
                    window.history.pushState(null, null, '');
                    window.addEventListener('popstate', function (event) {{
                        window.history.pushState(null, null, '');
                    }});
                    
                    // Bloquer l'accès à une nouvelle URL
                    const url = "http://127.0.0.1:8080/ADD";
                    if (window.location.href === url) {{
                    console.log("test")
                    window.location.replace("http://127.0.0.1:8080/BDD"); // rediriger l'utilisateur vers une page d'erreur
                    }}
                </script>
            </body>
        </html>
        "#,
        result
        .into_iter()
        .map(|(nom,value)| { // utilise la première valeur du tuple (le login)
            format!(
                r#"
                    <tr>
                        <td>{}</td>
                        <td>{}</td>
                    </tr>
                "#,
                nom,value
            )
        })
        .collect::<String>()
    );

    html
}



async fn bdd(db: web::Data<Pool>) -> impl Responder {
    let mut conn = db.get_conn().unwrap();

    let query = "SELECT nom , value FROM credential";
    let result = conn.query_map(query, |(nom, value)| (nom, value)).unwrap();
    let html_content = see_bdd(result);
    HttpResponse::Ok().body(html_content)
}

//////////////////////////////////////////ADD/////////////////////////////////////////////////////////////

fn ajouter_credential() -> &'static str {
    "<!DOCTYPE html>
    <html>
        <head>
            <meta charset=\"UTF-8\">
            <title>BDD Bifrost</title>
        </head>
        <body>
        <h1>Ajouter des credential</h1>
        <form id=\"my-form\">
            <label for=\"name\">Nom:</label>
            <input type=\"text\" id=\"name\" name=\"name\">
            <label for=\"value\">Value:</label>
            <input type=\"text\" id=\"value\" name=\"value\">
            <button type=\"submit\">Ajouter</button>


        </form>
            <script>
                const form = document.getElementById('my-form');
                form.addEventListener('submit', (event) => {
                    event.preventDefault();
                    const formData = new FormData(form);
                    fetch('/ADD', {
                        method: 'POST',
                        headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
                                 },
                        body: new URLSearchParams(formData)
    
                    })
                    .then(response => {
                        // traitement de la réponse ici
                        console.log(response.text());
                        // redirection de l'utilisateur
                        window.location.href = 'http://127.0.0.1:8080/BDD';
                    })
                    .catch(error => console.error(error));
                });
        
            </script>
        </body>
    </html>"
}

async fn add_credential() -> io::Result<impl Responder> {
    let html = ajouter_credential();
    Ok(HttpResponse::Ok().body(html))
}


async fn ajout_cred(db: web::Data<Pool>, form_data: web::Form<Credential>) -> HttpResponse {
    let nom = form_data.name.to_string();
    println!("Nom: {}", nom);
    let value = form_data.value.to_string();
    println!("Value: {}", value);
    
    let mut conn = db.get_conn().unwrap();
    conn.exec_drop(
        r"INSERT INTO credential (nom, value) VALUES (:nom, :value)",
        params! {
            "nom" => nom,
            "value" => value,
        },
    ).unwrap();

    HttpResponse::Found()
        .header("Location", "/")
        .finish()
}

///////////////////////////////////////////SUPPRIMER ///////////////////////////////////////////////////////////

fn supprimer_credential() -> &'static str {
    "<!DOCTYPE html>
    <html>
        <head>
            <meta charset=\"UTF-8\">
            <title>Supprimer des credentials</title>
        </head>
        <body>
        <h1>Supprimer des credentials</h1>
        <form id=\"my-form\">
            <label for=\"name\">Nom:</label>
            <input type=\"text\" id=\"name\" name=\"name\">
            <label for=\"value\">Value:</label>
            <input type=\"text\" id=\"value\" name=\"value\">
            <button type=\"submit\">Supprimer</button>
            


        </form>
            <script>
                const form = document.getElementById('my-form');
                form.addEventListener('submit', (event) => {
                    event.preventDefault();
                    const formData = new FormData(form);
                    fetch('/delete', {
                        method: 'POST',
                        headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
                                 },
                        body: new URLSearchParams(formData)
    
                    })
                .then(response => {
                    // traitement de la réponse ici
                    console.log(response.text());
                    // redirection de l'utilisateur
                    window.location.href = 'http://127.0.0.1:8080/BDD';
                })
                .catch(error => console.error(error));
            });
        
            </script>
        </body>
    </html>"
}

async fn delete_credential() -> io::Result<impl Responder> {
    let html = supprimer_credential();
    Ok(HttpResponse::Ok().body(html))
}


async fn delete_cred(db: web::Data<Pool>, form_data: web::Form<Credential>) -> HttpResponse {
    let nom = form_data.name.to_string();
    println!("Nom: {}", nom);
    let value = form_data.value.to_string();
    println!("Value: {}", value);
    
    let mut conn = db.get_conn().unwrap();
    conn.exec_drop(
        r"DELETE FROM credential WHERE nom = :nom AND value = :value",
        params! {
            "nom" => nom,
            "value" => value,
        },
    ).unwrap();

    HttpResponse::Found()
        .header("Location", "/")
        .finish()
}

///////////////////////////////////////////MODIFIER///////////////////////////////////////////////////////////

fn modify_credential() -> &'static str {
    "<!DOCTYPE html>
    <html>
        <head>
            <meta charset=\"UTF-8\">
            <title>Modifier les credentials</title>
        </head>
        <body>
        <h1>Modifier les credentials</h1>
        <form id=\"my-form\">
            <label for=\"name\">Nom:</label>
            <input type=\"text\" id=\"name\" name=\"name\">
            <label for=\"value\">Value:</label>
            <input type=\"text\" id=\"value\" name=\"value\">
            <button type=\"submit\">Modifier</button>


        </form>
            <script>
                const form = document.getElementById('my-form');
                form.addEventListener('submit', (event) => {
                    event.preventDefault();
                    const formData = new FormData(form);
                    fetch('/modifier', {
                        method: 'POST',
                        headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
                                 },
                        body: new URLSearchParams(formData)
    
                    })

                    .then(response => {
                        // traitement de la réponse ici
                        console.log(response.text());
                        // redirection de l'utilisateur
                        window.location.href = 'http://127.0.0.1:8080/ADD2';
                    })
                    .catch(error => console.error(error));
                });
        
            </script>
        </body>
    </html>"
}


async fn modifier_credential() -> io::Result<impl Responder> {
    let html = modify_credential();
    Ok(HttpResponse::Ok().body(html))
}


async fn modifier_cred(db: web::Data<Pool>, form_data: web::Form<Credential>) -> HttpResponse {
    let nom = form_data.name.to_string();
    println!("Nom: {}", nom);
    let value = form_data.value.to_string();
    println!("Value: {}", value);
    
    let mut conn = db.get_conn().unwrap();
    conn.exec_drop(
        r"DELETE FROM credential WHERE nom = :nom AND value = :value",
        params! {
            "nom" => nom,
            "value" => value,
        },
    ).unwrap();

    HttpResponse::Found()
        .header("Location", "/")
        .finish()
}

fn modify_credential2() -> &'static str {
    "<!DOCTYPE html>
    <html>
        <head>
            <meta charset=\"UTF-8\">
            <title>BDD Bifrost</title>
        </head>
        <body>
        <h1>Rentrer les nouvelles données</h1>
        <form id=\"my-form\">
            <label for=\"name\">Nom:</label>
            <input type=\"text\" id=\"name\" name=\"name\">
            <label for=\"value\">Value:</label>
            <input type=\"text\" id=\"value\" name=\"value\">
            <button type=\"submit\">Ajouter</button>

        </form>
            <script>
                const form = document.getElementById('my-form');
                form.addEventListener('submit', (event) => {
                    event.preventDefault();
                    const formData = new FormData(form);
                    fetch('/ADD2', {
                        method: 'POST',
                        headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
                                 },
                        body: new URLSearchParams(formData)
    
                    })
                    
                    .then(response => {
                        // traitement de la réponse ici
                        console.log(response.text());
                        // redirection de l'utilisateur
                        window.location.href = 'http://127.0.0.1:8080/BDD';
                    })
                    .catch(error => console.error(error));
                });
            </script>
        </body>
    </html>"
}

async fn add_credential2() -> io::Result<impl Responder> {
    let html = modify_credential2();
    Ok(HttpResponse::Ok().body(html))
}


async fn ajout_credential2(db: web::Data<Pool>, form_data: web::Form<Credential>) -> HttpResponse {
    let nom = form_data.name.to_string();
    println!("Nom: {}", nom);
    let value = form_data.value.to_string();
    println!("Value: {}", value);
    
    let mut conn = db.get_conn().unwrap();
    conn.exec_drop(
        r"INSERT INTO credential (nom, value) VALUES (:nom, :value)",
        params! {
            "nom" => nom,
            "value" => value,
        },
    ).unwrap();

    HttpResponse::Found()
        .header("Location", "/")
        .finish()
}

///////////////////////////////////////////MAIN///////////////////////////////////////////////////////////

#[actix_web::main]
async fn main() -> io::Result<()> {
    let db_url = "mysql://credential:credential@localhost:3306/bifrost2";
    let db_pool = Pool::new(db_url).unwrap();
    let db_data = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(Files::new("/templates", "templates").show_files_listing())
            .service(web::resource("/ADD").route(web::get().to(add_credential)).route(web::post().to(ajout_cred)))
            .route("/BDD", web::get().to(bdd))
            .service(web::resource("/delete").route(web::get().to(delete_credential)).route(web::post().to(delete_cred)))
            .service(web::resource("/modifier").route(web::get().to(modifier_credential)).route(web::post().to(modifier_cred)))
            .service(web::resource("/ADD2").route(web::get().to(add_credential2)).route(web::post().to(ajout_credential2)))
            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}