use rocket::{State, http::Status, response::status::Custom, serde::json::Json};
use tokio_postgres::Client;

use crate::{db::{execute_query, fetch_all}, models::event::Event};



#[post("/",data = "<event>")] 
pub async fn add_event( connection: &State<Client>,event: Json<Event> ) //
    -> Result<Json<Vec<Event>>,Custom<String>> {
    execute_query(connection, "INSERT INTO events (name) VALUES ($1)",
        &[&event.name]
    ).await?;
    get_events(connection).await
}

#[get("/")]
pub async fn get_events( connection: &State<Client> ) -> Result<Json<Vec<Event>>, Custom<String>> { //
    get_events_from_database(connection).await.map(Json)
}

pub async fn get_events_from_database( client: &Client ) -> Result<Vec<Event>, Custom<String>> { //
    fetch_all(client, "SELECT id, name FROM events", |row| Event {
        id: Some(row.get(0)),
        name: row.get(1),
    }).await
}

#[delete("/<id>")]
pub async fn delete_event( connection: &State<Client>, id: i32 )
    -> Result<Status, Custom<String>>{
    execute_query(connection, "DELETE FROM events where id = $1", 
    &[&id]).await?;
        Ok(Status::NoContent)
}


#[put("/<id>", data ="<event>")]
pub async fn update_event( connection: &State<Client>, id: i32, event:Json<Event> ) //
    -> Result<Json<Vec<Event>>,Custom<String>> {
        execute_query(connection, "UPDATE events SET name = $1 WHERE id = $2", 
        &[&event.name,  &id]).await?;
        get_events(connection).await
}

pub fn routes() -> Vec<rocket::Route>{
    routes![add_event, get_events, update_event, delete_event]
}