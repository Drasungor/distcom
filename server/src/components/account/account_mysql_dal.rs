use super::{dal::AccountDal, db_models::account::NewAccount};

// Define a struct that will implement the Printable trait
struct AccountMysqlDal {
}

// Implement the Printable trait for the Point struct
impl AccountDal for AccountMysqlDal {
    // fn print(&self) {
    //     println!("Point coordinates: ({}, {})", self.x, self.y);
    // }

    async fn register_account(new_account_data: NewAccount) {

    }

}