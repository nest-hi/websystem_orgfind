use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize,Clone)]
pub struct Tag{
    pub id: Option<i32>,
    pub name: String,
} 