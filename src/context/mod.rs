use crate::sqlx::pool::DbConnector;

#[derive(Debug, Clone)]
pub struct Context {
    pub db_client: DbConnector,
}
