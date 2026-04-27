use std::collections::HashMap;

use rocket::{State, http::Status, response::status::Custom, serde::json::Json};
use tokio_postgres::Client;

use crate::{db::execute_query, models::{organization::Organization, tag::Tag}};


// pub async fn get_linked_tags(){
    
// }

#[post("/",data = "<organization>")] 
pub async fn add_organization( connection: &State<Client>, organization: Json<Organization> )
    -> Result<Json<Vec<Organization>>,Custom<String>> {
    execute_query(connection, "INSERT INTO organizations (name) VALUES ($1)",
        &[&organization.name ]
    ).await?;
    get_organizations(connection).await
}

#[get("/")] // this'll return every organization in the database if we go to "http://0.0.0.0:8002/api/organizations"
pub async fn get_organizations(connection: &State<Client>)
    -> Result<Json<Vec<Organization>>, Custom<String>> {
    get_organization_from_database(connection).await.map(Json)
}



#[get("/<id>")]
pub async fn get_organization(connection: &State<Client>, id: i32)
    -> Result<Json<Organization>, Custom<String>> {
    let rows = connection
        .query(
            "
            SELECT o.id, o.name, t.id, t.name 
            FROM organizations o 
            LEFT JOIN organization_tags ot ON o.id = ot.organization_id
            LEFT JOIN tags t ON ot.tag_id = t.id
            WHERE o.id = $1;
            ",
            &[&id],
        )
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    if rows.is_empty() {
        return Err(Custom(Status::NotFound, "Organization not found".to_string()));
    }

    let mut organization = Organization {
        id: Some(rows[0].get(0)),
        name: rows[0].get(1),
        tags: vec![],
    };

    for row in rows {
        let tag_id: Option<i32> = row.get(2);
        let tag_name: Option<String> = row.get(3);

        if let (id, Some(name)) = (tag_id, tag_name) {
            organization.tags.push(Tag {
                id,
                name,
            });
        }
    }

    Ok(Json(organization))
}



#[delete("/<id>")]
pub async fn delete_organization( connection: &State<Client>, id: i32 )
    -> Result<Status, Custom<String>>{
    execute_query(connection, "DELETE FROM organizations where id = $1", 
    &[&id]).await?;
        Ok(Status::NoContent)
}

pub async fn get_organization_from_database( client: &Client )
    -> Result<Vec<Organization>, Custom<String>> {
    

    
    let rows = client
        .query(
            "
            SELECT o.id, o.name, t.id, t.name 
            FROM organizations o 
            LEFT JOIN organization_tags ot ON o.id = ot.organization_id
            LEFT JOIN tags t ON ot.tag_id = t.id;
            ",
            &[]
        )
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let mut map: HashMap<i32, Organization> = HashMap::new();

    for row in rows {
        let org_id: i32 = row.get(0);
        let org_name: String = row.get(1);

        let tag_id: Option<i32> = row.get(2);
        let tag_name: Option<String> = row.get(3);

        let org = map.entry(org_id).or_insert(Organization {
            id: Some(org_id),
            name: org_name,
            tags: vec![],
        });

        if let (id, Some(name)) = (tag_id, tag_name) {
            org.tags.push(Tag {
                id,
                name,
                
            });
        }
    }

    Ok(map.into_values().collect())
}

#[put("/<id>", data ="<organization>")]
pub async fn update_organization( connection: &State<Client>, id: i32, organization:Json<Organization> ) //
    -> Result<Json<Vec<Organization>>,Custom<String>> {
        execute_query(connection, "UPDATE organizations SET name = $1 WHERE id = $2", 
        &[&organization.name , &id]).await?;
        get_organizations(connection).await
}

pub fn routes() -> Vec<rocket::Route> {
    routes![add_organization, get_organizations,get_organization, update_organization, delete_organization]
}