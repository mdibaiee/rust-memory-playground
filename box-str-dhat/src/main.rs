#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    let _profiler = dhat::Profiler::new_heap();

    let boxed_str: Box<str> = "hello".into();

    println!("boxed_str: {}", boxed_str);
}
