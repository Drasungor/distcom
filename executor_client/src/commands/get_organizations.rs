use clap::{crate_name, Parser, Subcommand};

use crate::{common::communication::EndpointResult, service::server_requests::ReturnedOrganization};



#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct OrganizationsArgs {
    #[command(subcommand)]
    cmd: GetOrganizationsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetOrganizationsCommands {
    Page {
        #[clap(index = 1)]
        page: u32,
    },
    // OrganizationPrograms {
    //     #[clap(short = 'l', long = "limit")]
    //     limit: Option<u32>,

    //     #[clap(short = 'p', long = "page")]
    //     page: Option<u32>,
    // },
    // AllPrograms {
    //     #[clap(short = 'l', long = "limit")]
    //     limit: Option<u32>,

    //     #[clap(short = 'p', long = "page")]
    //     page: Option<u32>,
    // },
}



async fn print_organization(organizations: &Vec<ReturnedOrganization>) {
    loop {

        println!("Please execute a command");

        let mut buf = format!("{} ", crate_name!());
        
        std::io::stdin().read_line(&mut buf).expect("Couldn't parse stdin");
        let line = buf.trim();

        println!("Line value: {}", line);

        let args = shlex::split(line).ok_or("error: Invalid quoting").unwrap();

        println!("{:?}" , args);

        match OrganizationsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    GetOrganizationsCommands::Page{page} => {


                        
                    },
                    // Commands::OrganizationPrograms{limit, page} => {
                    //     if (limit.is_some()) {
                    //         println!("Get valuea: {}", limit.unwrap());
                    //     }
                    //     if (page.is_some()) {
                    //         println!("Get valueb: {}", page.unwrap());
                    //     }
                    // },
                    // Commands::AllPrograms{limit, page} => {
                    //     if (limit.is_some()) {
                    //         println!("Get valuea: {}", limit.unwrap());
                    //     }
                    //     if (page.is_some()) {
                    //         println!("Get valueb: {}", page.unwrap());
                    //     }
                    // },
               }
            }
            Err(_) => {
                println!("That's not a valid command!");
            }
       };
    }
}

