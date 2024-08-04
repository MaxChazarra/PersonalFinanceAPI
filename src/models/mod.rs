pub mod account;
pub mod transaction;

use std::sync::Mutex;
use account::Account;

pub struct AppState {
    pub accounts: Mutex<Vec<Account>>,
}