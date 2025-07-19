/// #PostgresDB
///
/// Will provide methods to connect to a postgres database to pull relevant property and/or
/// simulation information
///
/// properties:
/// 1. db_name
/// 2. query
/// 3. status
/// 4. connection_key
use sqlx::PgPool;
use uuid::Uuid;

enum DBStatus {
    Successful,
    Failure,
    InProgress,
}

pub struct PostgresDB {
    pub db_name: String,
    pub input_query: String,
    pub request_status: DBStatus,
    db_key: Uuid,
    db_pool: PgPool,
}
