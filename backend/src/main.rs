#[macro_use] extern crate rocket;

mod db;
mod models;
mod routes;

use rocket::fairing::AdHoc;
use rocket_cors::{AllowedOrigins, CorsOptions};
use tokio_postgres::NoTls;
use routes::*;

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
                .attach(cors)
        }))//
}