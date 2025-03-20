mod data_set;

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

fn menu() {
   clear_terminal();
    println!("Welcome to my simplified SGBD written in Rust!\n
Choose one of the following options:\n 
1 - Create a new database\n
2 - Access an existing database\n
0 - Quit\n");
}


fn main() {
    use crate::data_set::table::test;
    menu();
    test();
}
