use rocket::response::content::RawHtml;
use yew::{Html, Properties, ServerRenderer, component, html};

use crate::{ViewableData, models::event::Event};


#[derive(Properties, PartialEq, Default)]
pub struct EventProps {
    pub event_id: i32,
}

#[component(EventView)]
pub fn event_view(props: &EventProps) -> Html {
    html! {
        <>
            <div>
                { format!("replace-Event ID: {}", props.event_id) }
            </div>
        </>
    }
}

#[get("/event/<id>")]
pub async fn event_rendered_view(id:i32) -> RawHtml<String> {
    
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
            if let Ok(data) = resp.json::<Event>().await {
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
                        <a href="/index.html">Home</a>
                        <a href="/discover.html" class="active">Discover</a>
                        <a href="/about.html">About Us</a>
                        <a href="/login.html">Log In</a>
                    </nav>
                    <img src="images/moon-icon.png" alt="" class="dark-mode-toggle" onclick="toggledarkmode()">
                    <img src="images/user-icon.png" alt="" class="profile-button" link="/login.html">
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

pub fn routes() -> Vec<rocket::Route> {
    routes![event_rendered_view]
}
