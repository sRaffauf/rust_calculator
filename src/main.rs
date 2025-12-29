use std::io;

fn main() {
    let mut result = get_number_input();
    loop {
        let operation = get_string_input();

    }
}

fn get_number_input() -> f64 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read input");
    let x: f64 = match buf.trim().parse() {
        Ok(num) => num,
        Err(_) => f64::NAN
    };
    return x;
}

fn get_string_input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read input"); 
    let x: String = match buf.trim().parse() {
        Ok(str) => str,
        Err(_) => String::new()
    };
    return x;
}