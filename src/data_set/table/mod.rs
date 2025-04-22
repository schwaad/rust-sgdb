pub mod column;
pub mod row;

use column::{Column, ColumnType};
use row::Row;
use std::collections::BTreeMap;

pub struct Table {
    pub table_name : String, 
    pub columns : BTreeMap<String,ColumnType>,
    pub rows : BTreeMap<i32,BTreeMap<i32,String>>,
}

pub fn create_column( new_column_name : String, new_column_type: ColumnType) -> Column{
    Column {
        column_name : new_column_name,
        column_type : new_column_type,
    }
}

pub fn create_row(row_index : i32) -> Row{
    Row {
        index : row_index,
        cells : BTreeMap::<i32,String>::new(),
    }
}

pub fn add_column(mut current_table : Table, new_column : Column) -> Table{
    current_table.columns.insert(new_column.column_name , new_column.column_type);
    current_table
}

pub fn add_row(mut current_table : Table, new_row : Row) -> Table{
    current_table.rows.insert(new_row.index, new_row.cells);
    current_table
}

pub fn create_empty_table() -> Table{
    Table {
        table_name : String::new(),
        columns : BTreeMap::<String,ColumnType>::new(),
        rows : BTreeMap::<i32,BTreeMap<i32,String>>::new(),
    }
}

pub fn print_table(printed_table : &Table) {
    println!("{}",printed_table.table_name);
    for (_row_index,cells) in &printed_table.rows{
        for (_cell_index,content) in cells{
            print!("{} ", content);
        }
        println!("");
    }
}

