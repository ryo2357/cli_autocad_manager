use clap::{Parser, Subcommand};
mod models;
mod controllers;

#[derive(Parser)]
#[command(name = "cli-AutoCad-manager")]
#[command(about = "AutoCad manager cli app")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Aggregate
}

#[allow(unused_variables)]
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {

        Some(Commands::Aggregate) => {
            controllers::aggregate::aggregate_xlsx_parts_list()?;
        }

        None => {}
    }

    Ok(())
}
