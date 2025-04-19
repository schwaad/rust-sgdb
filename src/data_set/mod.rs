pub mod table;

use self::table::*;
use crate::utils::*;
use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::{self, Write, BufRead};
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

pub fn add_table_to_data_set(mut currentDataSet: DataSet, newTable: Table) -> DataSet {
    let table_name = &newTable.table_name;
    currentDataSet.tables.insert(table_name.to_string(), newTable);
    currentDataSet
}

pub fn print_data_set_tables(currentDataSet: &DataSet) {
    for (tableName, table) in &currentDataSet.tables {
        print_table(table);
    }
}

pub fn check_existance_of_data_set_file(currentDataSet: &DataSet) -> bool {
    let data_set_file_path = get_data_set_file_path(&currentDataSet.data_set_name);
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

pub fn create_data_set_csv(currentDataSet: &DataSet) {
    let data_set_file_path = get_data_set_file_path(&currentDataSet.data_set_name);
    match File::create(&data_set_file_path) {
        Err(why) => println!("Erro criando arquivo CSV: {:?}", why.kind()),
        Ok(_) => {},
    }
}

pub fn save_data_set(currentDataSet: &DataSet) -> Result<(), io::Error> {
    if !check_existance_of_data_set_dir() {
        create_data_set_dir();
    }
    if !check_existance_of_data_set_file(currentDataSet) {
        create_data_set_csv(currentDataSet);
    }
    
    let data_set_file_path = get_data_set_file_path(&currentDataSet.data_set_name);
    let mut data_set_csv_file = File::create(&data_set_file_path)?;
    
    for (tableName, table) in &currentDataSet.tables {
        writeln!(&mut data_set_csv_file, "{}", tableName)?;
        for (rowIndex, cells) in &table.rows {
            for (cellIndex, content) in cells {
                write!(&mut data_set_csv_file, "{},", content)?;
            }
            writeln!(&mut data_set_csv_file)?;
        }
    }
    Ok(())
}

