use mongodb::{Client, bson::doc};

pub async fn connect_db() -> Result<Client, mongodb::error::Error> {
    let uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://root:root@localhost:27017".into());

    let client = Client::with_uri_str(&uri).await?;

    client
        .database("admin")
        .run_command(doc! { "ping": 1 })
        .await?;

    Ok(client)
}
