use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct Account {
    id: Uuid,
    name: String,
    transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Transaction {
    id: Uuid,
    amount: f64,
    description: String,
}

struct AppState {
    accounts: Mutex<Vec<Account>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        accounts: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/accounts", web::post().to(create_account))
            .route("/accounts", web::get().to(get_accounts))
            .route("/accounts/{id}", web::get().to(get_account))
            .route("/accounts/{id}", web::put().to(update_account))
            .route("/accounts/{id}", web::delete().to(delete_account))
            .route("/accounts/{id}/transactions", web::post().to(create_transaction))
            .route("/accounts/{id}/transactions", web::get().to(get_transactions))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn create_account(data: web::Data<AppState>, account: web::Json<Account>) -> impl Responder {
    let mut accounts = data.accounts.lock().unwrap();
    let mut new_account = account.into_inner();
    new_account.id = Uuid::new_v4();
    accounts.push(new_account.clone());
    HttpResponse::Ok().json(new_account)
}

async fn get_accounts(data: web::Data<AppState>) -> impl Responder {
    let accounts = data.accounts.lock().unwrap();
    HttpResponse::Ok().json(accounts.clone())
}

async fn get_account(data: web::Data<AppState>, account_id: web::Path<Uuid>) -> impl Responder {
    let accounts = data.accounts.lock().unwrap();
    if let Some(account) = accounts.iter().find(|a| a.id == *account_id) {
        HttpResponse::Ok().json(account.clone())
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn update_account(data: web::Data<AppState>, account_id: web::Path<Uuid>, updated_account: web::Json<Account>) -> impl Responder {
    let mut accounts = data.accounts.lock().unwrap();
    if let Some(account) = accounts.iter_mut().find(|a| a.id == *account_id) {
        account.name = updated_account.name.clone();
        HttpResponse::Ok().json(account.clone())
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn delete_account(data: web::Data<AppState>, account_id: web::Path<Uuid>) -> impl Responder {
    let mut accounts = data.accounts.lock().unwrap();
    if accounts.iter().any(|a| a.id == *account_id) {
        accounts.retain(|a| a.id != *account_id);
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn create_transaction(data: web::Data<AppState>, account_id: web::Path<Uuid>, transaction: web::Json<Transaction>) -> impl Responder {
    let mut accounts = data.accounts.lock().unwrap();
    if let Some(account) = accounts.iter_mut().find(|a| a.id == *account_id) {
        let mut new_transaction = transaction.into_inner();
        new_transaction.id = Uuid::new_v4();
        account.transactions.push(new_transaction.clone());
        HttpResponse::Ok().json(new_transaction)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn get_transactions(data: web::Data<AppState>, account_id: web::Path<Uuid>) -> impl Responder {
    let accounts = data.accounts.lock().unwrap();
    if let Some(account) = accounts.iter().find(|a| a.id == *account_id) {
        HttpResponse::Ok().json(account.transactions.clone())
    } else {
        HttpResponse::NotFound().finish()
    }
}