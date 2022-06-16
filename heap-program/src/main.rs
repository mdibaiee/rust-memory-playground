use std::io::{self, Read};

fn stdin_boxed_str() -> Box<str> {
    let mut buff = [0; 10];
    let mut stdin = io::stdin(); // We get `Stdin` here.
    stdin.read(&mut buff).unwrap();
    let s = std::str::from_utf8(&buff).unwrap();

    s.into()
}

fn stdin_string() -> String {
    let mut buff = String::new();
    std::io::stdin().read_line(&mut buff).unwrap();

    buff
}

fn my_simple_program(a: i32) -> i32 {
    let b = 10;
    a + b
}


fn main() {
    println!("{}", my_simple_program(5));

    let boxed_str = stdin_boxed_str();

    println!("boxed_str: {}", boxed_str);

    let stdin_str = stdin_string();

    println!("stdin_str: {}", stdin_str);
}
