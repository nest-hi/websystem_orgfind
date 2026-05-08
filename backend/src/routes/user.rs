use rocket::{State, serde::json::Json};
use rocket::response::status::Custom;
use rocket::http::Status;
use tokio_postgres::Client;

use crate::models::user::User;
use crate::db::{execute_query, fetch_all};

#[post("/",data = "<user>")] 
pub async fn add_user( connection: &State<Client>,user: Json<User> ) //
    -> Result<Json<Vec<User>>,Custom<String>> {
    execute_query(connection, "INSERT INTO users (name,email,password) VALUES ($1,$2,$3)",
        &[&user.name ,&user.email, &user.password]
    ).await?;
    get_users(connection).await
}


#[delete("/<id>")]
pub async fn delete_user( connection: &State<Client>, id: i32 ) //
    -> Result<Status, Custom<String>>{
    execute_query(connection, "DELETE FROM users where id = $1", 
    &[&id]).await?;
        Ok(Status::NoContent)
}

#[get("/")]
pub async fn get_users( connection: &State<Client> ) -> Result<Json<Vec<User>>, Custom<String>> { //
    get_users_from_database(connection).await.map(Json)
}


pub async fn get_users_from_database( client: &Client ) -> Result<Vec<User>, Custom<String>> { //
    fetch_all(client, "SELECT id, name, email, password FROM users", |row| User {
        id: Some(row.get(0)),
        name: row.get(1),
        email: row.get(2),
        password: row.get(3)
    }).await
}

#[put("/<id>", data ="<user>")]
pub async fn update_user( connection: &State<Client>, id: i32, user:Json<User> ) //
    -> Result<Json<Vec<User>>,Custom<String>> {
        execute_query(connection, "UPDATE users SET name = $1, email = $2, password = $3 WHERE id = $4", 
        &[&user.name, &user.email ,&user.password, &id]).await?;
        get_users(connection).await
}

pub fn routes() -> Vec<rocket::Route> {
    routes![add_user, get_users, update_user, delete_user]
}

