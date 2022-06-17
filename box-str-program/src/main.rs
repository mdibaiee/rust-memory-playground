fn main() {
    let boxed_str: Box<str> = "hello".into();

    println!("boxed_str: {}", boxed_str);
    println!("size of boxed_str on stack: {}", std::mem::size_of_val(&boxed_str));
}
