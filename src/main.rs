pub mod data_set;
pub mod tui;
pub mod utils;
use crate::tui::*;
use crate::data_set::*;
use crate::data_set::table::*;
use crate::data_set::table::column::ColumnType::Int;
use std::collections::BTreeMap;


fn main() {
    //clear_terminal();
    menu();
    let mut new_data_set = create_empty_data_set();
    let mut new_table = create_empty_table();
    new_data_set.data_set_name = "meu banco".to_string();
    new_table.table_name = "minha tabela".to_string();
    new_table = add_column(new_table, create_column("Coluna 1".to_string(), Int));
    new_table = add_column(new_table, create_column("Coluna 2".to_string(), Int));
    new_table = add_row(new_table, create_row(0));
    new_table = add_row(new_table, create_row(1));

    // Garantindo que as linhas existem antes de inserir valores
    new_table.rows.entry(0).or_insert_with(BTreeMap::new).insert(0, "isso".to_string());
    new_table.rows.entry(0).or_insert_with(BTreeMap::new).insert(1, "é".to_string());
    new_table.rows.entry(1).or_insert_with(BTreeMap::new).insert(0, "um".to_string());
    new_table.rows.entry(1).or_insert_with(BTreeMap::new).insert(1, "teste".to_string());

    print_table(&new_table);
    new_data_set = add_table_to_data_set(new_data_set, new_table);
    let _ = save_data_set(&new_data_set);
}

