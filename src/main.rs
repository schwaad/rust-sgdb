pub mod data_set;
pub mod tui;
pub mod utils;
pub mod cli;
use crate::cli::{Cli, CommandType, ResourceType};
use clap::Parser;
fn main() {
    let cli = Cli::parse();

    match cli.command {
        CommandType::Create(r) => println!("Criando {:?}", r.resource),
        CommandType::Read(r) => println!("Lendo {:?}", r.resource),
        CommandType::Update(r) => println!("Atualizando {:?}", r.resource),
        CommandType::Delete(r) => println!("Deletando {:?}", r.resource),
    }
}

