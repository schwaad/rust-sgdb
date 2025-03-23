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

pub fn create_row(rowIndex : i32) -> Row{
    Row {
        index : rowIndex,
        cells : BTreeMap::<i32,String>::new(),
    }
}

pub fn add_column(mut currentTable : Table, newColumn : Column) -> Table{
    currentTable.columns.insert(newColumn.column_name , newColumn.column_type);
    currentTable
}

pub fn add_row(mut currentTable : Table, newRow : Row) -> Table{
    currentTable.rows.insert(newRow.index, newRow.cells);
    currentTable
}

pub fn create_empty_table() -> Table{
    Table {
        table_name : String::new(),
        columns : BTreeMap::<String,ColumnType>::new(),
        rows : BTreeMap::<i32,BTreeMap<i32,String>>::new(),
    }
}

pub fn print_table(printedTable : &Table) {
    println!("{}",printedTable.table_name);
    for (rowIndex,cells) in &printedTable.rows{
        for (cellIndex,content) in cells{
            print!("{} ", content);
        }
        println!("");
    }
}

