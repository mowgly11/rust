use std::io;

fn main() {
    println!("Insert your name: ");
    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("no name specified");
    println!("{}", name);
}