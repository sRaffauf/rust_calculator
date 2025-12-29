use std::io;

fn main() {
    println!("Input a number");
    let mut result = get_number_input();

    loop {
        println!("Input the desired operation (type q to quit)");
        let operation = get_string_input();
        match operation.as_str() {
            "*" => result = mult(result),
            "/" => result = div(result),
            "+" => result = add(result),
            "-" => result = sub(result),
            "q" => break,
            _ => println!("Invalid operation"),
        };
        println!("{}", result)
    }
}

fn get_number_input() -> f64 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read input");
    let x: f64 = match buf.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("This is not a number");
            f64::NAN
        }
    };
    return x;
}

fn get_string_input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read input"); 
    let x: String = match buf.trim().parse() {
        Ok(str) => str,
        Err(_) => {
            println!("Invalid Input");
            String::new()
        }
    };
    return x;
}

fn add(num1: f64) -> f64 {
    println!("Input a number");
    let num2 = get_number_input();
    num1 + num2
}

fn sub(num1: f64) -> f64 {
    println!("Input a number");
    let num2 = get_number_input();
    num1 - num2
}

fn mult(num1: f64) -> f64 {
    println!("Input a number");
    let num2 = get_number_input();
    num1 * num2
}

fn div(num1: f64) -> f64 {
    println!("Input a number");
    let num2 = get_number_input();
    num1 / num2
}