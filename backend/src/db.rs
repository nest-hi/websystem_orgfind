use std::collections::HashMap;

use rocket::serde::json::Json;
use rocket::{State, http::Status};
use rocket::response::status::Custom;
use serde::Serialize;
use tokio_postgres::{Client, Row};
// use std::collections::HashMap;

pub async fn execute_query(
    client: &Client,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<u64, Custom<String>> {
    client
        .execute(query, params)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
}

pub async fn fetch_all<T, F>(
    client: &Client,
    query: &str,
    mapper: F,
) -> Result<Vec<T>, Custom<String>>
where
    F: Fn(&Row) -> T,
{
    let rows = client
        .query(query, &[])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(rows.iter().map(mapper).collect())
}


#[derive(Serialize)]
pub struct SearchResult {
    pub result_type: String,
    pub title: String,
    pub tags: Vec<String>,
    pub is_academic: Option<bool>,
}

// pub async fn search_query(
//     client: &State<Client>,
//     query: &str,
//     params: &[&(dyn tokio_postgres::types::ToSql + Sync)])
//      -> Result<Json<Vec<SearchResult>>, Custom<String>> {
//     let rows = client.query(
//         "
//         ", &[])
//         .await
//         .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
//     if rows.is_empty(){
//         return Err(Custom(Status::NotFound, "No Results Found".to_string()));
//     } else {

//         let mut map: HashMap<i32, SearchResult> = HashMap::new();

//         for row in rows {

//         }






//         return Ok(Json(rows));
//     }


    
        
    // fetch_all(client, query, |row| SearchResult{
    //     result_type: row.get(0),
    //     title: row.get(1),
    //     tags: row.get(2),
    //     is_academic: row.get(3)
    // } ).await
// }

pub async fn search_by_tags(
    client: &Client,
    tags: &[&str],
) -> Result<Json<Vec<SearchResult>>, Custom<String>> {
    if tags.is_empty() {
        return Err(Custom(Status::BadRequest, "No tags provided".to_string()));
    }

    // Convert tags to a format suitable for SQL
    let tags_param: Vec<&str> = tags.iter().copied().collect();
    
    let query = "
        SELECT 'user' as result_type, u.username as title, u.tags, NULL as is_academic
        FROM users u
        WHERE u.tags && $1::text[]
        UNION ALL
        SELECT 'organization' as result_type, o.name as title, o.tags, NULL as is_academic
        FROM organizations o
        WHERE o.tags && $1::text[]
        UNION ALL
        SELECT 'event' as result_type, e.title, e.tags, e.is_academic
        FROM events e
        WHERE e.tags && $1::text[]
        ORDER BY result_type, title
    ";

    let rows = client
        .query(query, &[&tags_param])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    if rows.is_empty() {
        return Err(Custom(Status::NotFound, "No results found".to_string()));
    }

    let results: Vec<SearchResult> = rows
        .iter()
        .map(|row| SearchResult {
            result_type: row.get(0),
            title: row.get(1),
            tags: row.get(2),
            is_academic: row.get(3),
        })
        .collect();

    Ok(Json(results))
}