use std::sync::Arc;

use self::mongo::MongoConnection;

pub mod mongo;

pub struct AppData {
    pub db_conn: Arc<MongoConnection>,
}
