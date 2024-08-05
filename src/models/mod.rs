pub mod account;
pub mod transaction;

use std::sync::Mutex;
use account::Account;
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub accounts: Mutex<Vec<Account>>,
    pub database: DatabaseConnection
}