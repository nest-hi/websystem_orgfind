use rocket::{State, http::Status, response::status::Custom, serde::json::Json};
use tokio_postgres::Client;

use crate::{db::{execute_query, fetch_all}, models::tag::Tag};



#[post("/",data = "<tag>")] 
pub async fn add_tag( connection: &State<Client>, tag: Json<Tag> )
    -> Result<Json<Vec<Tag>>,Custom<String>> {
    execute_query(connection, "INSERT INTO tags (name) VALUES ($1)",
        &[&tag.name]
    ).await?;
    get_tags(connection).await
}

// #[get("/<id>",data = "<id>")]
// pub async fn get_tag( connection: &State<Client> , id:u64 )
//     -> Result<Json<Tag>,Custom<String>>{

// }

#[get("/")]
pub async fn get_tags(connection: &State<Client>)
    -> Result<Json<Vec<Tag>>, Custom<String>> {
    get_tag_from_database(connection).await.map(Json)
}

#[delete("/<id>")]
pub async fn delete_tag( connection: &State<Client>, id: i32 )
    -> Result<Status, Custom<String>>{
    execute_query(connection, "DELETE FROM tags where id = $1", 
    &[&id]).await?;
        Ok(Status::NoContent)
}

pub async fn get_tag_from_database( client: &Client )
    -> Result<Vec<Tag>, Custom<String>> {
    fetch_all(client, "SELECT id, name FROM tags", |row| Tag {
        id: Some(row.get(0)),
        name: row.get(1),
    }).await
}

#[put("/<id>", data ="<tag>")]
pub async fn update_tag( connection: &State<Client>, id: i32, tag:Json<Tag> ) //
    -> Result<Json<Vec<Tag>>,Custom<String>> {
        execute_query(connection, "UPDATE tags SET name = $1 WHERE id = $2", 
        &[&tag.name , &id]).await?;
        get_tags(connection).await
}


pub fn routes() -> Vec<rocket::Route> {
    routes![add_tag, get_tags, update_tag, delete_tag]
}