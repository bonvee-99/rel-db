use std::io::{self, Write};

fn main() {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut buff = String::new();

    io::stdin().read_line(&mut buff).expect("Error getting command");

}
