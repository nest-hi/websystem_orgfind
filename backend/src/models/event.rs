
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Clone)]
pub struct Event{
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub date_occuring: NaiveDate,
    pub host_id: i32,
}



