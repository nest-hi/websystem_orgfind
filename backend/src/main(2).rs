


#[macro_use] extern crate rocket;

use std::collections::HashMap;

use rocket::State;
use rocket::serde::json::Json;
use rocket::{response::status::Custom, http::Status};
use serde::Deserialize;
use serde::Serialize;
use tokio_postgres::{Client,NoTls};
use rocket_cors::{CorsOptions, AllowedOrigins};


#[derive(Serialize,Deserialize,Clone)]
pub struct Event{
    id: Option<i32>,
    name: String
} 

#[derive(Serialize,Deserialize,Clone)]
pub struct User{
    id: Option<i32>,
    name: String,
    email: String,
} 

#[derive(Serialize,Deserialize,Clone)]
pub struct Organization{
    id: Option<i32>,
    name: String,
    tags: Vec<Tag>
} 


pub async fn execute_query( client: &Client, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)] )
    -> Result<u64 , Custom<String>> {
    client
    .execute(query, params)
    .await
    .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
}

async fn fetch_all<T, F>(
    client: &Client,
    query: &str,
    mapper: F,
) -> Result<Vec<T>, Custom<String>>
where
    F: Fn(&tokio_postgres::Row) -> T,
{
    let rows = client
        .query(query, &[])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(rows.iter().map(mapper).collect())
}

//event

//user


//tag


// organization

//

#[launch]
async fn rocket() -> _{
    let (client,connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=bash dbname=postgres"
        , NoTls)
        .await.expect("failed to connect postgres");
    
    tokio::spawn(async move{
        if let Err(e) =  connection.await{
            eprintln!("Failed to connect to postgres {}" ,e);
        } 
    });

    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL
        )",
        &[]).await.expect("Failed to Create users Table");

    
    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .to_cors()
    .expect("ERROR while building CORS");

    rocket::build()
    .manage(client)
    .mount("/", routes![index])

    .mount("/api/organizations", routes![add_organization, get_organizations,get_organization, update_organization, delete_organization])
    .mount("/api/tags", routes![add_tag, get_tags, update_tag, delete_tag])
    .mount("/api/users", routes![add_user, get_users, update_user, delete_user])
    .mount("/api/events", routes![add_event, get_events, update_event, delete_event])
    .attach(cors)

}

#[get("/")]
async fn index()-> &'static str { //wellness check
    "Hello, world!"
}

