use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug, Clone, PartialOrd, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String
}