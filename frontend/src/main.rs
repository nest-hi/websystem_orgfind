
mod components;
mod models;

use crate::components::users_list::App;

// fn main(){
//     yew::Renderer::<App>::new().render();
// }

fn main() {
    yew::Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("app").unwrap().into()).render();
}

