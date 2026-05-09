use rocket::response::content::RawHtml;
use yew::{Html, Properties, ServerRenderer, component, html};

use crate::{ViewableData, models::user::User};



#[derive(Properties, PartialEq, Default)]
pub struct UserProps {
    pub user_id: i32,
}


#[component(UserView)]
pub fn user_view(props: &UserProps) -> Html {
    html! {
        <>        
            <div>
                { format!("replace-User ID: {}", props.user_id) }
            </div>
        </>
    }
}


#[get("/user/<id>")]
pub async fn user_rendered_view(id:i32) -> RawHtml<String> {
    
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
            if let Ok(data) = resp.json::<User>().await {
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
        .unwrap_or("Meep User".to_string()),
        html
    ))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![user_rendered_view]
}
