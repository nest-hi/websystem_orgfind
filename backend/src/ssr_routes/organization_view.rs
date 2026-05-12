use rocket::response::content::RawHtml;
use yew::{Html, Properties, ServerRenderer, component, html};

use crate::models::{organization::Organization, tag::Tag};



#[derive(Properties, PartialEq, Default)]
pub struct OrganizationProps {
    pub organization: Organization
}


#[component(OrganizationView)]
pub fn organization_view(props: &OrganizationProps) -> Html {
    let org = &props.organization;

    html! {
        <>
            <div class="organization-page">
                <div class="background-container">
                    <img
                        class="background-image"
                        src={
                            org.bgp
                                .clone()
                                .unwrap_or("/images/default-bg.png".into())
                        }
                    />
                </div>
                <div class="hero">
                    <img
                        class="profile-image"
                        src={
                            org.pfp
                                .clone()
                                .unwrap_or("/images/default-pfp.png".into())
                        }
                    />
                    
                    <div class="info">
                        <h1 class="name">{ &org.name }</h1>
                        <div id="imaginary_points">
                            <h6 class="followers">{""}</h6>
                        </div>
                    </div>
                    
                </div>
                

                <p>
                    // { format!("Organization ID: {:?}", org.id) }
                </p>

            </div>
        </>
    }
}





#[get("/organization/<id>")]
pub async fn organization_rendered_view(id:i32) -> RawHtml<String> {
    
   
    let organization =
        reqwest::get(
            format!(
                "http://127.0.0.1:8000/api/organizations/{}",
                id
            )
        )
        .await
        .unwrap()
        .json::<Organization>()
        .await
        .unwrap();

    let title = organization.name.clone();

    let html =
        ServerRenderer::<OrganizationView>::with_props(
            move || OrganizationProps {
                organization,
            }
        )
        .render()
        .await;

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
            <link rel="stylesheet" href="/stylesheets/organization-view.css">
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
            
            <body id="root">{}</body>
        </body>
        </html>
        "#,
        title,
        html
    ))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![organization_rendered_view]
}
