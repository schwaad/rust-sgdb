use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(name = "rust-sgbd")]
#[command(about = "Um SGBD em Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CommandType,
}

#[derive(Subcommand)]
pub enum CommandType {
    Create(Resource),
    Read(Resource),
    Update(Resource),
    Delete(Resource),
}

#[derive(Args, Debug)]
pub struct Resource {
    #[arg(value_enum)]
    pub resource: ResourceType,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ResourceType {
    Dataset,
    Table,
    Column,
    Row,
}

