use clap::{Parser, Subcommand};
use std::{fs, path::Path, process::Command};
use std::time::{SystemTime, Duration};

use crate::{common::{self, communication::EndpointResult}, models::{returned_organization::ReturnedOrganization, returned_program::{print_programs_list, ReturnedProgram}}, services::program_distributor::{PagedPrograms, UploadedProof}, utils::process_inputs::process_user_input};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct ProgramsArgs {
    #[command(subcommand)]
    cmd: GetProgramsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetProgramsCommands {
    Page {
        #[clap(index = 1)]
        page: usize,
    },
    Run {
        #[clap(index = 1)]
        index: usize,
    },
}


async fn download_and_run_program(program: &ReturnedProgram) {
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

async fn retrieve_programs(organization_option: Option<&ReturnedOrganization>, limit: Option<usize>, page: Option<usize>) -> EndpointResult<PagedPrograms> {
    let read_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.read().expect("Error in rw lock");
    if (organization_option.is_some()) {
        let organization = organization_option.unwrap();
        return read_guard.get_organization_programs(&organization.organization_id, limit, page).await;
    } else {
        return read_guard.get_general_programs(limit, page).await;
    }
}

async fn select_program(organization_option: Option<&ReturnedOrganization>) {
    let mut programs_page = retrieve_programs(organization_option, Some(50), Some(1)).await;
    print_programs_list(&programs_page.data.programs);

    loop {
        println!("Please execute a command:");
        let args = process_user_input();
        match ProgramsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    GetProgramsCommands::Page{page} => {
                        // get_organization_programs(organization_id: &String, limit: Option<u32>, page: Option<u32>)
                        // programs_page = get_organization_programs(&organization.organization_id, Some(50), Some(page)).await;
                        programs_page = retrieve_programs(organization_option, Some(50), Some(page)).await;
                    },
                    GetProgramsCommands::Run{index} => {
                        let chosen_program = &programs_page.data.programs[index];
                        download_and_run_program(chosen_program).await;
                    },
               }
            }
            Err(_) => {
                println!("That's not a valid command!");
            }
       };
        print_programs_list(&programs_page.data.programs);

    }    
}


pub async fn select_organization_programs(organization: &ReturnedOrganization) {
    select_program(Some(organization)).await;
}

pub async fn select_general_programs() {
    select_program(None).await;
}


