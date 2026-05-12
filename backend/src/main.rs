#[macro_use] extern crate rocket;

mod db;
mod models;
mod routes;
mod ssr_routes;

use std::path::{Path, PathBuf};




use rocket::{  fairing::AdHoc, fs::{FileServer, NamedFile}};
use rocket_cors::{AllowedOrigins, CorsOptions};
use tokio_postgres::NoTls;
use routes::*;
use ssr_routes::*;



#[launch]
async fn rocket() -> _ {
    let (client, connection) =
        tokio_postgres::connect(
        "host=localhost user=postgres password=bash dbname=postgres"
        , NoTls)
            .await
            .expect("Failed to connect");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .to_cors()
    .expect("ERROR while building CORS");

    

    rocket::build()
        .manage(client)
        .attach(AdHoc::on_ignite("Routes", |rocket| async {
            rocket
            
        .mount("/api/users", user::routes())
        .mount("/api/events", event::routes())
        .mount("/api/tags", tag::routes())
        .mount("/api/organizations", organization::routes())

        .mount("/", user_view::routes())
        .mount("/", organization_view::routes())
        .mount("/",event_view::routes() )

        // .mount("/", routes![user_rendered_view,organization_rendered_view,event_rendered_view])
        
        .mount("/", FileServer::from("../frontend/webroot").rank(5))
        .mount("/", FileServer::from("../frontend/dist").rank(10))
        .mount("/image/", FileServer::from("image").rank(1))

        .mount("/", routes![spa_fallback])

        .attach(cors)
        }))
}


#[get("/<path..>", rank = 20)]
async fn spa_fallback(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("../frontend/dist/index.html")).await.ok()
}


#[derive(Clone)]
pub struct ViewableData {
    id: Option<i32>,
    name: Option<String>,
}
