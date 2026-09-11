use actix_web::test;
use crate::application::transaction_service::TransactionService;
use crate::domain::Transaction;

#[actix_rt::test]
async fn test_integration_create_transaction() {
    let transaction_service = TransactionService::new(DieselRepository);
    let transaction = Transaction {
        id: 1,
        user_id: 1,
        amount: 100.0,
        date: String::from("2024-07-15"),
        status: String::from("completed"),
    };
    let app = test::init_service(App::new()
       .app_data(web::Data::new(transaction_service))
       .service(web::resource("/transactions").route(web::post().to(crate::infrastructure::actix_handlers::create_transaction)))
    ).await;

    let req = test::TestRequest::post()
       .uri("/transactions")
       .set_json(&transaction)
       .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}