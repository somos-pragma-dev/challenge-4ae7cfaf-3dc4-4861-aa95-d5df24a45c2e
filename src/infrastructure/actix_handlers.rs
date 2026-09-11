use actix_web::{web, HttpResponse, Responder};
use crate::application::transaction_service::TransactionService;
use crate::domain::Transaction;

pub async fn create_transaction(transaction_service: web::Data<TransactionService>, transaction: web::Json<Transaction>) -> impl Responder {
    match transaction_service.create_transaction(transaction.into_inner()) {
        Ok(transaction) => HttpResponse::Ok().json(transaction),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

pub async fn list_transactions(transaction_service: web::Data<TransactionService>) -> impl Responder {
    match transaction_service.list_transactions() {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

pub async fn update_transaction(transaction_service: web::Data<TransactionService>, transaction: web::Json<Transaction>) -> impl Responder {
    match transaction_service.update_transaction(transaction.into_inner()) {
        Ok(transaction) => HttpResponse::Ok().json(transaction),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

pub async fn delete_transaction(transaction_service: web::Data<TransactionService>, id: web::Path<i32>) -> impl Responder {
    match transaction_service.delete_transaction(*id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}