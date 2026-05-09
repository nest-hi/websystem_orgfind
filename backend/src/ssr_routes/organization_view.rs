use rocket::response::content::RawHtml;
use yew::{Html, Properties, ServerRenderer, component, html};

use crate::{ViewableData, models::organization::Organization};



#[derive(Properties, PartialEq, Default)]
pub struct OrganizationProps {
    pub organization_id: i32,
}


#[component(OrganizationView)]
pub fn organization_view(props: &OrganizationProps) -> Html {
    html! {
        <>
            <div>
                { format!("replace-Organization ID: {}", props.organization_id) }
            </div>
        </>
    }
}

#[get("/organization/<id>")]
pub async fn organization_rendered_view(id:i32) -> RawHtml<String> {
    
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
                        <a href="/index.html">Home</a>
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

pub fn routes() -> Vec<rocket::Route> {
    routes![organization_rendered_view]
}
