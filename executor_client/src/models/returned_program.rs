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
    if programs.is_empty() {
        println!("No programs are available")
    }
    for program in programs {
        println!("Program {}:", index);
        println!("\torganization_id: {}", program.organization_id);
        println!("\tprogram_id: {}", program.program_id);
        println!("\tname: {}", program.name);
        println!("\tdescription: {}", program.description);
        println!("\tinput_lock_timeout: {}", program.input_lock_timeout);
        index += 1;
    }
}