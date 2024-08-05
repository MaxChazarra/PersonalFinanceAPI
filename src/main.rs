use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use sea_orm::{Database, DatabaseConnection};
use dotenv::dotenv;
use std::env;

mod handlers;
mod models;
mod utils;

use handlers::{account::*, transaction::*};
use models::AppState;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    dotenv().ok();
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db: DatabaseConnection = Database::connect(database_url).await.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;

    let app_state = web::Data::new(AppState {
        accounts: Mutex::new(Vec::new()),
        database: db.clone(),
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