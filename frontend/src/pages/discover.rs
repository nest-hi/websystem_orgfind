use gloo::net::http::Request;
use serde::{Deserialize};
use wasm_bindgen_futures::spawn_local;
use yew::{html, prelude::*};

use crate::components::search_bar::SearchBar;

#[function_component(Discover)]
pub fn discover() -> Html  {

    

    html!{
        <>
            <h1>{"This is from discover.rs"}</h1>
            <SearchBar/>
        </> 
    }
}