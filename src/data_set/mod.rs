pub mod table;

use crate::table::*;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;

pub struct DataSet{
    data_set_name : String,
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

pub fn print_data_set_tables(currentDataSet : DataSet ){
    for (tableName, table) in currentDataSet.tables {
        print_table(table);
    }
}
/*
pub fn check_existance_and_save_data_set(currentDataSet : DataSet ){
    let data_set_dir_exists = fs::exists("../../data_sets/");
    if(!data_set_dir_exists){
        create_data_set_dir();
        save_data_set();
    }
    else{
        save_data_set();
    }
}

pub fn create_data_set_dir(){
    match fs::create_dir("../../data_sets/") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }
}

pub fn save_data_set(currentDataSet : DataSet){}
*/
