use std::{f64::consts::PI, io};

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
            "%" => result = modu(result),
            "^" => result = power(result),
            "!" => result = fak(result),
            "rt" => result = root(result),
            "log" => result = log(result),
            "sin" => result = sin(result),
            "cos" => result = cos(result),
            "tan" => result = tan(result),
            "q" => break,
            "=" => println!("{}", result),
	    "ac" => result = 0.0,
            _ => println!("Invalid operation"),
        };
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

fn modu(num1: f64) -> f64 {
    println!("Input a number");
    let num2 = get_number_input();
    num1 % num2
}

fn root(num1: f64) -> f64 {
    println!("Input a number");
    let num2 = get_number_input();
    num1.powf(1.0/num2)
}

fn power(num1: f64) -> f64 {
    println!("Input a number");
    let num2 = get_number_input();
    num1.powf(num2)
}

fn fak(mut num1: f64) -> f64 {
    if num1 == 0.0 {
        0.0
    } else {
        for num in 1..num1 as i64 {
            num1 = num1 * num as f64;
        }
        return num1;
    }
}

fn log(num1: f64) -> f64 {
    println!("Input a number");
    let num2 = get_number_input();
    num1.ln() / num2.ln()
}

fn sin(num1: f64) -> f64 {
    (num1 * PI / 180.0).sin()
}

fn cos(num1: f64) -> f64 {
    (num1 * PI / 180.0).cos()
}

fn tan(num1: f64) -> f64 {
    (num1 * PI / 180.0).tan()
}
