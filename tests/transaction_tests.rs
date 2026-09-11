use crate::application::transaction_service::TransactionService;
use crate::domain::Transaction;

#[actix_rt::test]
async fn test_create_transaction() {
    let transaction_service = TransactionService::new(DieselRepository);
    let transaction = Transaction {
        id: 1,
        user_id: 1,
        amount: 100.0,
        date: String::from("2024-07-15"),
        status: String::from("completed"),
    };
    let result = transaction_service.create_transaction(transaction);
    assert!(result.is_ok());
}