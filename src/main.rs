use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

mod handlers;
mod models;
mod utils;

use handlers::{account::*, transaction::*};
use models::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        accounts: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(transaction_config)
            .configure(account_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}