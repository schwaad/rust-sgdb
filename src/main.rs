pub mod data_set;
pub mod tui;
use crate::tui::*;
use crate::data_set::*;
use crate::data_set::table::*;
use crate::data_set::table::column::*;
use crate::data_set::table::column::ColumnType::Int;
use crate::data_set::table::row::*;
use std::collections::BTreeMap;


fn main() {
    //clear_terminal();
    menu();
    let mut newDataSet = create_empty_data_set();
    let mut newTable = create_empty_table();
    newDataSet.data_set_name = "meu banco".to_string();
    newTable.table_name = "minha tabela".to_string();
    newTable = add_column(newTable, create_column("Coluna 1".to_string(), Int));
    newTable = add_column(newTable, create_column("Coluna 2".to_string(), Int));
    newTable = add_row(newTable, create_row(0));
    newTable = add_row(newTable, create_row(1));

    // Garantindo que as linhas existem antes de inserir valores
    newTable.rows.entry(0).or_insert_with(BTreeMap::new).insert(0, "1".to_string());
    newTable.rows.entry(0).or_insert_with(BTreeMap::new).insert(1, "2".to_string());
    newTable.rows.entry(1).or_insert_with(BTreeMap::new).insert(0, "3".to_string());
    newTable.rows.entry(1).or_insert_with(BTreeMap::new).insert(1, "4".to_string());

    print_table(&newTable);
    newDataSet = add_table_to_data_set(newDataSet, newTable);
    save_data_set(&newDataSet);
}

