pub struct MongoConnection;
impl MongoConnection {
    pub async fn connect() -> Self {
        Self
    }
    pub async fn get_data(&self) -> String {
        "data".to_string()
    }
}
