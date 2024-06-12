use std::sync::Mutex;

use sqlx::PgPool;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<i32>,
    pub db: PgPool,
}
