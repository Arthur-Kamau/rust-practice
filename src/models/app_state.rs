use std::sync::Mutex;
use diesel::pg::PgConnection;
pub struct AppState {
    pub db_connection:Mutex<PgConnection>
}