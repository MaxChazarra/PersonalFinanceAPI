use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::AppState;
use crate::models::transaction::Transaction;

pub fn transaction_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/accounts/{id}/transactions")
            .route("", web::post().to(create_transaction))
            .route("", web::get().to(get_transactions)),
    );
}

async fn create_transaction(data: web::Data<AppState>, account_id: web::Path<Uuid>, transaction: web::Json<Transaction>) -> impl Responder {
    let mut accounts = data.accounts.lock().unwrap();
    if let Some(account) = accounts.iter_mut().find(|a| a.id == *account_id) {
        let mut new_transaction = transaction.into_inner();
        new_transaction.id = Uuid::new_v4();
        account.transactions.push(new_transaction.clone());
        HttpResponse::Ok().json(new_transaction)
    } else {
        println!("Account with ID {} not found", account_id);
        HttpResponse::NotFound().finish()
    }
}

async fn get_transactions(data: web::Data<AppState>, account_id: web::Path<Uuid>) -> impl Responder {
    let accounts = data.accounts.lock().unwrap();
    if let Some(account) = accounts.iter().find(|a| a.id == *account_id) {
        HttpResponse::Ok().json(account.transactions.clone())
    } else {
        println!("Account with ID {} not found", account_id);
        HttpResponse::NotFound().finish()
    }
}