pub mod data_set;
pub mod tui;
use crate::tui::*;
use crate::data_set::*;
use crate::data_set::table::*;
use crate::data_set::table::column::*;
use crate::data_set::table::column::ColumnType::Int;
use crate::data_set::table::row::*;


fn main() {
    clear_terminal();
    menu();
    /*
    let mut newDataSet = create_empty_data_set();
    let mut newTable = create_empty_table();
    newTable = add_column(newTable, create_column(Int));
    newTable = add_column(newTable, create_column(Int));
    newTable = add_row(newTable, create_row());
    newTable.rows[0].cells.push("1".to_string());
    newTable.rows[0].cells.push("2".to_string());
    newTable = add_row(newTable, create_row());
    newTable.rows[1].cells.push("3".to_string());
    newTable.rows[1].cells.push("4".to_string());
    println!("minha tabela: ");
    print_table(newTable);
    */
}
