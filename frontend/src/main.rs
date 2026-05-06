
mod components;
mod models;
mod pages;

use yew::{Callback, Html,  html};
use yew_router::{BrowserRouter, Routable, Switch, hooks::use_navigator};

use crate::components::{search_bar::SearchBar};
use crate::pages::{discover::Discover};

// fn main(){
//     yew::Renderer::<App>::new().render();
// }

fn main() {
    yew::Renderer::<Discover>::with_root(gloo::utils::document().get_element_by_id("search_bar").unwrap().into()).render();
}



