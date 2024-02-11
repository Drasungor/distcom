use super::dal::AccountDal;

// Define a struct that will implement the Printable trait
struct AccountMysqlDal {
}

// Implement the Printable trait for the Point struct
impl AccountDal for AccountMysqlDal {
    // fn print(&self) {
    //     println!("Point coordinates: ({}, {})", self.x, self.y);
    // }
}