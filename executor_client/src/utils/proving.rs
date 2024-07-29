use std::{fs, path::Path, process::Command, time::SystemTime};

use crate::{common, models::returned_program::ReturnedProgram, services::program_distributor::{PagedPrograms, UploadedProof}};



pub async fn download_and_run_program(program: &ReturnedProgram) {
    let read_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.read().expect("Error in rw lock");

    let downloaded_files_names = read_guard.get_program_and_input_group(&program.program_id).await;

    let csv_file_name = downloaded_files_names.input_file_name;

    let execution_args = vec![csv_file_name.clone()];

    let start_time = SystemTime::now();

    let output = Command::new("cargo")
        .arg("run")
        .args(execution_args)
        .current_dir("./src/runner")
        .output()
        .expect("Failed to execute child program");

    println!("Program output: {:?}", output);

    let input_group_id = csv_file_name.split(".").collect::<Vec<&str>>()[0];

    let _ = fs::remove_file(format!("./program_with_input/{csv_file_name}"));
    let _ = fs::remove_file(format!("./program_with_input/{}", downloaded_files_names.program_file_name));

    let uploaded_proof_data = UploadedProof {
        organization_id: program.organization_id.clone(),
        program_id: program.program_id.clone(),
        input_group_id: input_group_id.to_string(),
    };

    read_guard.upload_proof(Path::new("./src/runner/proof.bin"), uploaded_proof_data).await.expect("Error uploading proof");

    let after_proof_time = SystemTime::now();

    println!("Proof was uploaded, total seconds passed: {}", after_proof_time.duration_since(start_time).expect("Time went backwards").as_secs());

}

pub async fn retrieve_programs(organization_option: Option<&str>, limit: Option<usize>, page: Option<usize>) -> PagedPrograms {
    let read_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.read().expect("Error in rw lock");
    if (organization_option.is_some()) {
        let organization_id = organization_option.unwrap();
        return read_guard.get_organization_programs(&organization_id.to_string(), limit, page).await;
    } else {
        return read_guard.get_general_programs(limit, page).await;
    }
}

pub async fn run_some_programs(organization_id: Option<&str>, programs_amount: usize) {
    let page_size: usize = common::config::CONFIG_OBJECT.max_page_size;
    let mut programs_page = retrieve_programs(organization_id, Some(page_size), Some(1)).await;
    let mut programs_list = programs_page.programs;
    let mut programs_counter = 0;
    while programs_list.len() != 0 && programs_counter < programs_amount {
        let mut current_page_iterator = 0;
        while programs_list.len() != 0 && programs_counter < programs_amount {
            let returned_program = &programs_list[current_page_iterator];
            download_and_run_program(&returned_program);
            current_page_iterator += 1;
            programs_counter += 1;
        }
        programs_page = retrieve_programs(organization_id, Some(page_size), Some(1)).await;
        programs_list = programs_page.programs;
    }

}