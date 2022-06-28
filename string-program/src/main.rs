fn main() {
    let s = String::from("hello");
    println!("{}", s);
    println!("size of string on stack: {}", std::mem::size_of_val(&s));
}
