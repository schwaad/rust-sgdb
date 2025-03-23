pub mod column;
pub mod row;

use column::{Column, ColumnType};
use row::Row;

pub struct Table {
    pub columns : Vec<Column>,
    pub rows : Vec<Row>,
}

pub fn create_column(new_column_type: ColumnType) -> Column{
    Column {
        column_type : new_column_type
    }
}

pub fn create_row() -> Row{
    Row {
        cells : Vec::<String>::new(),
    }
}

pub fn add_column(mut currentTable : Table, newColumn : Column) -> Table{
    currentTable.columns.push(newColumn);
    currentTable
}

pub fn add_row(mut currentTable : Table, newRow : Row) -> Table{
    currentTable.rows.push(newRow);
    currentTable
}

pub fn create_empty_table() -> Table{
    Table {
        columns : Vec::<Column>::new(),
        rows : Vec::<Row>::new(),
    }
}

pub fn print_table(printedTable : Table) {
    for i in 0..printedTable.rows.len(){
        for j in 0..printedTable.columns.len(){
            print!("{} ", printedTable.rows[i].cells[j]);
        }
        println!("");
    }
}

