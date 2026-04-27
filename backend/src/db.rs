use rocket::http::Status;
use rocket::response::status::Custom;
use tokio_postgres::{Client, Row};
use std::collections::HashMap;

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

// For organization + tags
pub fn group_organizations(rows: Vec<Row>) -> Vec<crate::models::organization::Organization> {
    use crate::models::organization::Organization;
    use crate::models::tag::Tag;

    let mut map: HashMap<i32, Organization> = HashMap::new();

    for row in rows {
        let org_id: i32 = row.get(0);
        let org_name: String = row.get(1);

        let tag_id: Option<i32> = row.get(2);
        let tag_name: Option<String> = row.get(3);

        let org = map.entry(org_id).or_insert_with(|| Organization {
            id: Some(org_id),
            name: org_name,
            tags: Vec::new(),
        });

        if let (Some(tid), Some(tname)) = (tag_id, tag_name) {
            org.tags.push(Tag {
                id: Some(tid),
                name: tname,
            });
        }
    }

    map.into_values().collect()
}