
use std::process::Command;
use clap::{CommandFactory, Parser, Subcommand};

mod config;
mod models;
mod controllers;

#[derive(Parser)]
#[command(name = "cli-drawing-manager")]
#[command(about = "assist drawing work cli app")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "process parts table in XLSX", aliases = ["t"])]
    PartsTable{
        #[command(subcommand)]
        subcommand: Option<PartsTableCommands>
    },
    #[command(about = "manage parts database", aliases = ["m"])]
    ManageDatabase{
        #[command(subcommand)]
        subcommand: Option<ManageDatabaseCommands>
    },

    Test,
}

#[derive(Subcommand)]
enum PartsTableCommands {
    #[command(about = "Collect XLSX parts list")]
    Collection,
    #[command(about = "Check duplicate error in collection csv ")]
    Check,
    #[command(about = "Aggregate from collection csv")]
    Aggregate,
}

#[derive(Subcommand)]
enum ManageDatabaseCommands {
    #[command(about = "Collect XLSX parts list")]
    Collection,
    // #[command(about = "Check duplicate error in collection csv ")]
    // Check,
    // #[command(about = "Aggregate from collection csv")]
    // Aggregate,
}
#[allow(unused_variables)]
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::PartsTable{subcommand})=>{
            match subcommand{
                Some(PartsTableCommands::Collection)=>{
                    controllers::parts_table::collect::collect_xlsx_parts_list()?;
                },
                Some(PartsTableCommands::Check)=>{
                    controllers::parts_table::check::check_collection_csv()?;
                    },
                Some(PartsTableCommands::Aggregate)=>{
                    controllers::parts_table::aggregate::aggregate_collection_csv()?;
                    },
                None=>{
                    controllers::parts_table::batch_processing::aggregate_parts_table_from_xlsx()?;
                },
            }
        }

        Some(Commands::ManageDatabase{subcommand})=>{
            match subcommand{
                Some(ManageDatabaseCommands::Collection)=>{
                    controllers::parts_table::collect::collect_xlsx_parts_list()?;
                },
                // Some(PartsTableCommands::Check)=>{
                //     controllers::parts_table::check::check_collection_csv()?;
                //     },
                // Some(PartsTableCommands::Aggregate)=>{
                //     controllers::parts_table::aggregate::aggregate_collection_csv()?;
                //     },
                None=>{
                    controllers::parts_table::batch_processing::aggregate_parts_table_from_xlsx()?;
                },
            }
        }

        Some(Commands::Test)=>{
            let config = config::Config::read();
            println!("config.database_dir: {:?}",config.database_dir());

            let path_ref = config.database_dir();
            let file_path= path_ref.join("aggregate_parts_list.csv");
            Command::new("cmd")
                .args(["/C", "start", "excel", file_path.to_str().unwrap()])
                .spawn()?;
        }

        None => {
            // 引数がない場合はヘルプを表示
            Cli::command().print_help()?;
            std::process::exit(0);
        }
    }

    Ok(())
}
