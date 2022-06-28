const NAME: &str = "asqar";

fn hello() -> u8 {
    let hello = NAME;
    println!("hello {}", hello);
    0
}

fn main() {
    for _ in 1..5 {
        let v = hello();
    }
}
