use std::io;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

struct T {
    name: String
}

fn parse(s: String) -> T {
    T {
        name: s.clone()
    }
}

fn mamad() -> T {
    let mut buff = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buff).unwrap();

    //let name = Box::new(s);
    //parse(&name)
    parse(buff)
}


fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let t = mamad();

    println!("name: {}", t.name);

    std::thread::sleep(std::time::Duration::from_secs(10));

    drop(t);

    println!("done");

    std::thread::sleep(std::time::Duration::from_secs(10));
}
