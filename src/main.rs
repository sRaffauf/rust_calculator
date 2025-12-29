use std::io;

fn main() {
    let mut result: f64;
    let mut user_input = String::new();
    println!("Input Number");
    get_user_input(&mut user_input);
}

fn get_user_input(buffer: &mut String) {
    buffer.clear();
    match io::stdin().read_line(buffer) {
        Ok(_) => {},
        Err(err) => println!("{}", err)
    }
} 