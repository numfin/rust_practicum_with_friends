use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub db: ConfigDB,
}
impl Config {
    pub fn parse() -> envy::Result<Self> {
        Ok(Self {
            db: envy::prefixed("MONGODB_").from_env::<ConfigDB>()?,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct ConfigDB {
    pub url: String,
    pub name: String,
}
