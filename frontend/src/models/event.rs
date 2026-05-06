use serde::Deserialize;

#[derive(Deserialize, Clone, PartialOrd, PartialEq)]
pub struct Event {
    pub id: i32,
    pub name: String,
}


