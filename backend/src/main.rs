#[macro_use] extern crate rocket;

mod db;
mod models;
mod routes;

use std::path::{Path, PathBuf};


use yew::{Properties, ServerRenderer};

use rocket::{Route, fairing::AdHoc, fs::{FileServer, NamedFile}, response::content::RawHtml};
use rocket_cors::{AllowedOrigins, CorsOptions};
use tokio_postgres::NoTls;
use routes::*;
use yew::{Html, component, html};

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
        .mount("/", routes![user_view,organization_view,event_view])
        
        .mount("/", FileServer::from("../frontend/webroot").rank(5))
        .mount("/", FileServer::from("../frontend/dist").rank(10))
        .mount("/", routes![spa_fallback])

        .attach(cors)
        }))
}


#[get("/<path..>", rank = 20)]
async fn spa_fallback(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("../frontend/dist/index.html")).await.ok()
}

#[derive(Properties, PartialEq, Default)]
pub struct UserProps {
    pub user_id: i32,
}

#[derive(Properties, PartialEq, Default)]
pub struct OrganizationProps {
    pub organization_id: i32,
}

#[derive(Properties, PartialEq, Default)]
pub struct EventProps {
    pub event_id: i32,
}

#[component(UserView)]
fn user_view(props: &UserProps) -> Html {
    html! {
        <div>
            { format!("User ID: {}", props.user_id) }
        </div>
    }
}
#[component(OrganizationView)]
fn user_view(props: &UserProps) -> Html {

    

    html! {
        <div>
            { format!("Organization ID: {}", props.user_id) }
        </div>
    }
}
#[component(EventView)]
fn user_view(props: &UserProps) -> Html {
    html! {
        <div>
            { format!("Event ID: {}", props.user_id) }
        </div>
    }
}

#[get("/user/<id>")]
async fn user_view(id:i32) -> RawHtml<String> {
    
    let html = ServerRenderer::<UserView>::with_props(move || UserProps {
        user_id: id,
    })
    .render()
    .await;

    RawHtml(html)
}

#[get("/organization/<id>")]
async fn organization_view(id:i32) -> RawHtml<String> {
    
    let html = ServerRenderer::<OrganizationView>::with_props(move || UserProps {
        user_id: id,
    })
    .render()
    .await;

    RawHtml(html)
}

#[get("/event/<id>")]
async fn event_view(id:i32) -> RawHtml<String> {
    
    let html = ServerRenderer::<EventView>::with_props(move || UserProps {
        user_id: id,
    })
    .render()
    .await;

    RawHtml(html)
}

