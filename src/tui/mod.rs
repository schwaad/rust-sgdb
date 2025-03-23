use std::io;

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn menu() {
    println!("Welcome to my simplified SGBD written in Rust!\n
Choose one of the following options:\n 
1 - Create a new database\n
2 - Access an existing database\n
0 - Quit\n");
}

