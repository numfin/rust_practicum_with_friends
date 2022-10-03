use std::sync::Arc;

use crate::config::Config;

use self::mongo::MongoConnection;

pub mod mongo;

pub struct AppData {
    pub db_conn: Arc<MongoConnection>,
    pub config: Config,
}
