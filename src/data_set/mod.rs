pub mod table;

use self::table::*;
use crate::utils::*;
use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub struct DataSet {
    pub data_set_name: String,
    tables: BTreeMap<String, Table>,
}

pub fn create_empty_data_set() -> DataSet {
    DataSet {
        data_set_name: String::new(),
        tables: BTreeMap::new(),
    }
}

pub fn add_table_to_data_set(mut current_data_set: DataSet, new_table: Table) -> DataSet {
    let table_name = &new_table.table_name;
    current_data_set.tables.insert(table_name.to_string(), new_table);
    current_data_set
}

pub fn print_data_set_tables(current_data_set: &DataSet) {
    for (_table_name, table) in &current_data_set.tables {
        print_table(table);
    }
}

pub fn check_existance_of_data_set_file(current_data_set: &DataSet) -> bool {
    let data_set_file_path = get_data_set_file_path(&current_data_set
.data_set_name);
    Path::new(&data_set_file_path).exists()
}

pub fn check_existance_of_data_set_dir() -> bool {
    let data_sets_dir_path = get_data_sets_dir_path();
    Path::new(&data_sets_dir_path).exists()
}

pub fn create_data_set_dir() {
    let data_sets_dir_path = get_data_sets_dir_path();
    match fs::create_dir_all(&data_sets_dir_path) {
        Err(why) => println!("Erro criando diretório: {:?}", why.kind()),
        Ok(_) => {},
    }
}

pub fn create_data_set_csv(current_data_set: &DataSet) {
    let data_set_file_path = get_data_set_file_path(&current_data_set
.data_set_name);
    match File::create(&data_set_file_path) {
        Err(why) => println!("Erro criando arquivo CSV: {:?}", why.kind()),
        Ok(_) => {},
    }
}

pub fn save_data_set(current_data_set: &DataSet) -> Result<(), io::Error> {
    if !check_existance_of_data_set_dir() {
        create_data_set_dir();
    }
    if !check_existance_of_data_set_file(current_data_set) {
        create_data_set_csv(current_data_set
);
    }
    
    let data_set_file_path = get_data_set_file_path(&current_data_set
.data_set_name);
    let mut data_set_csv_file = File::create(&data_set_file_path)?;
    
    for (_table_name, table) in &current_data_set.tables {
        writeln!(&mut data_set_csv_file, "{}", _table_name)?;
        for (_row_index, cells) in &table.rows {
            for (_cell_index, content) in cells {
                write!(&mut data_set_csv_file, "{},", content)?;
            }
            writeln!(&mut data_set_csv_file)?;
        }
    }
    Ok(())
}

