use super::{config::DbConnectOptions, error::Error};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

#[derive(Debug, Clone)]
pub struct DbConnector {
    pub connection: PgPool,
}

impl DbConnector {
    pub async fn new(config: &DbConnectOptions) -> Result<Self, Error> {
        let mut pg_config = PgConnectOptions::new()
            .host(&config.host)
            .username(&config.username)
            .port(config.port)
            .application_name(&config.application_name);

        if let Some(database) = config.database.as_ref() {
            pg_config = pg_config.database(database);
        }

        if let Some(password) = config.password.as_ref() {
            pg_config = pg_config.password(password);
        }

        let db_pool = PgPoolOptions::new()
            .min_connections(config.min_connections)
            .max_connections(config.max_connections)
            .connect_with(pg_config)
            .await
            .map_err(|e| Error::Sqlx(e))?;
        Ok(Self {
            connection: db_pool,
        })
    }
}
