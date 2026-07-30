use lib;

fn main() {
    let num = 12;
    println!("Hello, world!{} + 12 = {}",
        num,
        lib::add(num, 12),
    );
}
