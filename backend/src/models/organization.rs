
use serde::{Deserialize, Serialize};

use crate::models::tag::Tag;

#[derive(Serialize,Deserialize,Clone)]
pub struct Organization{
    pub id: Option<i32>,
    pub name: String,
    pub tags: Vec<Tag>
} 