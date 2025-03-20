pub mod column;
pub mod row;

use column::{Column, ColumnType};
use row::Row;

pub struct Table {
    columns : Vec<Column>,
    rows : Vec<Row>,
}

pub fn create_column(new_column_type: ColumnType) -> Column{
    Column {
        column_type : new_column_type
    }
}

pub fn create_row(new_row_cells: Vec<String>) -> Row{
    Row {
        cells : new_row_cells
    }
} 

pub fn test() {
    let table1 = Table {
        columns: vec![create_column(ColumnType::Str), create_column(ColumnType::Int)],
        rows: vec![create_row(vec!["oi".to_string(), "tchau".to_string()]), create_row(vec!["1".to_string(),"2".to_string()])],
    };
    for i in 0..table1.rows.len() {
        for j in 0..table1.columns.len() {
            print!("{} ", table1.rows[i].cells[j]);
        }
        println!("");
    }
}

