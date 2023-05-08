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

fn see_bdd2(result: Vec<(String,String)>) -> String {
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
    let html_content = see_bdd2(result);
    HttpResponse::Ok().body(html_content)
}

//////////////////////////////////////////PAGE BDD///////////////////////////////////////////////////////////

fn see_bdd(result: Vec<String>) -> String {
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8">
            <title>BDD USER Bifrost</title>
        </head>
        <body>
            <h1>USER B!FROST</h1>
            <input type="button" id="retour" onclick="window.location.href='http://127.0.0.1:8080/templates/menu1.html'" value="Retour">
            <input type="button" onclick="window.location.href='http://127.0.0.1:8080/delete'" value="Supprimer User">
            <input type="button" onclick="window.location.href='http://127.0.0.1:8080/templates/ajouter_user.html'" value="Ajouter User">
            <table>
                <tr>
                    <th>Login</th>
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
                    console.log("test");
                    window.location.replace("http://127.0.0.1:8080/BDD"); // rediriger l'utilisateur vers une page d'erreur
                }}
            </script>
        </body>
        </html>
        "#,
        result
        .into_iter()
        .map(|login| { // utilise la première valeur du tuple (le login)
            format!(
                r#"
                    <tr>
                        <td>{}</td>
                    </tr>
                "#,
                login
            )
        })
        .collect::<String>()
    );

    html
}



async fn BDD(db: web::Data<Pool>) -> impl Responder {
    let mut conn = db.get_conn().unwrap();

    let query = "SELECT login FROM password";
    let result = conn.query_map(query, |login: String| login).unwrap();
    let html_content = see_bdd(result);
    HttpResponse::Ok().body(html_content)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let db_url = "mysql://lolo:lolo@localhost:3306/loic";
    let db_pool = Pool::new(db_url).unwrap();
    let db_data = web::Data::new(db_pool);
    //let db_url2 = "mysql://credential:credential@localhost:3306/bifrost2";
    //let db_pool2 = Pool::new(db_url2).unwrap();
    //let db_data2 = web::Data::new(db_pool2);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(Files::new("/templates", "templates").show_files_listing())
            //.service(web::resource("/ADD").route(web::get().to(add_credential)).route(web::post().to(ajout_cred)))
            .route("/BDD2", web::get().to(bdd))
            //.service(web::resource("/delete").route(web::get().to(delete_credential)).route(web::post().to(delete_cred)))
            //.service(web::resource("/modifier").route(web::get().to(modifier_credential)).route(web::post().to(modifier_cred)))
            //.service(web::resource("/ADD2").route(web::get().to(add_credential2)).route(web::post().to(ajout_credential2)))
            //.app_data(db_data.clone())
            .route("/BDD", web::get().to(BDD))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}