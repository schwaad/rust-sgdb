pub mod data_set;
pub mod tui;
mod utils;
use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(name = "rust-sgbd")]
#[command(about = "Um SGBD em Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: CommandType,
}

#[derive(Subcommand)]
enum CommandType {
    Create(Resource),
    Read(Resource),
    Update(Resource),
    Delete(Resource),
}

#[derive(Args, Debug)]
struct Resource {
    #[arg(value_enum)]
    resource: ResourceType,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum ResourceType {
    Dataset,
    Table,
    Column,
    Row,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        CommandType::Create(r) => println!("Criando {:?}", r.resource),
        CommandType::Read(r) => println!("Lendo {:?}", r.resource),
        CommandType::Update(r) => println!("Atualizando {:?}", r.resource),
        CommandType::Delete(r) => println!("Deletando {:?}", r.resource),
    }
}

