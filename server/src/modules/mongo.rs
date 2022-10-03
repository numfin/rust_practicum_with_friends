use mongodb::{Client, Database};

pub struct MongoConnection {
    pub db: Database,
}
impl MongoConnection {
    pub async fn connect(db_url: &str, db_name: &str) -> Result<Self, mongodb::error::Error> {
        let client = Client::with_uri_str(db_url).await?;
        let db = client.database(db_name);
        println!("Available collections: ");
        for col in db.list_collection_names(None).await? {
            println!("{col}");
        }
        Ok(Self { db })
    }
    pub async fn get_data(&self) -> String {
        "data".to_string()
    }
}
