#[macro_use] extern crate rocket;

mod db;
mod models;
mod routes;

use std::path::{Path, PathBuf};

use web_sys::wasm_bindgen::JsCast;
use yew::{Callback, Properties, ServerRenderer};

use rocket::{ fairing::AdHoc, fs::{FileServer, NamedFile}, response::content::RawHtml};
use rocket_cors::{AllowedOrigins, CorsOptions};
use tokio_postgres::NoTls;
use routes::*;
use yew::{Html, component, html};

use crate::models::organization::Organization;


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
        .mount("/", routes![user_rendered_view,organization_rendered_view,event_rendered_view])
        
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
        
        <>
            // <header class={"header-container"}>
            //     <div class={"header-title-box"}>
            //         <img src={"/images/hub-light-icon.png"} class={"header-logo"} />
                    
            //         <div class={"header-title"}>
            //             <h1>{"Organization and Events Hub"}</h1>
            //             <p>{"Connecting you to the world of organizations and events"}</p>
            //         </div>
            //     </div>

            //     <div class={"navigation-container"}>
            //         <nav class={"navigation"}>
            //             <a href={"/home.html"}>{"Home"}</a>
            //             <a href={"/discover.html"} class={"active"}>{"Discover"}</a>
            //             <a href={"/about.html"}>{"About Us"}</a>
            //             <a href={"/login.html"}>{"Log In"}</a>
            //         </nav>

            //         <img
            //             src={"/images/moon-icon.png"} 
            //             class={"dark-mode-toggle"}
                        
            //         />

            //         <img
            //             src={"/images/user-icon.png"} 
            //             class={"profile-button"}
            //         />
            //     </div>
            // </header>
        
            <div>
                { format!("replace-User ID: {}", props.user_id) }
            </div>
        </>
    }
}
#[component(OrganizationView)]
fn organization_view(props: &OrganizationProps) -> Html {
    html! {
        <>
            <div>
                { format!("replace-Organization ID: {}", props.organization_id) }
            </div>
        </>
    }
}

#[component(EventView)]
fn event_view(props: &EventProps) -> Html {
    html! {
        <>
            <div>
                { format!("replace-Event ID: {}", props.event_id) }
            </div>
        </>
    }
}

#[get("/user/<id>")]
async fn user_rendered_view(id:i32) -> RawHtml<String> {
    
    let html = ServerRenderer::<UserView>::with_props(move || UserProps {
        user_id: id,
    })
    .render()
    .await;

    let mut title = ViewableData {
        id: None,
        name: None,
    };

    if let Ok(resp) = reqwest::get(
        format!("http://127.0.0.1:8000/api/users/{}", id)
    ).await {
        if resp.status().is_success() {
            if let Ok(data) = resp.json::<Organization>().await {
                title = ViewableData {
                    id: data.id,
                    name: Some(data.name),
                };
            }
        }
    }

    RawHtml(format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="utf-8">
            <script>
                if (localStorage.getItem("theme") === "dark") document.documentElement.classList.add("darkmode");
            </script>
            <script src="/scripts/script.js"></script>
            <link rel="stylesheet" href="/stylesheets/style.css">
            <link rel="stylesheet" href="/stylesheets/org-event.css">
            <title>{}</title>
        </head>

        

        <body>
            <header class="header-container">
                <div class="header-title-box">
                    <img src="/images/hub-light-icon.png" alt="" class="header-logo">
                    <div class="header-title">
                        <h1>Organization and Events Hub</h1>
                        <p>Connecting you to the world of organizations and events</p>
                    </div>
                </div>
                <div class="navigation-container">
                    <nav class="navigation">
                        <a href="/home.html">Home</a>
                        <a href="/discover.html" class="active">Discover</a>
                        <a href="/about.html">About Us</a>
                        <a href="/login.html">Log In</a>
                    </nav>
                    <img src="/images/moon-icon.png" alt="" class="dark-mode-toggle" onclick="toggledarkmode()">
                    <img src="/images/user-icon.png" alt="" class="profile-button" link="/login.html">
                </div>
            </header>
            
            <div id="root">{}</div>
        </body>
        </html>
    "#, 
    title
        .name
        .clone()
        .unwrap_or("Default Title".to_string()),
        html
    ))
}

#[get("/organization/<id>")]
async fn organization_rendered_view(id:i32) -> RawHtml<String> {
    
    let html = ServerRenderer::<OrganizationView>::with_props(move || OrganizationProps {
        organization_id: id,
    })
    .render()
    .await;

    let mut title = ViewableData {
        id: None,
        name: None,
    };

    if let Ok(resp) = reqwest::get(
        format!("http://127.0.0.1:8000/api/organizations/{}", id)
    ).await {
        if resp.status().is_success() {
            if let Ok(data) = resp.json::<Organization>().await {
                title = ViewableData {
                    id: data.id,
                    name: Some(data.name),
                };
            }
        }
    }

    RawHtml(format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="utf-8">
            <script>
                if (localStorage.getItem("theme") === "dark") document.documentElement.classList.add("darkmode");
            </script>
            <script src="/scripts/script.js"></script>
            <link rel="stylesheet" href="/stylesheets/style.css">
            <link rel="stylesheet" href="/stylesheets/org-event.css">
            <title>{}</title>
        </head>

        

        <body>
            <header class="header-container">
                <div class="header-title-box">
                    <img src="/images/hub-light-icon.png" alt="" class="header-logo">
                    <div class="header-title">
                        <h1>Organization and Events Hub</h1>
                        <p>Connecting you to the world of organizations and events</p>
                    </div>
                </div>
                <div class="navigation-container">
                    <nav class="navigation">
                        <a href="/home.html">Home</a>
                        <a href="/discover.html" class="active">Discover</a>
                        <a href="/about.html">About Us</a>
                        <a href="/login.html">Log In</a>
                    </nav>
                    <img src="/images/moon-icon.png" alt="" class="dark-mode-toggle" onclick="toggledarkmode()">
                    <img src="/images/user-icon.png" alt="" class="profile-button" link="/login.html">
                </div>
            </header>
            
            <div id="root">{}</div>
        </body>
        </html>
    "#,
    title
        .name
        .clone()
        .unwrap_or("Default Title".to_string()),
    html
    ))
}

#[get("/event/<id>")]
async fn event_rendered_view(id:i32) -> RawHtml<String> {
    
    let html = ServerRenderer::<EventView>::with_props(move || EventProps {
        event_id: id,
    })
    .render()
    .await;

    let mut title = ViewableData {
        id: None,
        name: None,
    };

    if let Ok(resp) = reqwest::get(
        format!("http://127.0.0.1:8000/api/events/{}", id)
    ).await {
        if resp.status().is_success() {
            if let Ok(data) = resp.json::<Organization>().await {
                title = ViewableData {
                    id: data.id,
                    name: Some(data.name),
                };
            }
        }
    }

    // RawHtml(html)
    RawHtml(format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="utf-8">
            <script>
                if (localStorage.getItem("theme") === "dark") document.documentElement.classList.add("darkmode");
            </script>
            <script src="/scripts/script.js"></script>
            <link rel="stylesheet" href="/stylesheets/style.css">
            <link rel="stylesheet" href="/stylesheets/org-event.css">
            <title>{}</title>
        </head>

        

        <body>
            <header class="header-container">
                <div class="header-title-box">
                    <img src="/images/hub-light-icon.png" alt="" class="header-logo">
                    <div class="header-title">
                        <h1>Organization and Events Hub</h1>
                        <p>Connecting you to the world of organizations and events</p>
                    </div>
                </div>
                <div class="navigation-container">
                    <nav class="navigation">
                        <a href="/home.html">Home</a>
                        <a href="/discover.html" class="active">Discover</a>
                        <a href="/about.html">About Us</a>
                        <a href="/login.html">Log In</a>
                    </nav>
                    <img src="/images/moon-icon.png" alt="" class="dark-mode-toggle" onclick="toggledarkmode()">
                    <img src="/images/user-icon.png" alt="" class="profile-button" link="/login.html">
                </div>
            </header>
            
            <div id="root">{}</div>
        </body>
        </html>
    "#,
    title
        .name
        .clone()
        .unwrap_or("Default Title".to_string()),
    html
    ))
}




#[derive(Clone)]
pub struct ViewableData {
    id: Option<i32>,
    name: Option<String>,
}
