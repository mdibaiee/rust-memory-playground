use std::cell::RefCell;

fn main() {
    let refcell_str: RefCell<&str> = RefCell::new("hello");

    println!("refcell_str");
    let second = refcell_str.clone();
    println!("refcell_str second:");
}
