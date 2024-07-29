use serde_derive::{Deserialize};


#[derive(Debug, Deserialize)]
pub struct ReturnedProof {
    pub organization_id: String,
    pub program_id: String,
    pub name: String,
    pub description: String,
    pub input_lock_timeout: i64,
}

pub fn print_proofs_list(proofs: &Vec<ReturnedProof>) {
    if (proofs.len() == 0) {
        println!("No proofs remaining");
    }
    let mut index = 0;
    for proof in proofs {
        println!("Proof {}:", index);
        println!("\torganization_id: {}", proof.organization_id);
        println!("\tprogram_id: {}", proof.program_id);
        println!("\tname: {}", proof.name);
        println!("\tdescription: {}", proof.description);
        println!("\tinput_lock_timeout: {}", proof.input_lock_timeout);
        index += 1;
    }
}