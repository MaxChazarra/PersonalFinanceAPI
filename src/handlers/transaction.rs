use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::AppState;
use crate::models::transaction::Transaction;

pub fn transaction_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/accounts/{account_id}/transactions")
            .route("", web::post().to(create_transaction))
            .route("", web::get().to(get_transactions))
            .route("/{transaction_id}", web::put().to(update_transaction))
            .route("/{transaction_id}", web::delete().to(delete_transaction)),
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

async fn update_transaction(data: web::Data<AppState>, account_id: web::Path<Uuid>, transaction_id: web::Path<Uuid>, updated_transaction: web::Json<Transaction>) -> impl Responder {
    let mut accounts = data.accounts.lock().unwrap();
    if let Some(account) = accounts.iter_mut().find(|a| a.id == *account_id) {
        if let Some(transaction) = account.transactions.iter_mut().find(|t| t.id == *transaction_id) {
            *transaction = updated_transaction.into_inner();
            HttpResponse::Ok().json(transaction.clone())
        } else {
            HttpResponse::NotFound().finish()
        }
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn delete_transaction(data: web::Data<AppState>, account_id: web::Path<Uuid>, transaction_id: web::Path<Uuid>) -> impl Responder {
    let mut accounts = data.accounts.lock().unwrap();
    if let Some(account) = accounts.iter_mut().find(|a| a.id == *account_id) {
        if let Some(index) = account.transactions.iter().position(|t| t.id == *transaction_id) {
            let deleted_transaction = account.transactions.remove(index);
            HttpResponse::Ok().json(deleted_transaction)
        } else {
            HttpResponse::NotFound().finish()
        }
    } else {
        HttpResponse::NotFound().finish()
    }
}