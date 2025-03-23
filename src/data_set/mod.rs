pub mod table;

use crate::table::*;
use std::path::Path;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::BufRead;

pub struct DataSet{
    pub data_set_name : String,
    tables : BTreeMap<String,Table>,
}

pub fn create_empty_data_set() -> DataSet {
    DataSet {
        data_set_name : String::new(),
        tables: BTreeMap::<String,Table>::new(), 
    }
}

pub fn add_table_to_data_set(mut currentDataSet : DataSet, newTable : Table) -> DataSet {
    let table_name = &newTable.table_name;
    currentDataSet.tables.insert(table_name.to_string(), newTable);
    currentDataSet
}

pub fn print_data_set_tables(currentDataSet : &DataSet ){
    for (tableName, table) in &currentDataSet.tables {
        print_table(table);
    }
}

pub fn check_existance_of_data_set_file(currentDataSet : &DataSet) -> bool{
    let data_set_file_path = format!("../../data_sets/{}.csv", currentDataSet.data_set_name);
    if(!Path::new(&data_set_file_path).exists()){
        false
    }
    else{
        true
    }
}

pub fn check_existance_of_data_set_dir() -> bool{
    let data_set_dir_path= format!("../../data_sets/");
    if(!Path::new(&data_set_dir_path).exists()){
        false
    }
    else{
        true
    }
}

pub fn create_data_set_dir(){
    match fs::create_dir("../../data_sets/") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }
}

pub fn create_data_set_csv(currentDataSet : &DataSet) {
    let data_set_file_path = format!("../../data_sets/{}.csv", currentDataSet.data_set_name);
    match File::create(&data_set_file_path){
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }
}

pub fn save_data_set(currentDataSet : &DataSet) -> Result<(), io::Error>{
    if(check_existance_of_data_set_dir() == false){
        create_data_set_dir();
    }
    if(check_existance_of_data_set_file(currentDataSet) == false){
        create_data_set_csv(currentDataSet);
    }
    let mut data_set_file_path = format!("../../data_sets/{}.csv", currentDataSet.data_set_name);
    let mut data_set_csv_file = File::create(&data_set_file_path)?;
    for (tableName, table) in &currentDataSet.tables{
        writeln!(&mut data_set_csv_file, "{}", tableName)?;
       for (rowIndex,cells) in &table.rows{
            for (cellIndex,content) in cells{
                write!(&mut data_set_csv_file, "{},", content)?;
            }
            writeln!(&mut data_set_csv_file, "")?;
        }
    }
    Ok(())
}

