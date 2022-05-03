mod container;
mod content;
use container::prelude::*;
use content::prelude::*;

use anyhow::Result;
use dotenv::dotenv;
use sea_orm::{ConnectionTrait, Database, EntityTrait, QueryTrait, Schema};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // db connection
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(database_url).await?;

    // schema creation
    let db_backend = db.get_database_backend();
    let schema = Schema::new(db_backend);

    let create_container_table_op = db
        .execute(db_backend.build(&schema.create_table_from_entity(Container)))
        .await;
    println!(
        "`CREATE TABLE container` {:?}",
        match create_container_table_op {
            Ok(_) => "Operation Successful".to_owned(),
            Err(e) => format!("Unsuccessful - Error {:?}", e),
        }
    );

    let create_content_table_op = db
        .execute(db_backend.build(&schema.create_table_from_entity(Content)))
        .await;
    println!(
        "`CREATE TABLE content` {:?}",
        match create_content_table_op {
            Ok(_) => "Operation Successful".to_owned(),
            Err(e) => format!("Unsuccessful - Error {:?}", e),
        }
    );

    // buggy query
    let content_by_container_query = Container::find().find_with_related(Content);
    println!(
        "{}",
        &content_by_container_query.build(db_backend).to_string()
    );

    let content_by_container = content_by_container_query.all(&db).await?;
    println!("{:?}", content_by_container);

    Ok(())
}
