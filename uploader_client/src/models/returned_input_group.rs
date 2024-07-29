use serde_derive::{Deserialize};
use chrono::{NaiveDateTime};


#[derive(Debug, Deserialize)]
pub struct ReturnedInputGroup {
    pub input_group_id: String,
    pub program_id: String,
    pub last_reserved: Option<NaiveDateTime>,
    pub proven_datetime: Option<NaiveDateTime>,
}

pub fn print_input_groups_list(input_groups: &Vec<ReturnedInputGroup>) {
    if (input_groups.len() == 0) {
        println!("No input groups remaining");
    }
    let mut index = 0;
    for input_group in input_groups {
        println!("Program input group {}:", index);
        println!("\tinput_group_id: {}", input_group.input_group_id);
        println!("\tprogram_id: {}", input_group.program_id);
        if let Some(last_reserved) = input_group.last_reserved {
            println!("\tlast_reserved: {}", last_reserved);
        }
        if let Some(proven_datetime) = input_group.proven_datetime {
            println!("\tproven_datetime: {}", proven_datetime);
        }
        index += 1;
    }
}