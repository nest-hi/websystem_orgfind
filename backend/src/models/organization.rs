
use serde::{Deserialize, Serialize};

use crate::models::tag::Tag;

#[derive(Serialize,Deserialize,Clone, PartialEq, Default)]
pub struct Organization{
    pub id: Option<i32>,
    pub name: String,
    pub pfp: Option<String>,
    pub bgp: Option<String>,
    pub tags: Vec<Tag>
} 