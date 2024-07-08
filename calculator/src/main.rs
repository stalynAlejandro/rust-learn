use std::io;

fn main() {
    println!("Hello, world! - This is calculator progam");

    println!("Enter first number.");
    let f_number: u32 = get_number_from_user();

    println!("Enter second number.");
    let s_number: u32 = get_number_from_user();

    println!("Numbers: {f_number} {s_number}");
    println!("Enter Option: 1 = Add, 2 = Substract, 3 = Multiply and 4 = Divide");
    let option: u32 = get_number_from_user();
    let result = match option {
        1 => add(f_number, s_number),
        2 => substract(f_number, s_number),
        3 => multiply(f_number, s_number),
        4 => divide(f_number, s_number),
        _ => 0,
    };

    println!("This is your result {result}")
}

fn get_number_from_user() -> u32 {
    let mut number = String::from("");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    number
}

fn add(n: u32, m: u32) -> u32 {
    n + m
}

fn substract(n: u32, m: u32) -> u32 {
    // Todo: check if n > m
    if n < m {
        0
    } else {
        n - m
    }
}

fn multiply(n: u32, m: u32) -> u32 {
    n * m
}

fn divide(n: u32, m: u32) -> u32 {
    // Todo: check if m != 0
    if m <= 0 {
        0
    } else {
        n / m
    }
}
