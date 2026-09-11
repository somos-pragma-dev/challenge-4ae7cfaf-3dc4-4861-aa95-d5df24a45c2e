use crate::domain::Transaction;
use crate::infrastructure::diesel_repository::DieselRepository;

pub struct TransactionService {
    repository: DieselRepository,
}

impl TransactionService {
    pub fn new(repository: DieselRepository) -> Self {
        TransactionService { repository }
    }

    pub fn create_transaction(&self, transaction: Transaction) -> Result<Transaction, String> {
        // Lógica para crear una transacción
        Ok(transaction)
    }

    pub fn list_transactions(&self) -> Result<Vec<Transaction>, String> {
        // Lógica para listar transacciones
        Ok(vec![])
    }

    pub fn update_transaction(&self, transaction: Transaction) -> Result<Transaction, String> {
        // Lógica para actualizar una transacción
        Ok(transaction)
    }

    pub fn delete_transaction(&self, id: i32) -> Result<(), String> {
        // Lógica para eliminar una transacción
        Ok(())
    }
}