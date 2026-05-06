use serde::Deserialize;

#[derive(Deserialize, Clone, PartialOrd, PartialEq)]
pub struct Organization {
    pub id: i32,
    pub name: String,
}