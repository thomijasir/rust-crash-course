impl TransactionHistory {
    // Create a new, empty TransactionHistory
    fn new() -> Self {
        TransactionHistory {
            transactions: Vec::new(),
        }
    }

    // Add a new transaction
    fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    // Get a transaction by index
    fn get_transaction(&self, index: usize) -> Option<&Transaction> {
        self.transactions.get(index)
    }

    // Get the total number of transactions
    fn len(&self) -> usize {
        self.transactions.len()
    }

    // Print all transactions
    fn print_all(&self) {
        for transaction in &self.transactions {
            println!("{:?}", transaction);
        }
    }
}

// USAGE
// fn main() {
//     let mut history = TransactionHistory::new();

//     // Add some transactions
//     history.add_transaction(Transaction {
//         id: 1,
//         amount: 100.0,
//         description: "Deposit".to_string(),
//         timestamp: Utc::now(),
//     });

//     history.add_transaction(Transaction {
//         id: 2,
//         amount: -50.0,
//         description: "Withdrawal".to_string(),
//         timestamp: Utc::now(),
//     });

//     // Print all transactions
//     history.print_all();

//     // Access a specific transaction
//     if let Some(transaction) = history.get_transaction(0) {
//         println!("First transaction: {:?}", transaction);
//     }

//     println!("Total transactions: {}", history.len());
// }
