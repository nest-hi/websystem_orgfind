
mod components;
mod models;
mod pages;


use crate::pages::discover::Discover;

// fn main(){
//     yew::Renderer::<App>::new().render();
// }

fn main() {
    yew::Renderer::<Discover>::with_root(gloo::utils::document().get_element_by_id("search_bar").unwrap().into()).render();
}



