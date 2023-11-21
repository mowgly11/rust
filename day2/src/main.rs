use std::io;

fn main() {
    println!("Insert the operaction: ");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("something went wrong");

    let result: Vec<&str> = operation.split(" ").collect();

    let x = result[0].trim().parse().expect("this is not a number");
    let y = result[2].trim().parse().expect("this is not a number");
    let operator = result[1];

    if operator == "+" {
        plus(x, y);
    } else if operator == "-" {
        minus(x, y);
    } else if operator == "/" {
        divide(x, y);
    } else if operator == "*" {
        let result = multiply(x, y);
        println!("{}", result);
    }
}

fn multiply(x: i32, y: i32) -> i32 {
    return x * y;
}

fn divide(x: i32, y: i32) {
    println!("{}", x / y);
}

fn plus(x: i32, y: i32) {
    println!("{}", x + y);
}

fn minus(x: i32, y: i32) {
    println!("{}", x - y);
}
