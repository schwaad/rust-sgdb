pub mod table;

use crate::table::*;
use std::fs::*;

pub struct DataSet{
    tables : Vec<Table>
}

pub fn create_empty_data_set() -> DataSet {
    DataSet {
        tables: Vec::<Table>::new(), 
    }
}

pub fn add_table_to_data_set(mut currentDataSet : DataSet, newTable : Table) -> DataSet {
    currentDataSet.tables.push(newTable);
    currentDataSet
}

pub fn print_data_set_tables(currentDataSet : DataSet ){
    for table in currentDataSet.tables {
        print_table(table);
    }
}

