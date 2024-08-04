use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::AppState;
use crate::models::account::Account;

pub fn account_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/accounts")
            .route("", web::post().to(create_account))
            .route("", web::get().to(get_accounts))
            .route("/{id}", web::get().to(get_account))
            .route("/{id}", web::put().to(update_account))
            .route("/{id}", web::delete().to(delete_account)),
    );
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