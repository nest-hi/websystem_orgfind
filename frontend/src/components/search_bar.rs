use gloo::net::http::Request;
use serde::{Deserialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::models::{organization::Organization, user::User, event::Event};



#[derive(Clone, Debug, PartialEq, Deserialize, PartialOrd , Eq, Ord)]
pub enum SearchResult {
    User { id: i32, name: String },
    Organization { id: i32, name: String },
    Event { id: i32, name: String },
}




#[function_component(SearchBar)]
pub fn search_bar() -> Html {

    
    let search_state = use_state(|| "".to_string());
    let result_list: UseStateHandle<Vec<SearchResult>> = use_state(Vec::new);

    let values = result_list.clone();
    use_effect_with((), {

        
        
        move |_| {
            spawn_local(async move {
                let mut combined: Vec<SearchResult> = Vec::new();

                // users
                if let Ok(resp) = Request::get("http://127.0.0.1:8000/api/users").send().await {
                    if resp.ok() {
                        let data: Vec<User> = resp.json().await.unwrap();
                        combined.extend(data.into_iter().map(|u| SearchResult::User {
                            id: u.id,
                            name: u.name,
                        }));
                    }
                }

                // orgs
                if let Ok(resp) = Request::get("http://127.0.0.1:8000/api/organizations").send().await {
                    if resp.ok() {
                        let data: Vec<Organization> = resp.json().await.unwrap();
                        combined.extend(data.into_iter().map(|o| SearchResult::Organization {
                            id: o.id,
                            name: o.name,
                        }));
                    }
                }

                // events
                if let Ok(resp) = Request::get("http://127.0.0.1:8000/api/events").send().await {
                    if resp.ok() {
                        let data: Vec<Event> = resp.json().await.unwrap();
                        combined.extend(data.into_iter().map(|e| SearchResult::Event {
                            id: e.id,
                            name: e.name,
                        }));
                    }
                }

                result_list.set(combined);
            });

            || ()
        }
    });


    let oninput = {
        let search_state = search_state.clone();

        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            search_state.set(input.value());
        })
    };

    let filtered = {
    let query = search_state.to_lowercase();

    (*values)
        .iter()
        .filter(|item| match item {
              SearchResult::User { name, .. }
            | SearchResult::Organization { name, .. }
            | SearchResult::Event { name, .. } => {
                name.to_lowercase().contains(&query)
            }
        })
        
        .clone()
        .collect::<Vec<_>>()
        
        
        
};
    html!{
        <div>
            <div class="search_box">
                <input
                type="text"
                class="search_bar"
                placeholder="Search..."
                value={(*search_state).clone()}
                oninput = {oninput}
                />
                <button>
                    {"Filter"}
                </button>
            </div>
            
            
            <div class="search_results">
                {
                    for filtered.iter().map(|item| {
                        match item {
                            SearchResult::User { name, id, .. } => html! {
                                <a
                                href={format!("http://127.0.0.1:8000/user/{}",id)}
                                class="user_result item"
                                >{ format!("{}", name) }</a>
                                
                            },
                            SearchResult::Organization { name, id, .. } => html! {
                                <a
                                href={format!("http://127.0.0.1:8000/organization/{}",id)}
                                class="org_result item"
                                >{ format!("{}", name) }</a>
                            },
                            SearchResult::Event { name, id, .. } => html! {
                                <a
                                href={format!("http://127.0.0.1:8000/event/{}",id)}
                                class="event_result item"
                                >{ format!("{}", name) }</a>
                            },
                        }
                    })
                }
            </div>
        </div>
    }
}

