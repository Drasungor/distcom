use serde_derive::{Deserialize};


#[derive(Debug, Deserialize)]
pub struct ReturnedProgram {
    pub organization_id: String,
    pub program_id: String,
    pub name: String,
    pub description: String,
    pub input_lock_timeout: i64,
}

pub fn print_programs_list(programs: &Vec<ReturnedProgram>) {
    let mut index = 0;
    for program in programs {
        println!("Program {}", index);


        index += 1;
    }
}