pub mod data_set;
pub mod tui;
pub mod utils;
use crate::tui::*;
use crate::data_set::*;
use crate::data_set::table::*;
use crate::data_set::table::column::ColumnType::Int;
use std::collections::BTreeMap;
use std::env;
use clap::Parser;

#[derive(Parser)]
#[command(name = "rust-sgbd")]
#[command(about = "Um SGBD simplificado implementado em Rust", long_about = None)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    #[arg(short,long)]
    create: Option<String>,
    #[arg(short,long)]
    read: Option<String>,
    #[arg(short,long)]
    update: Option<String>,
    #[arg(short,long)]
    delete: Option<String>
}

fn main() {
    let args = Cli::parse();
    if args.verbose{
        println!("Verbose mode on");
    }
    if args.create.is_some(){
        println!("Create");
    }
    if args.read.is_some(){
        println!("Read");
    }
    if args.update.is_some(){
        println!("Update");
    }
    if args.delete.is_some(){
        println!("Delete");
    }
}   


